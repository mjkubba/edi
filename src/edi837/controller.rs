use crate::error::{EdiError, EdiResult};
use crate::transaction_processor::TransactionSet;
use log::info;
use serde::{Deserialize, Serialize};

use crate::edi837::interchangecontrol::InterchangeHeader;
use crate::edi837::interchangecontroltrailer::InterchangeTrailer;
use crate::edi837::loop2000a::{parse_loop2000a, write_loop2000a, Loop2000a};
use crate::edi837::loop2000b::{parse_loop2000b, write_loop2000b, Loop2000b};
use crate::edi837::loop2000c::{parse_loop2000c, write_loop2000c, Loop2000c};
use crate::edi837::loop2010aa::{parse_loop2010aa, write_loop2010aa, Loop2010aa};
use crate::edi837::loop2010ab::{parse_loop2010ab, write_loop2010ab, Loop2010ab};
use crate::edi837::loop2010ac::{parse_loop2010ac, write_loop2010ac, Loop2010ac};
use crate::edi837::loop2300::{parse_loop2300, write_loop2300, Loop2300};
use crate::edi837::loop2400::{parse_loop2400, write_loop2400, Loop2400};
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
/// Per X12 spec, all three subtypes share identical loop structure above Loop 2400.
/// Differences (SV1 vs SV2 vs SV3+TOO, CL1, DTP qualifiers) are handled by the
/// shared Loop2300/Loop2400 structs which carry all segment variants as Options.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837 {
    pub subtype: Edi837Subtype,
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000b: Vec<Loop2000b>,
    pub loop2000c: Vec<Loop2000c>,
    pub loop2010aa: Loop2010aa,
    pub loop2010ab: Option<Loop2010ab>,
    pub loop2010ac: Option<Loop2010ac>,
    pub loop2300: Vec<Loop2300>,
    pub loop2400: Vec<Loop2400>,
    pub interchange_trailer: InterchangeTrailer,
    // Raw segments for segments we don't parse yet
    pub isa: String,
    pub gs: String,
    pub st: String,
    pub se: String,
    pub ge: String,
    pub iea: String,
}

/// Type aliases for backward compatibility with existing JSON files
pub type Edi837P = Edi837;
pub type Edi837I = Edi837;
pub type Edi837D = Edi837;

/// Detect 837 subtype from content
fn detect_subtype(contents: &str) -> EdiResult<Edi837Subtype> {
    if contents.contains("005010X222") {
        Ok(Edi837Subtype::Professional)
    } else if contents.contains("005010X223") {
        Ok(Edi837Subtype::Institutional)
    } else if contents.contains("005010X224") {
        Ok(Edi837Subtype::Dental)
    } else {
        // Default to Professional if we can't determine
        Ok(Edi837Subtype::Professional)
    }
}

