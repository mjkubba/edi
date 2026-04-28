use serde::{Deserialize, Serialize};

use crate::edi834::loop2330::*;
use crate::helper::edihelper::build_segment;
use crate::segments::dtp::*;
use crate::segments::r#ref::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct COB {
    pub cob01_payer_responsibility_sequence_number_code: String,
    pub cob02_reference_identification: String,
    pub cob03_coordination_of_benefits_code: String,
    pub cob04_service_type_code: String,
}

pub fn get_cob(content: String) -> COB {
    let parts: Vec<&str> = content.split('*').collect();
    let get = |i: usize| parts.get(i).unwrap_or(&"").to_string();
    COB {
        cob01_payer_responsibility_sequence_number_code: get(0),
        cob02_reference_identification: get(1),
        cob03_coordination_of_benefits_code: get(2),
        cob04_service_type_code: get(3),
    }
}

pub fn write_cob(cob: &COB) -> String {
    if cob
        .cob01_payer_responsibility_sequence_number_code
        .is_empty()
        && cob.cob03_coordination_of_benefits_code.is_empty()
    {
        return String::new();
    }
    build_segment(&[
        "COB",
        &cob.cob01_payer_responsibility_sequence_number_code,
        &cob.cob02_reference_identification,
        &cob.cob03_coordination_of_benefits_code,
        &cob.cob04_service_type_code,
    ])
}

/// Loop 2320 - Coordination of Benefits
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2320 {
    pub cob: COB,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub loop2330_segments: Vec<Loop2330>,
}

pub fn get_loop2320(contents: &str) -> (Loop2320, String) {
    let mut contents = contents.to_string();
    let mut loop2320 = Loop2320::default();

    // Parse COB segment
    if let Some(cob_start) = contents.find("COB*") {
        if let Some(cob_end) = contents[cob_start..].find("~") {
            let cob_content = &contents[cob_start + 4..cob_start + cob_end];
            loop2320.cob = get_cob(cob_content.to_string());
            contents = contents[cob_start + cob_end + 1..].to_string();
        }
    }

    // Parse REF segments
    while let Some(ref_start) = contents.find("REF*") {
        let boundary = ["DTP*", "NM1*", "COB*", "HD*", "INS*", "SE*", "LC*"]
            .iter()
            .filter_map(|s| contents.find(s))
            .min()
            .unwrap_or(contents.len());
        if ref_start > boundary {
            break;
        }
        if let Some(ref_end) = contents[ref_start..].find("~") {
            let ref_content = &contents[ref_start + 4..ref_start + ref_end];
            loop2320.ref_segments.push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse DTP segments
    while let Some(dtp_start) = contents.find("DTP*") {
        let boundary = ["NM1*", "COB*", "HD*", "INS*", "SE*", "LC*"]
            .iter()
            .filter_map(|s| contents.find(s))
            .min()
            .unwrap_or(contents.len());
        if dtp_start > boundary {
            break;
        }
        if let Some(dtp_end) = contents[dtp_start..].find("~") {
            let dtp_content = &contents[dtp_start + 4..dtp_start + dtp_end];
            loop2320.dtp_segments.push(get_dtp(dtp_content.to_string()));
            contents = contents[dtp_start + dtp_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse Loop2330 segments (COB Other Insurance Company) — up to 3
    loop {
        let trimmed = contents.trim_start_matches(|c: char| c == '\n' || c == '\r');
        if !trimmed.starts_with("NM1*") {
            break;
        }
        let (loop2330, new_contents) = get_loop2330(&contents);
        if loop2330.nm1.entity_id.is_empty() {
            contents = new_contents;
            break;
        }
        loop2320.loop2330_segments.push(loop2330);
        contents = new_contents;
    }

    (loop2320, contents)
}

pub fn write_loop2320(loop2320: Loop2320) -> String {
    let mut result = String::new();

    let cob_str = write_cob(&loop2320.cob);
    if cob_str.is_empty() {
        return result;
    }
    result.push_str(&cob_str);
    result.push('\n');

    for ref_seg in &loop2320.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }

    for dtp_seg in &loop2320.dtp_segments {
        result.push_str(&write_dtp(dtp_seg.clone()));
        result.push('\n');
    }

    for loop2330 in loop2320.loop2330_segments {
        result.push_str(&write_loop2330(loop2330));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cob() {
        let cob = get_cob("P*XYZ123*1".to_string());
        assert_eq!(cob.cob01_payer_responsibility_sequence_number_code, "P");
        assert_eq!(cob.cob02_reference_identification, "XYZ123");
        assert_eq!(cob.cob03_coordination_of_benefits_code, "1");
    }

    #[test]
    fn test_write_cob() {
        let cob = COB {
            cob01_payer_responsibility_sequence_number_code: "P".to_string(),
            cob02_reference_identification: "XYZ123".to_string(),
            cob03_coordination_of_benefits_code: "1".to_string(),
            cob04_service_type_code: "".to_string(),
        };
        assert_eq!(write_cob(&cob), "COB*P*XYZ123*1~");
    }

    #[test]
    fn test_parse_loop2320_with_loop2330() {
        let content = "COB*P*XYZ123*1~REF*60*GROUP123~DTP*344*D8*20230101~NM1*IN*2*OTHER INSURANCE CO*****PI*99999~HD*021~~";
        let (loop2320, remaining) = get_loop2320(content);
        assert_eq!(
            loop2320.cob.cob01_payer_responsibility_sequence_number_code,
            "P"
        );
        assert_eq!(loop2320.ref_segments.len(), 1);
        assert_eq!(loop2320.dtp_segments.len(), 1);
        assert_eq!(loop2320.loop2330_segments.len(), 1);
        assert_eq!(loop2320.loop2330_segments[0].nm1.entity_id, "IN");
        assert!(remaining.contains("HD*"));
    }
}
