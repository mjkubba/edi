use log::info;
use serde::{Serialize, Deserialize};

use crate::edi271::interchangecontrol::*;
use crate::edi271::table1::*;
use crate::edi271::loop2000a::*;
use crate::edi271::loop2000b::*;
use crate::segments::se::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi271 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_271(mut contents: String) -> EdiResult<(Edi271, String)> {
    let mut edi271 = Edi271::default();
    
    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();
    
    // Parse Interchange Header
    let (interchange_header, new_contents) = get_interchange_header(contents.clone());
    edi271.interchange_header = interchange_header;
    contents = new_contents;
    
    // Parse Table 1
    match get_table1(contents.clone()) {
        Ok((table1, new_contents)) => {
            edi271.table1 = table1;
            contents = new_contents;
        },
        Err(e) => return Err(e),
    }
    
    // Parse Loop 2000A (Information Source)
    match get_loop_2000a(contents.clone()) {
        Ok((loop2000a, new_contents)) => {
            edi271.loop2000a = loop2000a;
            contents = new_contents;
        },
        Err(e) => return Err(e),
    }
    
    // Parse Loop 2000B (Information Receiver) - can be multiple
    let mut loop2000b_vec = Vec::new();
    while contents.contains("HL") && contents.contains("*21*") {
        match get_loop_2000b(contents.clone()) {
            Ok((loop2000b, new_contents)) => {
                loop2000b_vec.push(loop2000b);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    edi271.loop2000b = loop2000b_vec;
    
    // Parse SE segment
    if contents.contains("SE") {
        info!("SE segment found");
        let se_content = get_segment_contents("SE", &contents);
        edi271.se_segments = get_se(se_content);
        info!("SE segment parsed");
        contents = content_trim("SE", contents);
    } else {
        info!("Warning: Required SE segment not found");
    }
    
    // Parse Interchange Trailer
    let (interchange_trailer, new_contents) = get_interchange_trailer(contents.clone());
    edi271.interchange_trailer = interchange_trailer;
    contents = new_contents;
    
    info!("Unprocessed segments: {:?}", contents);
    Ok((edi271, contents))
}

pub fn write_271(edi271: &Edi271) -> String {
    let mut new_edi = String::new();
    
    // Write Interchange Header
    new_edi.push_str(&write_interchange_control(&edi271.interchange_header));
    
    // Write Table 1
    new_edi.push_str(&write_table1(&edi271.table1));
    
    // Write Loop 2000A
    new_edi.push_str(&write_loop_2000a(&edi271.loop2000a));
    
    // Write Loop 2000B (multiple)
    for loop2000b in &edi271.loop2000b {
        new_edi.push_str(&write_loop_2000b(loop2000b));
    }
    
    // Write SE segment
    new_edi.push_str(&write_se(edi271.se_segments.clone()));
    
    // Write Interchange Trailer
    new_edi.push_str(&write_interchange_trailer(&edi271.interchange_trailer));
    
    info!("Generated EDI 271: {}", new_edi);
    new_edi
}

// Function to detect if JSON contains 271 format data
pub fn is_271_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 271 format
    contents.contains("\"transaction_set_id\":\"271\"") || 
    contents.contains("\"eb01_eligibility_indicator\":") ||
    (contents.contains("\"bht01_hierarchical_structure_code\":") && 
     contents.contains("\"bht02_transaction_set_purpose_code\":\"11\""))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_271_json() {
        let json = r#"{"transaction_set_id":"271","bht01_hierarchical_structure_code":"0022"}"#;
        assert!(is_271_json(json));
        
        let json = r#"{"transaction_set_id":"270"}"#;
        assert!(!is_271_json(json));
    }
}