/// Parse any 837 variant from EDI content
fn parse_837_common(contents: String) -> EdiResult<(Edi837, String)> {
    let subtype = detect_subtype(&contents)?;
    info!("Parsing EDI837 {:?} content", subtype);

    let mut edi837 = Edi837 {
        subtype,
        ..Default::default()
    };
    let mut remaining_content = contents;

    // Parse ISA segment
    if let Some(isa_pos) = remaining_content.find("ISA*") {
        let isa_end = remaining_content[isa_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + isa_pos;
        edi837.isa = remaining_content[isa_pos..=isa_end].to_string();
        remaining_content = remaining_content[isa_end + 1..].to_string();
    } else {
        return Err(EdiError::MissingSegment(
            "ISA segment not found".to_string(),
        ));
    }

    // Parse GS segment
    if let Some(gs_pos) = remaining_content.find("GS*") {
        let gs_end = remaining_content[gs_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + gs_pos;
        edi837.gs = remaining_content[gs_pos..=gs_end].to_string();
        remaining_content = remaining_content[gs_end + 1..].to_string();
    } else {
        return Err(EdiError::MissingSegment("GS segment not found".to_string()));
    }

    // Parse ST segment
    if let Some(st_pos) = remaining_content.find("ST*") {
        let st_end = remaining_content[st_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + st_pos;
        edi837.st = remaining_content[st_pos..=st_end].to_string();
        remaining_content = remaining_content[st_end + 1..].to_string();
    } else {
        return Err(EdiError::MissingSegment("ST segment not found".to_string()));
    }

    // Parse BHT segment
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

    // Parse Loop2000A (Billing Provider Hierarchical Level)
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

    // Parse Loop2000B (Subscriber Hierarchical Level)
    let mut loop2000b_vec = Vec::new();
    while remaining_content.contains("HL*") && remaining_content.contains("*22*") {
        let (loop2000b, remaining) = parse_loop2000b(&remaining_content);
        if loop2000b.hl.is_empty() {
            break;
        }
        loop2000b_vec.push(loop2000b);
        remaining_content = remaining;
    }
    edi837.loop2000b = loop2000b_vec;

    // Parse Loop2000C (Patient Hierarchical Level)
    let mut loop2000c_vec = Vec::new();
    while remaining_content.contains("HL*") && remaining_content.contains("*23*") {
        let (loop2000c, remaining) = parse_loop2000c(&remaining_content);
        if loop2000c.hl.is_empty() {
            break;
        }
        loop2000c_vec.push(loop2000c);
        remaining_content = remaining;
    }
    edi837.loop2000c = loop2000c_vec;

    // Parse Loop2300 (Claim Information)
    let mut loop2300_vec = Vec::new();
    while remaining_content.contains("CLM*") {
        let (loop2300, remaining) = parse_loop2300(&remaining_content);
        if loop2300.clm.is_empty() {
            break;
        }
        loop2300_vec.push(loop2300);
        remaining_content = remaining;

        // Parse Loop2400 (Service Line Information) for this claim
        let mut loop2400_vec = Vec::new();
        while remaining_content.contains("LX*") {
            let (loop2400, remaining) = parse_loop2400(&remaining_content);
            if loop2400.lx.is_empty() {
                break;
            }
            loop2400_vec.push(loop2400);
            remaining_content = remaining;
        }

        // Add service lines to the claim
        if !loop2400_vec.is_empty() {
            let last_index = loop2300_vec.len() - 1;
            loop2300_vec[last_index].loop2400 = loop2400_vec;
        }
    }
    edi837.loop2300 = loop2300_vec;

    // Parse interchange trailer
    if let Some(se_pos) = remaining_content.find("SE*") {
        let se_end = remaining_content[se_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + se_pos;
        edi837.se = remaining_content[se_pos..=se_end].to_string();
        remaining_content = remaining_content[se_end + 1..].to_string();
    }

    if let Some(ge_pos) = remaining_content.find("GE*") {
        let ge_end = remaining_content[ge_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + ge_pos;
        edi837.ge = remaining_content[ge_pos..=ge_end].to_string();
        remaining_content = remaining_content[ge_end + 1..].to_string();
    }

    if let Some(iea_pos) = remaining_content.find("IEA*") {
        let iea_end = remaining_content[iea_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + iea_pos;
        edi837.iea = remaining_content[iea_pos..=iea_end].to_string();
        remaining_content = remaining_content[iea_end + 1..].to_string();
    }

    Ok((edi837, remaining_content))
}

/// Generate EDI from an Edi837 struct
fn write_837_common(edi837: &Edi837) -> EdiResult<String> {
    info!("Generating EDI837 {:?} content", edi837.subtype);

    let mut result = String::new();

    // Write envelope segments (already contain ~ terminator)
    result.push_str(&edi837.isa);
    result.push('\n');
    result.push_str(&edi837.gs);
    result.push('\n');
    result.push_str(&edi837.st);
    result.push('\n');
    result.push_str(&edi837.table1.table1.bht);
    result.push('\n');

    // Write Loop2000A (Billing Provider Hierarchical Level)
    result.push_str(&write_loop2000a(&edi837.table1.loop2000a));

    // Write Loop2010AA (Billing Provider Name)
    result.push_str(&write_loop2010aa(&edi837.loop2010aa));

    // Write Loop2010AB (Pay-to Address) if present
    if let Some(loop2010ab) = &edi837.loop2010ab {
        result.push_str(&write_loop2010ab(loop2010ab));
    }

    // Write Loop2010AC (Pay-to Plan Name) if present
    if let Some(loop2010ac) = &edi837.loop2010ac {
        result.push_str(&write_loop2010ac(loop2010ac));
    }

    // Write Loop2000B (Subscriber Hierarchical Level)
    for loop2000b in &edi837.loop2000b {
        result.push_str(&write_loop2000b(loop2000b));
    }

    // Write Loop2000C (Patient Hierarchical Level)
    for loop2000c in &edi837.loop2000c {
        result.push_str(&write_loop2000c(loop2000c));
    }

    // Write Loop2300 (Claim Information)
    for loop2300 in &edi837.loop2300 {
        result.push_str(&write_loop2300(loop2300));

        // Write Loop2400 (Service Line) for each claim
        for loop2400 in &loop2300.loop2400 {
            result.push_str(&write_loop2400(loop2400));
        }
    }

    // Write trailer segments (already contain ~ terminator)
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

/// Parse EDI837 content (any subtype — auto-detected from version identifier)
pub fn get_837(content: &str) -> EdiResult<Edi837> {
    match parse_837_common(content.to_string()) {
        Ok((edi837, _)) => Ok(edi837),
        Err(e) => Err(e),
    }
}

/// Generate EDI837 content
pub fn write_837(edi837: &Edi837) -> EdiResult<String> {
    write_837_common(edi837)
}

// Backward-compatible wrapper functions
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

    #[test]
    fn test_parse_837p() {
        let result = get_837(SAMPLE_837P);
        assert!(result.is_ok(), "Failed to parse 837P: {:?}", result.err());

        let edi837 = result.unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Professional);
        assert!(!edi837.isa.is_empty());
        assert!(!edi837.table1.table1.bht.is_empty());
        assert!(!edi837.loop2000b.is_empty());
        assert!(!edi837.loop2300.is_empty());
        assert!(edi837.loop2300[0].clm.contains("CLM*051068*766.50"));
        assert!(!edi837.loop2300[0].loop2400.is_empty());
        assert!(edi837.loop2300[0].loop2400[0]
            .sv1
            .as_ref()
            .unwrap()
            .contains("SV1*HC:A0427:RH*700*UN*1"));
    }

    #[test]
    fn test_write_837p() {
        let result = get_837(SAMPLE_837P);
        assert!(result.is_ok());
        let edi837 = result.unwrap();
        let write_result = write_837(&edi837);
        assert!(write_result.is_ok());
        let generated = write_result.unwrap();
        assert!(generated.contains("BHT*0019*00*000017712"));
        assert!(generated.contains("CLM*051068*766.50"));
        assert!(generated.contains("SV1*HC:A0427:RH*700*UN*1"));
    }

    #[test]
    fn test_parse_837i() {
        let result = get_837(SAMPLE_837I);
        assert!(result.is_ok(), "Failed to parse 837I: {:?}", result.err());

        let edi837 = result.unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Institutional);
        assert!(!edi837.isa.is_empty());
        assert!(!edi837.table1.table1.bht.is_empty());
        assert!(!edi837.loop2000b.is_empty());
        assert!(!edi837.loop2300.is_empty());
        assert!(edi837.loop2300[0].clm.contains("CLM*756048Q*89.93"));
        assert!(!edi837.loop2300[0].loop2400.is_empty());
        assert!(edi837.loop2300[0].loop2400[0]
            .sv2
            .as_ref()
            .unwrap()
            .contains("SV2*0305*HC:85025*13.39*UN*1"));
    }

    #[test]
    fn test_write_837i() {
        let result = get_837(SAMPLE_837I);
        assert!(result.is_ok());
        let edi837 = result.unwrap();
        let write_result = write_837(&edi837);
        assert!(write_result.is_ok());
        let generated = write_result.unwrap();
        assert!(generated.contains("BHT*0019*00*0123"));
        assert!(generated.contains("CLM*756048Q*89.93"));
    }

    #[test]
    fn test_parse_837d() {
        let result = get_837(SAMPLE_837D);
        assert!(result.is_ok(), "Failed to parse 837D: {:?}", result.err());

        let edi837 = result.unwrap();
        assert_eq!(edi837.subtype, Edi837Subtype::Dental);
        assert!(!edi837.isa.is_empty());
        assert!(!edi837.loop2300.is_empty());
        assert!(edi837.loop2300[0].clm.contains("CLM*26407789*115"));
        assert!(!edi837.loop2300[0].loop2400.is_empty());
        assert!(edi837.loop2300[0].loop2400[0]
            .sv3
            .as_ref()
            .unwrap()
            .contains("SV3*AD:D0120*100"));
    }

    #[test]
    fn test_write_837d() {
        let result = get_837(SAMPLE_837D);
        assert!(result.is_ok());
        let edi837 = result.unwrap();
        let write_result = write_837(&edi837);
        assert!(write_result.is_ok());
        let generated = write_result.unwrap();
        assert!(generated.contains("BHT*0019*00*000002"));
        assert!(generated.contains("CLM*26407789*115"));
        assert!(generated.contains("SV3*AD:D0120*100"));
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
        // Ensure old function names still work
        let p = get_837p(SAMPLE_837P);
        assert!(p.is_ok());
        let i = get_837i(SAMPLE_837I);
        assert!(i.is_ok());
        let d = get_837d(SAMPLE_837D);
        assert!(d.is_ok());

        assert!(write_837p(&p.unwrap()).is_ok());
        assert!(write_837i(&i.unwrap()).is_ok());
        assert!(write_837d(&d.unwrap()).is_ok());
    }
}
