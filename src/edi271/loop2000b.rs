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
pub struct Loop2000B {
    pub hl_segments: HL,
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
    pub aaa_segments: Vec<AAA>,
    pub loop2100b: Vec<Loop2100B>,
    pub loop2000c: Vec<Loop2000C>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100B {
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
    pub aaa_segments: Vec<AAA>,
}

pub fn get_loop_2000b(mut contents: String) -> EdiResult<(Loop2000B, String)> {
    let mut loop2000b = Loop2000B::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        if hl_content.is_empty() {
            return Err(EdiError::MissingSegment("HL".to_string()));
        }
        loop2000b.hl_segments = get_hl(hl_content);
        
        // Verify this is an Information Receiver level HL segment (level code = 21)
        if loop2000b.hl_segments.hl03_hierarchical_level_code != "21" {
            return Err(EdiError::ValidationError(format!(
                "Expected HL03 code '21' for Information Receiver level, got '{}'",
                loop2000b.hl_segments.hl03_hierarchical_level_code
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
        loop2000b.nm1_segments = get_nm1(nm1_content);
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
        loop2000b.per_segments.push(per);
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
        loop2000b.ref_segments.push(ref_segment);
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
        loop2000b.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process Loop 2100B segments (can be multiple)
    while contents.contains("NM1") && !is_next_loop_2000c(&contents) {
        match get_loop_2100b(contents.clone()) {
            Ok((loop2100b, new_contents)) => {
                loop2000b.loop2100b.push(loop2100b);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    // Process Loop 2000C segments (can be multiple)
    while contents.contains("HL") && is_next_loop_2000c(&contents) {
        match get_loop_2000c(contents.clone()) {
            Ok((loop2000c, new_contents)) => {
                loop2000b.loop2000c.push(loop2000c);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2000B parsed");
    Ok((loop2000b, contents))
}

// Helper function to check if the next segment starts a new 2000C loop
fn is_next_loop_2000c(contents: &str) -> bool {
    if let Some(hl_content) = get_full_segment_contents("HL", contents) {
        let parts: Vec<&str> = hl_content.split('*').collect();
        if parts.len() > 3 && parts[3] == "22" {
            return true;
        }
    }
    false
}

pub fn get_loop_2100b(mut contents: String) -> EdiResult<(Loop2100B, String)> {
    let mut loop2100b = Loop2100B::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2100B");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2100b.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed for Loop 2100B");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found for Loop 2100B");
        let per_content = get_segment_contents("PER", &contents);
        if per_content.is_empty() {
            break;
        }
        let per = get_per(per_content);
        info!("PER segment parsed for Loop 2100B");
        loop2100b.per_segments.push(per);
        contents = content_trim("PER", contents);
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found for Loop 2100B");
        let ref_content = get_segment_contents("REF", &contents);
        if ref_content.is_empty() {
            break;
        }
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed for Loop 2100B");
        loop2100b.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process AAA segments (situational, can be multiple)
    while contents.starts_with("AAA") {
        info!("AAA segment found for Loop 2100B");
        let aaa_content = get_segment_contents("AAA", &contents);
        if aaa_content.is_empty() {
            break;
        }
        let aaa = get_aaa(aaa_content);
        info!("AAA segment parsed for Loop 2100B");
        loop2100b.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    info!("Loop 2100B parsed");
    Ok((loop2100b, contents))
}

pub fn write_loop_2000b(loop2000b: &Loop2000B) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000b.hl_segments.clone()));
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000b.nm1_segments.clone()));
    
    // Write all PER segments
    for per in &loop2000b.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    // Write all REF segments
    for ref_segment in &loop2000b.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2000b.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write all Loop 2100B segments
    for loop2100b in &loop2000b.loop2100b {
        contents.push_str(&write_loop_2100b(loop2100b));
    }
    
    // Write all Loop 2000C segments
    for loop2000c in &loop2000b.loop2000c {
        contents.push_str(&write_loop_2000c(loop2000c));
    }
    
    contents
}

pub fn write_loop_2100b(loop2100b: &Loop2100B) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2100b.nm1_segments.clone()));
    
    // Write all PER segments
    for per in &loop2100b.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    // Write all REF segments
    for ref_segment in &loop2100b.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2100b.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    contents
}

// Import Loop2000C to avoid circular dependency
use crate::edi271::loop2000c::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_next_loop_2000c() {
        let contents = "HL*3*2*22*0~NM1*IL*1*DOE*JOHN****MI*12345678901~".to_string();
        assert!(is_next_loop_2000c(&contents));
        
        let contents = "HL*2*1*21*1~NM1*1P*2*ACME MEDICAL CENTER*****XX*1234567890~".to_string();
        assert!(!is_next_loop_2000c(&contents));
    }
}
