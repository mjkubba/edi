use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::trn::*;
use crate::segments::r#ref::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::dmg::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl_segments: HL,
    pub trn_segments: Option<TRN>,
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub dmg_segments: Option<DMG>,
    pub loop2000d: Vec<Loop2000D>,
}

pub fn get_loop_2000c(mut contents: String) -> (Loop2000C, String) {
    let mut loop2000c = Loop2000C::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        loop2000c.hl_segments = get_hl(hl_content);
        
        // Verify this is a Subscriber level HL segment (level code = 22)
        if loop2000c.hl_segments.hl03_hierarchical_level_code != "22" {
            info!("Warning: Expected HL03 code '22' for Subscriber level, got '{}'",
                loop2000c.hl_segments.hl03_hierarchical_level_code);
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents);
    } else {
        info!("Warning: Required HL segment not found in Loop 2000C");
    }
    
    // Process TRN segment (situational)
    if contents.contains("TRN") {
        info!("TRN segment found");
        let trn_content = get_segment_contents("TRN", &contents);
        loop2000c.trn_segments = Some(get_trn(trn_content));
        info!("TRN segment parsed");
        contents = content_trim("TRN", contents);
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents);
        loop2000c.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents);
    } else {
        info!("Warning: Required NM1 segment not found in Loop 2000C");
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found");
        let ref_content = get_segment_contents("REF", &contents);
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed");
        loop2000c.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found");
        let n3_content = get_segment_contents("N3", &contents);
        loop2000c.n3_segments = Some(get_n3(n3_content));
        info!("N3 segment parsed");
        contents = content_trim("N3", contents);
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found");
        let n4_content = get_segment_contents("N4", &contents);
        loop2000c.n4_segments = Some(get_n4(n4_content));
        info!("N4 segment parsed");
        contents = content_trim("N4", contents);
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found");
        let dmg_content = get_segment_contents("DMG", &contents);
        loop2000c.dmg_segments = Some(get_dmg(dmg_content));
        info!("DMG segment parsed");
        contents = content_trim("DMG", contents);
    }
    
    // Process Loop 2000D segments (can be multiple)
    let mut loop2000d_vec = Vec::new();
    while contents.contains("HL") && is_loop_2000d(&contents) {
        let (loop2000d, new_contents) = get_loop_2000d(contents.clone());
        loop2000d_vec.push(loop2000d);
        contents = new_contents;
    }
    loop2000c.loop2000d = loop2000d_vec;
    
    info!("Loop 2000C parsed");
    (loop2000c, contents)
}

// Helper function to check if the next HL segment is for Loop 2000D
fn is_loop_2000d(contents: &str) -> bool {
    if let Some(hl_content) = get_full_segment_contents("HL", contents) {
        let parts: Vec<&str> = hl_content.split('*').collect();
        if parts.len() > 3 && parts[3] == "23" {
            return true;
        }
    }
    false
}

// Import Loop2000D to avoid circular dependency
use crate::edi270::loop2000d::*;

pub fn write_loop_2000c(loop2000c: &Loop2000C) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000c.hl_segments.clone()));
    
    // Write TRN segment if present
    if let Some(trn) = &loop2000c.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000c.nm1_segments.clone()));
    
    // Write all REF segments
    for ref_segment in &loop2000c.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000c.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000c.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000c.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    // Write all Loop 2000D segments
    for loop2000d in &loop2000c.loop2000d {
        contents.push_str(&write_loop_2000d(loop2000d));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_loop_2000d() {
        let contents = "HL*3*2*23*0~NM1*IL*1*DOE*JANE****MI*98765432101~".to_string();
        assert!(is_loop_2000d(&contents));
        
        let contents = "HL*2*1*22*0~NM1*IL*1*DOE*JOHN****MI*12345678901~".to_string();
        assert!(!is_loop_2000d(&contents));
    }
}
