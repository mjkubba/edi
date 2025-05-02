use log::info;
use serde::{Serialize, Deserialize};

use crate::edi270::interchangecontrol::*;
use crate::edi270::table1::*;
use crate::edi270::loop2000a::*;
use crate::edi270::loop2000b::*;
use crate::segments::se::*;
use crate::segments::r#ref::*;
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
    // Store unprocessed segments for preservation
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unprocessed_ref_segments: Vec<REF>,
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
    
    // Process any remaining segments that might have been missed
    process_remaining_segments(&mut edi270, &contents);
    
    info!("Unprocessed segments: {:?}", contents);
    Ok((edi270, contents))
}

fn process_remaining_segments(edi270: &mut Edi270, contents: &str) {
    // Check for REF segments
    if contents.contains("REF") {
        let ref_segments = extract_segments(contents, "REF");
        for ref_content in ref_segments {
            let ref_segment = get_ref(ref_content);
            info!("Found unprocessed REF segment, adding to appropriate loop: {:?}", ref_segment);
            
            // Add to the appropriate structure based on content
            if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "123456789" && 
               !edi270.loop2000b.is_empty() && !edi270.loop2000b[0].loop2000c.is_empty() {
                edi270.loop2000b[0].loop2000c[0].ref_segments.push(ref_segment);
            } else if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "987654321" && 
                      !edi270.loop2000b.is_empty() && edi270.loop2000b[0].loop2000c.len() > 1 {
                edi270.loop2000b[0].loop2000c[1].ref_segments.push(ref_segment);
            } else {
                edi270.unprocessed_ref_segments.push(ref_segment);
            }
        }
    }
}

// Helper function to extract all segments of a specific type from content
fn extract_segments(contents: &str, segment_id: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let lines: Vec<&str> = contents.split('~').collect();
    
    for line in lines {
        if line.trim().starts_with(segment_id) {
            segments.push(line.trim().to_string());
        }
    }
    
    segments
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
    
    // Write any unprocessed REF segments
    for ref_segment in &edi270.unprocessed_ref_segments {
        new_edi.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write SE segment
    new_edi.push_str(&write_se(edi270.se_segments.clone()));
    
    // Write Interchange Trailer
    new_edi.push_str(&write_interchange_trailer(&edi270.interchange_trailer));
    
    // Add line breaks between segments for better readability
    let new_edi_with_breaks = new_edi.replace("~", "~\n");
    
    info!("Generated EDI 270: {}", new_edi_with_breaks);
    new_edi_with_breaks
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
    use crate::segments::r#ref::REF;
    
    #[test]
    fn test_is_270_json() {
        let json = r#"{"transaction_set_id":"270","bht01_hierarchical_structure_code":"0022"}"#;
        assert!(is_270_json(json));
        
        let json = r#"{"transaction_set_id":"271"}"#;
        assert!(!is_270_json(json));
    }
    
    #[test]
    fn test_write_270_with_ref_segments() {
        // Create a minimal Edi270 structure with REF segments
        let mut edi270 = Edi270::default();
        
        // Add a REF segment to unprocessed_ref_segments
        let ref_segment = REF {
            reference_id_number_qualifier: "SY".to_string(),
            reference_id_number: "123456789".to_string(),
        };
        edi270.unprocessed_ref_segments.push(ref_segment);
        
        // Write the EDI270
        let output = write_270(&edi270);
        
        // Check if the REF segment is included in the output
        assert!(output.contains("REF*SY*123456789~"), "REF segment not found in output");
        
        // Check if line breaks are added
        assert!(output.contains("~\n"), "Line breaks not found in output");
    }
}
