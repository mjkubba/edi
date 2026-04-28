use crate::error::{EdiError, EdiResult};
use crate::transaction_processor::TransactionSet;
use log::info;
use serde::{Deserialize, Serialize};

use crate::edi837::interchangecontrol::InterchangeHeader;
use crate::edi837::interchangecontroltrailer::InterchangeTrailer;
use crate::edi837::loop2000a::{parse_loop2000a, write_loop2000a, Loop2000a};
use crate::edi837::loop2000b::{parse_loop2000b, write_loop2000b, Loop2000b};
use crate::edi837::loop2000c::parse_loop2000c;
use crate::edi837::loop2010aa::{parse_loop2010aa, write_loop2010aa, Loop2010aa};
use crate::edi837::loop2010ab::{parse_loop2010ab, write_loop2010ab, Loop2010ab};
use crate::edi837::loop2010ac::{parse_loop2010ac, write_loop2010ac, Loop2010ac};
use crate::edi837::loop2300::{parse_loop2300, Loop2300};
use crate::edi837::loop2400::parse_loop2400;
use crate::edi837::table1::Table1s;

/// Table1 structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1 {
    pub table1: Table1s,
    pub loop2000a: Loop2000a,
}

/// 837 subtype identifier
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Edi837Subtype {
    #[default]
    Professional,
    Institutional,
    Dental,
}

/// Unified EDI837 structure — covers Professional (P), Institutional (I), and Dental (D).
/// Claims are nested under their subscriber/patient per the HL parent-child tree.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837 {
    pub subtype: Edi837Subtype,
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2010aa: Loop2010aa,
    pub loop2010ab: Option<Loop2010ab>,
    pub loop2010ac: Option<Loop2010ac>,
    /// Subscriber levels — each contains nested patients and/or claims
    pub loop2000b: Vec<Loop2000b>,
    pub interchange_trailer: InterchangeTrailer,
    pub isa: String,
    pub gs: String,
    pub st: String,
    pub se: String,
    pub ge: String,
    pub iea: String,
}

/// Type aliases for backward compatibility
pub type Edi837P = Edi837;
pub type Edi837I = Edi837;
pub type Edi837D = Edi837;

fn detect_subtype(contents: &str) -> EdiResult<Edi837Subtype> {
    if contents.contains("005010X222") {
        Ok(Edi837Subtype::Professional)
    } else if contents.contains("005010X223") {
        Ok(Edi837Subtype::Institutional)
    } else if contents.contains("005010X224") {
        Ok(Edi837Subtype::Dental)
    } else {
        Ok(Edi837Subtype::Professional)
    }
}

/// Parse claims (Loop2300 + nested Loop2400) from remaining content
fn parse_claims(remaining_content: &mut String) -> Vec<Loop2300> {
    let mut claims = Vec::new();
    while remaining_content.contains("CLM*") {
        // Stop if the next HL comes before the next CLM
        if let Some(hl_pos) = remaining_content.find("HL*") {
            if let Some(clm_pos) = remaining_content.find("CLM*") {
                if hl_pos < clm_pos {
                    break;
                }
            }
        }
        let (loop2300, remaining) = parse_loop2300(remaining_content);
        if loop2300.clm.is_empty() {
            break;
        }
        claims.push(loop2300);
        *remaining_content = remaining;

        // Parse Loop2400 service lines for this claim
        let mut service_lines = Vec::new();
        while remaining_content.contains("LX*") {
            // Stop if next HL or CLM comes before next LX
            if let Some(lx_pos) = remaining_content.find("LX*") {
                if let Some(hl_pos) = remaining_content.find("HL*") {
                    if hl_pos < lx_pos {
                        break;
                    }
                }
                if let Some(clm_pos) = remaining_content.find("CLM*") {
                    if clm_pos < lx_pos {
                        break;
                    }
                }
            }
            let (loop2400, remaining) = parse_loop2400(remaining_content);
            if loop2400.lx.is_empty() {
                break;
            }
            service_lines.push(loop2400);
            *remaining_content = remaining;
        }
        if !service_lines.is_empty() {
            let last = claims.len() - 1;
            claims[last].loop2400 = service_lines;
        }
    }
    claims
}

