use log::info;
use serde::{Serialize, Deserialize};

use crate::edi270::interchangecontrol::*;
use crate::edi270::table1::*;
use crate::edi270::loop2000a::*;
use crate::edi270::loop2000b::*;
use crate::segments::se::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi270 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_270(mut contents: String) -> EdiResult<(Edi270, String)> {
    let mut edi270 = Edi270::default();
    
    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();
    
    // Parse Interchange Header
    let (interchange_header, new_contents) = get_interchange_header(contents.clone());
    edi270.interchange_header = interchange_header;
    contents = new_contents;
    
    // Parse Table 1
    match get_table1(contents.clone()) {
        Ok((table1, new_contents)) => {
            edi270.table1 = table1;
            contents = new_contents;
        },
        Err(e) => return Err(e),
    }
    
    // Parse Loop 2000A (Information Source)
    match get_loop_2000a(contents.clone()) {
        Ok((loop2000a, new_contents)) => {
            edi270.loop2000a = loop2000a;
            contents = new_contents;
        },
        Err(e) => return Err(e),
    }
    
    // Parse Loop 2000B (Information Receiver) - can be multiple
    let mut loop2000b_vec = Vec::new();
    while contents.contains("HL") && contents.contains("*21*") {
        // This is a simplification - in a real implementation, you would need to check
        // if the HL segment is actually for a 2000B loop by examining the HL03 value
        let (loop2000b, new_contents) = get_loop_2000b(contents.clone());
        loop2000b_vec.push(loop2000b);
        contents = new_contents;
    }
    edi270.loop2000b = loop2000b_vec;
    
    // Parse SE segment
    if contents.contains("SE") {
        let se_content = get_segment_contents("SE", &contents);
        edi270.se_segments = get_se(se_content);
        contents = content_trim("SE", contents);
    } else {
        info!("Warning: Required SE segment not found");
    }
    
    // Parse Interchange Trailer
    let (interchange_trailer, new_contents) = get_interchange_trailer(contents.clone());
    edi270.interchange_trailer = interchange_trailer;
    contents = new_contents;
    
    info!("Unprocessed segments: {:?}", contents);
    Ok((edi270, contents))
}

pub fn write_270(edi270: &Edi270) -> String {
    let mut new_edi = String::new();
    
    // Write Interchange Header
    new_edi.push_str(&write_interchange_control(&edi270.interchange_header));
    
    // Write Table 1
    new_edi.push_str(&write_table1(&edi270.table1));
    
    // Write Loop 2000A
    new_edi.push_str(&write_loop_2000a(&edi270.loop2000a));
    
    // Write Loop 2000B (multiple)
    for loop2000b in &edi270.loop2000b {
        new_edi.push_str(&write_loop_2000b(loop2000b));
    }
    
    // Write SE segment
    new_edi.push_str(&write_se(edi270.se_segments.clone()));
    
    // Write Interchange Trailer
    new_edi.push_str(&write_interchange_trailer(&edi270.interchange_trailer));
    
    info!("Generated EDI 270: {}", new_edi);
    new_edi
}

// Function to detect if JSON contains 270 format data
pub fn is_270_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 270 format
    contents.contains("\"transaction_set_id\":\"270\"") || 
    contents.contains("\"bht01_hierarchical_structure_code\":") ||
    contents.contains("\"hl03_hierarchical_level_code\":\"20\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_270_json() {
        let json = r#"{"transaction_set_id":"270","bht01_hierarchical_structure_code":"0022"}"#;
        assert!(is_270_json(json));
        
        let json = r#"{"transaction_set_id":"271"}"#;
        assert!(!is_270_json(json));
    }
}
