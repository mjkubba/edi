use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl_segments: HL,
    pub nm1_segments: NM1,
    pub loop2000c: Vec<Loop2000C>,
}

pub fn get_loop_2000b(mut contents: String) -> (Loop2000B, String) {
    let mut loop2000b = Loop2000B::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        loop2000b.hl_segments = get_hl(hl_content);
        
        // Verify this is an Information Receiver level HL segment (level code = 21)
        if loop2000b.hl_segments.hl03_hierarchical_level_code != "21" {
            info!("Warning: Expected HL03 code '21' for Information Receiver level, got '{}'",
                loop2000b.hl_segments.hl03_hierarchical_level_code);
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents);
    } else {
        info!("Warning: Required HL segment not found in Loop 2000B");
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents);
        loop2000b.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents);
    } else {
        info!("Warning: Required NM1 segment not found in Loop 2000B");
    }
    
    // Process Loop 2000C segments (can be multiple)
    let mut loop2000c_vec = Vec::new();
    while contents.contains("HL") && is_loop_2000c(&contents) {
        let (loop2000c, new_contents) = get_loop_2000c(contents.clone());
        loop2000c_vec.push(loop2000c);
        contents = new_contents;
    }
    loop2000b.loop2000c = loop2000c_vec;
    
    info!("Loop 2000B parsed");
    (loop2000b, contents)
}

// Helper function to check if the next HL segment is for Loop 2000C
fn is_loop_2000c(contents: &str) -> bool {
    if let Some(hl_content) = get_full_segment_contents("HL", contents) {
        let parts: Vec<&str> = hl_content.split('*').collect();
        if parts.len() > 3 && parts[3] == "22" {
            return true;
        }
    }
    false
}

// Import Loop2000C to avoid circular dependency
use crate::edi270::loop2000c::*;

pub fn write_loop_2000b(loop2000b: &Loop2000B) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000b.hl_segments.clone()));
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000b.nm1_segments.clone()));
    
    // Write all Loop 2000C segments
    for loop2000c in &loop2000b.loop2000c {
        contents.push_str(&write_loop_2000c(loop2000c));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_loop_2000c() {
        let contents = "HL*2*1*22*0~NM1*IL*1*DOE*JOHN****MI*12345678901~".to_string();
        assert!(is_loop_2000c(&contents));
        
        let contents = "HL*2*1*21*0~NM1*PR*2*INSURANCE COMPANY*****PI*12345~".to_string();
        assert!(!is_loop_2000c(&contents));
    }
}
