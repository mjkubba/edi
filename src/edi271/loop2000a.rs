use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;
use crate::segments::aaa::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl_segments: HL,
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
    pub aaa_segments: Vec<AAA>,
    pub loop2100a: Vec<Loop2100A>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100A {
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
    pub aaa_segments: Vec<AAA>,
}

pub fn get_loop_2000a(mut contents: String) -> EdiResult<(Loop2000A, String)> {
    let mut loop2000a = Loop2000A::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        if hl_content.is_empty() {
            return Err(EdiError::MissingSegment("HL".to_string()));
        }
        loop2000a.hl_segments = get_hl(hl_content);
        
        // Verify this is an Information Source level HL segment (level code = 20)
        if loop2000a.hl_segments.hl03_hierarchical_level_code != "20" {
            return Err(EdiError::ValidationError(format!(
                "Expected HL03 code '20' for Information Source level, got '{}'",
                loop2000a.hl_segments.hl03_hierarchical_level_code
            )));
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents);
    } else {
        return Err(EdiError::MissingSegment("HL".to_string()));
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2000a.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found");
        let per_content = get_segment_contents("PER", &contents);
        if per_content.is_empty() {
            break;
        }
        let per = get_per(per_content);
        info!("PER segment parsed");
        loop2000a.per_segments.push(per);
        contents = content_trim("PER", contents);
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found");
        let ref_content = get_segment_contents("REF", &contents);
        if ref_content.is_empty() {
            break;
        }
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed");
        loop2000a.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process AAA segments (situational, can be multiple)
    while contents.starts_with("AAA") {
        info!("AAA segment found");
        let aaa_content = get_segment_contents("AAA", &contents);
        if aaa_content.is_empty() {
            break;
        }
        let aaa = get_aaa(aaa_content);
        info!("AAA segment parsed");
        loop2000a.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process Loop 2100A segments (can be multiple)
    while contents.contains("NM1") && !is_next_loop_2000b(&contents) {
        match get_loop_2100a(contents.clone()) {
            Ok((loop2100a, new_contents)) => {
                loop2000a.loop2100a.push(loop2100a);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2000A parsed");
    Ok((loop2000a, contents))
}

// Helper function to check if the next segment starts a new 2000B loop
fn is_next_loop_2000b(contents: &str) -> bool {
    if let Some(hl_content) = get_full_segment_contents("HL", contents) {
        let parts: Vec<&str> = hl_content.split('*').collect();
        if parts.len() > 3 && parts[3] == "21" {
            return true;
        }
    }
    false
}

pub fn get_loop_2100a(mut contents: String) -> EdiResult<(Loop2100A, String)> {
    let mut loop2100a = Loop2100A::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2100A");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2100a.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed for Loop 2100A");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found for Loop 2100A");
        let per_content = get_segment_contents("PER", &contents);
        if per_content.is_empty() {
            break;
        }
        let per = get_per(per_content);
        info!("PER segment parsed for Loop 2100A");
        loop2100a.per_segments.push(per);
        contents = content_trim("PER", contents);
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found for Loop 2100A");
        let ref_content = get_segment_contents("REF", &contents);
        if ref_content.is_empty() {
            break;
        }
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed for Loop 2100A");
        loop2100a.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process AAA segments (situational, can be multiple)
    while contents.starts_with("AAA") {
        info!("AAA segment found for Loop 2100A");
        let aaa_content = get_segment_contents("AAA", &contents);
        if aaa_content.is_empty() {
            break;
        }
        let aaa = get_aaa(aaa_content);
        info!("AAA segment parsed for Loop 2100A");
        loop2100a.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    info!("Loop 2100A parsed");
    Ok((loop2100a, contents))
}

pub fn write_loop_2000a(loop2000a: &Loop2000A) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000a.hl_segments.clone()));
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000a.nm1_segments.clone()));
    
    // Write all PER segments
    for per in &loop2000a.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    // Write all REF segments
    for ref_segment in &loop2000a.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2000a.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write all Loop 2100A segments
    for loop2100a in &loop2000a.loop2100a {
        contents.push_str(&write_loop_2100a(loop2100a));
    }
    
    contents
}

pub fn write_loop_2100a(loop2100a: &Loop2100A) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2100a.nm1_segments.clone()));
    
    // Write all PER segments
    for per in &loop2100a.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    // Write all REF segments
    for ref_segment in &loop2100a.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2100a.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    contents
}

// Placeholder for AAA segment functions until we implement them
fn get_aaa(_aaa_content: String) -> AAA {
    AAA::default()
}

fn write_aaa(aaa: AAA) -> String {
    format!("AAA*{}*{}*{}~", aaa.aaa01_valid_request_indicator, aaa.aaa02_agency_qualifier_code, aaa.aaa03_reject_reason_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_next_loop_2000b() {
        let contents = "HL*2*1*21*1~NM1*1P*2*ACME MEDICAL CENTER*****XX*1234567890~".to_string();
        assert!(is_next_loop_2000b(&contents));
        
        let contents = "HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~".to_string();
        assert!(!is_next_loop_2000b(&contents));
    }
}