fn parse_837_common(contents: String) -> EdiResult<(Edi837, String)> {
    let subtype = detect_subtype(&contents)?;
    info!("Parsing EDI837 {:?} content", subtype);

    let mut edi837 = Edi837 {
        subtype,
        ..Default::default()
    };
    let mut remaining_content = contents;

    // Parse envelope: ISA, GS, ST, BHT
    for (seg, field, name) in [
        ("ISA*", &mut edi837.isa as &mut String, "ISA"),
        ("GS*", &mut edi837.gs, "GS"),
        ("ST*", &mut edi837.st, "ST"),
    ] {
        if let Some(pos) = remaining_content.find(seg) {
            let end = remaining_content[pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + pos;
            *field = remaining_content[pos..=end].to_string();
            remaining_content = remaining_content[end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment(format!(
                "{} segment not found",
                name
            )));
        }
    }

    if let Some(bht_pos) = remaining_content.find("BHT*") {
        let bht_end = remaining_content[bht_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + bht_pos;
        edi837.table1.table1.bht = remaining_content[bht_pos..=bht_end].to_string();
        remaining_content = remaining_content[bht_end + 1..].to_string();
    } else {
        return Err(EdiError::MissingSegment(
            "BHT segment not found".to_string(),
        ));
    }

    // Parse Loop2000A (Billing Provider HL)
    let (loop2000a, remaining) = parse_loop2000a(&remaining_content);
    edi837.table1.loop2000a = loop2000a;
    remaining_content = remaining;

    // Parse Loop2010AA (Billing Provider Name)
    let (loop2010aa, remaining) = parse_loop2010aa(&remaining_content);
    edi837.loop2010aa = loop2010aa;
    remaining_content = remaining;

    // Parse Loop2010AB (Pay-to Address) if present
    if remaining_content.contains("NM1*87*") {
        let (loop2010ab, remaining) = parse_loop2010ab(&remaining_content);
        edi837.loop2010ab = Some(loop2010ab);
        remaining_content = remaining;
    }

    // Parse Loop2010AC (Pay-to Plan Name) if present
    if remaining_content.contains("NM1*PE*") {
        let (loop2010ac, remaining) = parse_loop2010ac(&remaining_content);
        edi837.loop2010ac = Some(loop2010ac);
        remaining_content = remaining;
    }

    // Walk HL segments to build the tree.
    // HL*n*parent*22* = subscriber, HL*n*parent*23* = patient
    // After each subscriber, parse its patients and claims based on HL nesting.
    while remaining_content.contains("HL*") {
        // Peek at the next HL to determine its level code
        let hl_pos = match remaining_content.find("HL*") {
            Some(p) => p,
            None => break,
        };
        let hl_end = remaining_content[hl_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + hl_pos;
        let hl_seg = &remaining_content[hl_pos..=hl_end];

        if hl_seg.contains("*22*") {
            // Subscriber level
            let (mut loop2000b, remaining) = parse_loop2000b(&remaining_content);
            if loop2000b.hl.is_empty() {
                break;
            }
            remaining_content = remaining;

            // Check if subscriber has children (HL04=1) or is also the patient (HL04=0)
            let has_children = loop2000b.hl.contains("*1~");

            if has_children {
                // Parse child Loop2000C (patient) levels
                while remaining_content.contains("HL*") {
                    let next_hl_pos = match remaining_content.find("HL*") {
                        Some(p) => p,
                        None => break,
                    };
                    let next_hl_end = remaining_content[next_hl_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + next_hl_pos;
                    let next_hl = &remaining_content[next_hl_pos..=next_hl_end];

                    if next_hl.contains("*23*") {
                        // Patient level — child of this subscriber
                        let (mut loop2000c, remaining) = parse_loop2000c(&remaining_content);
                        if loop2000c.hl.is_empty() {
                            break;
                        }
                        remaining_content = remaining;

                        // Parse claims for this patient
                        loop2000c.loop2300 = parse_claims(&mut remaining_content);
                        loop2000b.loop2000c.push(loop2000c);
                    } else {
                        // Next HL is a new subscriber — stop parsing children
                        break;
                    }
                }
            } else {
                // Subscriber IS the patient — claims attach directly
                loop2000b.loop2300 = parse_claims(&mut remaining_content);
            }

            edi837.loop2000b.push(loop2000b);
        } else {
            // Unknown HL level or orphan — skip past it to avoid infinite loop
            remaining_content = remaining_content[hl_end + 1..].to_string();
        }
    }

    // Parse trailer segments
    for (seg, field) in [
        ("SE*", &mut edi837.se as &mut String),
        ("GE*", &mut edi837.ge),
        ("IEA*", &mut edi837.iea),
    ] {
        if let Some(pos) = remaining_content.find(seg) {
            let end = remaining_content[pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + pos;
            *field = remaining_content[pos..=end].to_string();
            remaining_content = remaining_content[end + 1..].to_string();
        }
    }

    Ok((edi837, remaining_content))
}

fn write_837_common(edi837: &Edi837) -> EdiResult<String> {
    info!("Generating EDI837 {:?} content", edi837.subtype);

    let mut result = String::new();

    // Envelope
    result.push_str(&edi837.isa);
    result.push('\n');
    result.push_str(&edi837.gs);
    result.push('\n');
    result.push_str(&edi837.st);
    result.push('\n');
    result.push_str(&edi837.table1.table1.bht);
    result.push('\n');

    // Loop2000A + 2010AA/AB/AC
    result.push_str(&write_loop2000a(&edi837.table1.loop2000a));
    result.push_str(&write_loop2010aa(&edi837.loop2010aa));
    if let Some(loop2010ab) = &edi837.loop2010ab {
        result.push_str(&write_loop2010ab(loop2010ab));
    }
    if let Some(loop2010ac) = &edi837.loop2010ac {
        result.push_str(&write_loop2010ac(loop2010ac));
    }

    // Loop2000B (subscribers) — write_loop2000b now handles nested 2000C and claims
    for loop2000b in &edi837.loop2000b {
        result.push_str(&write_loop2000b(loop2000b));
    }

    // Trailer
    result.push_str(&edi837.se);
    result.push('\n');
    result.push_str(&edi837.ge);
    result.push('\n');
    result.push_str(&edi837.iea);
    result.push('\n');

    Ok(result)
}

impl TransactionSet for Edi837 {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        parse_837_common(contents)
    }

    fn to_edi(&self) -> String {
        write_837_common(self).unwrap_or_default()
    }

    fn get_transaction_type() -> &'static str {
        "837"
    }

    fn detect(contents: &str) -> bool {
        contents.contains("ST*837*") && contents.contains("BHT*0019*00*")
    }
}

pub fn get_837(content: &str) -> EdiResult<Edi837> {
    match parse_837_common(content.to_string()) {
        Ok((edi837, _)) => Ok(edi837),
        Err(e) => Err(e),
    }
}

pub fn write_837(edi837: &Edi837) -> EdiResult<String> {
    write_837_common(edi837)
}

// Backward-compatible wrappers
pub fn get_837p(content: &str) -> EdiResult<Edi837> {
    get_837(content)
}
pub fn get_837i(content: &str) -> EdiResult<Edi837> {
    get_837(content)
}
pub fn get_837d(content: &str) -> EdiResult<Edi837> {
    get_837(content)
}
pub fn write_837p(edi837: &Edi837) -> EdiResult<String> {
    write_837(edi837)
}
pub fn write_837i(edi837: &Edi837) -> EdiResult<String> {
    write_837(edi837)
}
pub fn write_837d(edi837: &Edi837) -> EdiResult<String> {
    write_837(edi837)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_837P: &str = "ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*050208*1112*^*00501*000017712*0*T*:~GS*HC*1234567890*9876543210*20050208*1112*17712*X*005010X222A1~ST*837*000017712*005010X222A1~BHT*0019*00*000017712*20050208*1112*CH~HL*1**20*1~NM1*85*2*ACME MEDICAL GROUP****XX*1234567890~N3*100 MAIN STREET~N4*ANYTOWN*AL*35242~REF*EI*123456789~HL*2*1*22*0~SBR*P*18*******MC~NM1*IL*1*DOE*JOHN****MI*123456789A~CLM*051068*766.50***11:B:1*Y*A*Y*Y*P~LX*1~SV1*HC:A0427:RH*700*UN*1~DTP*472*D8*20050208~SE*15*000017712~GE*1*17712~IEA*1*000017712~";

    const SAMPLE_837I: &str = "ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*960918*0932*^*00501*000000001*0*T*:~GS*HC*1234567890*9876543210*19960918*0932*1*X*005010X223A2~ST*837*987654*005010X223A2~BHT*0019*00*0123*19960918*0932*CH~HL*1**20*1~NM1*85*2*GENERAL HOSPITAL****XX*1234567890~N3*100 MAIN STREET~N4*ANYTOWN*AL*35242~REF*EI*123456789~HL*2*1*22*0~SBR*P*18*******MC~NM1*IL*1*DOE*JOHN****MI*123456789A~CLM*756048Q*89.93***11:B:1*Y*A*Y*Y*P~LX*1~SV2*0305*HC:85025*13.39*UN*1~DTP*472*D8*19960918~SE*15*987654~GE*1*1~IEA*1*000000001~";

    const SAMPLE_837D: &str = "ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*050705*1112*^*00501*000000002*0*T*:~GS*HC*1234567890*9876543210*20050705*1112*2*X*005010X224A2~ST*837*000002*005010X224A2~BHT*0019*00*000002*20050705*1112*CH~HL*1**20*1~NM1*85*2*DENTAL CLINIC****XX*1234567890~N3*100 MAIN STREET~N4*ANYTOWN*AL*35242~REF*EI*123456789~HL*2*1*22*0~SBR*P*18*******CI~NM1*IL*1*DOE*JANE****MI*JA7654321~CLM*26407789*115***11:B:1*Y*A*Y*Y~LX*1~SV3*AD:D0120*100~DTP*472*D8*20050705~SE*15*000002~GE*1*2~IEA*1*000000002~";

    // Multi-subscriber batch with patient hierarchy
    const SAMPLE_MULTI_SUB: &str = "ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*050208*1112*^*00501*000017712*0*T*:~GS*HC*1234567890*9876543210*20050208*1112*17712*X*005010X222A1~ST*837*000017712*005010X222A1~BHT*0019*00*000017712*20050208*1112*CH~HL*1**20*1~NM1*85*2*ACME MEDICAL GROUP****XX*1234567890~N3*100 MAIN STREET~N4*ANYTOWN*AL*35242~REF*EI*123456789~HL*2*1*22*1~SBR*P*18*******CI~NM1*IL*1*DOE*JOHN****MI*111111111~NM1*PR*2*ACME INS*****PI*999996666~HL*3*2*23*0~PAT*19~NM1*QC*1*DOE*JANE****MI*222222222~N3*234 SOUTH ST~N4*ANYWHERE*TN*37214~CLM*CLAIM001*100***11:B:1*Y*A*Y*Y*P~LX*1~SV1*HC:99213*100*UN*1~DTP*472*D8*20050208~HL*4*1*22*0~SBR*S*18*******MC~NM1*IL*1*SMITH*BOB****MI*333333333~NM1*PR*2*MEDICARE*****PI*00435~CLM*CLAIM002*200***11:B:1*Y*A*Y*Y*P~LX*1~SV1*HC:99214*200*UN*1~DTP*472*D8*20050209~SE*30*000017712~GE*1*17712~IEA*1*000017712~";

    #[test]
    fn test_parse_837p_subscriber_is_patient() {
        let edi837 = get_837(SAMPLE_837P).unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Professional);
        assert_eq!(edi837.loop2000b.len(), 1);
        // Subscriber IS patient (HL04=0) — claims nested under loop2000b
        assert_eq!(edi837.loop2000b[0].loop2300.len(), 1);
        assert!(edi837.loop2000b[0].loop2300[0]
            .clm
            .contains("CLM*051068*766.50"));
        assert_eq!(edi837.loop2000b[0].loop2300[0].loop2400.len(), 1);
        assert!(edi837.loop2000b[0].loop2300[0].loop2400[0]
            .sv1
            .as_ref()
            .unwrap()
            .contains("SV1*HC:A0427:RH*700*UN*1"));
        // No separate patient level
        assert!(edi837.loop2000b[0].loop2000c.is_empty());
    }

    #[test]
    fn test_write_837p() {
        let edi837 = get_837(SAMPLE_837P).unwrap();
        let generated = write_837(&edi837).unwrap();
        assert!(generated.contains("BHT*0019*00*000017712"));
        assert!(generated.contains("CLM*051068*766.50"));
        assert!(generated.contains("SV1*HC:A0427:RH*700*UN*1"));
    }

    #[test]
    fn test_parse_837i() {
        let edi837 = get_837(SAMPLE_837I).unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Institutional);
        assert_eq!(edi837.loop2000b.len(), 1);
        assert_eq!(edi837.loop2000b[0].loop2300.len(), 1);
        assert!(edi837.loop2000b[0].loop2300[0]
            .clm
            .contains("CLM*756048Q*89.93"));
        assert!(edi837.loop2000b[0].loop2300[0].loop2400[0]
            .sv2
            .as_ref()
            .unwrap()
            .contains("SV2*0305*HC:85025*13.39*UN*1"));
    }

    #[test]
    fn test_parse_837d() {
        let edi837 = get_837(SAMPLE_837D).unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Dental);
        assert_eq!(edi837.loop2000b.len(), 1);
        assert_eq!(edi837.loop2000b[0].loop2300.len(), 1);
        assert!(edi837.loop2000b[0].loop2300[0].loop2400[0]
            .sv3
            .as_ref()
            .unwrap()
            .contains("SV3*AD:D0120*100"));
    }

    #[test]
    fn test_multi_subscriber_with_patient() {
        let edi837 = get_837(SAMPLE_MULTI_SUB).unwrap();

        // Two subscribers
        assert_eq!(edi837.loop2000b.len(), 2);

        // Subscriber 1 (HL04=1) has a patient child
        let sub1 = &edi837.loop2000b[0];
        assert!(sub1.hl.contains("*22*1~"));
        assert!(sub1.loop2300.is_empty()); // no direct claims
        assert_eq!(sub1.loop2000c.len(), 1); // one patient
        let patient = &sub1.loop2000c[0];
        assert!(patient.hl.contains("*23*"));
        assert_eq!(
            patient.nm1_patient.as_deref(),
            Some("NM1*QC*1*DOE*JANE****MI*222222222~")
        );
        assert_eq!(patient.loop2300.len(), 1);
        assert!(patient.loop2300[0].clm.contains("CLM*CLAIM001*100"));

        // Subscriber 2 (HL04=0) is also the patient
        let sub2 = &edi837.loop2000b[1];
        assert!(sub2.hl.contains("*22*0~"));
        assert!(sub2.loop2000c.is_empty()); // no patient children
        assert_eq!(sub2.loop2300.len(), 1); // claim directly on subscriber
        assert!(sub2.loop2300[0].clm.contains("CLM*CLAIM002*200"));
    }

    #[test]
    fn test_multi_subscriber_roundtrip() {
        let edi837 = get_837(SAMPLE_MULTI_SUB).unwrap();
        let generated = write_837(&edi837).unwrap();

        // Verify all key segments present in correct order
        assert!(generated.contains("HL*2*1*22*1~"));
        assert!(generated.contains("NM1*IL*1*DOE*JOHN"));
        assert!(generated.contains("HL*3*2*23*0~"));
        assert!(generated.contains("NM1*QC*1*DOE*JANE"));
        assert!(generated.contains("CLM*CLAIM001*100"));
        assert!(generated.contains("HL*4*1*22*0~"));
        assert!(generated.contains("NM1*IL*1*SMITH*BOB"));
        assert!(generated.contains("CLM*CLAIM002*200"));

        // Verify ordering: sub1 before patient before sub2
        let sub1_pos = generated.find("HL*2*1*22*1~").unwrap();
        let patient_pos = generated.find("HL*3*2*23*0~").unwrap();
        let sub2_pos = generated.find("HL*4*1*22*0~").unwrap();
        assert!(sub1_pos < patient_pos);
        assert!(patient_pos < sub2_pos);
    }

    #[test]
    fn test_subtype_detection() {
        assert_eq!(
            detect_subtype(SAMPLE_837P).unwrap(),
            Edi837Subtype::Professional
        );
        assert_eq!(
            detect_subtype(SAMPLE_837I).unwrap(),
            Edi837Subtype::Institutional
        );
        assert_eq!(detect_subtype(SAMPLE_837D).unwrap(), Edi837Subtype::Dental);
    }

    #[test]
    fn test_backward_compat_wrappers() {
        assert!(get_837p(SAMPLE_837P).is_ok());
        assert!(get_837i(SAMPLE_837I).is_ok());
        assert!(get_837d(SAMPLE_837D).is_ok());
        assert!(write_837p(&get_837p(SAMPLE_837P).unwrap()).is_ok());
        assert!(write_837i(&get_837i(SAMPLE_837I).unwrap()).is_ok());
        assert!(write_837d(&get_837d(SAMPLE_837D).unwrap()).is_ok());
    }
}
