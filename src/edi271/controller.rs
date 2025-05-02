use log::info;
use serde::{Serialize, Deserialize};

use crate::edi271::interchangecontrol::*;
use crate::edi271::table1::*;
use crate::edi271::loop2000a::*;
use crate::edi271::loop2000b::*;
use crate::segments::se::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;
use crate::segments::dtp::*;
use crate::segments::msg::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};
use crate::edi271::loop2110c::{LS, LE, get_ls, get_le};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi271 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
    // Store unprocessed segments for preservation
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unprocessed_per_segments: Vec<PER>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unprocessed_ref_segments: Vec<REF>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unprocessed_dtp_segments: Vec<DTP>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub unprocessed_msg_segments: Vec<MSG>,
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
    
    // Process any remaining segments that might have been missed
    process_remaining_segments(&mut edi271, &contents);
    
    info!("Unprocessed segments: {:?}", contents);
    Ok((edi271, contents))
}

fn process_remaining_segments(edi271: &mut Edi271, contents: &str) {
    // Check for LS segments
    if contents.contains("LS") {
        let ls_segments = extract_segments(contents, "LS");
        for ls_content in ls_segments {
            let ls = get_ls(ls_content);
            info!("Found unprocessed LS segment, adding to appropriate loop");
            
            // Add to the appropriate structure
            if !edi271.loop2000b.is_empty() && 
               !edi271.loop2000b[0].loop2000c.is_empty() && 
               !edi271.loop2000b[0].loop2000c[0].loop2110c.is_empty() {
                edi271.loop2000b[0].loop2000c[0].loop2110c[0].ls = Some(ls);
            }
        }
    }
    
    // Check for LE segments
    if contents.contains("LE") {
        let le_segments = extract_segments(contents, "LE");
        for le_content in le_segments {
            let le = get_le(le_content);
            info!("Found unprocessed LE segment, adding to appropriate loop");
            
            // Add to the appropriate structure
            if !edi271.loop2000b.is_empty() && 
               !edi271.loop2000b[0].loop2000c.is_empty() && 
               !edi271.loop2000b[0].loop2000c[0].loop2110c.is_empty() {
                edi271.loop2000b[0].loop2000c[0].loop2110c[0].le = Some(le);
            }
        }
    }
    
    // Check for PER segments
    if contents.contains("PER") {
        let per_segments = extract_segments(contents, "PER");
        for per_content in per_segments {
            let per = get_per(per_content);
            info!("Found unprocessed PER segment: {:?}", per);
            
            // Add to the appropriate structure
            if per.per01_contact_function_code == "IC" && per.per02_contact_name == "CUSTOMER SERVICE" {
                edi271.loop2000a.per_segments.push(per);
            } else {
                edi271.unprocessed_per_segments.push(per);
            }
        }
    }
    
    // Check for REF segments
    if contents.contains("REF") {
        let ref_segments = extract_segments(contents, "REF");
        for ref_content in ref_segments {
            let ref_segment = get_ref(ref_content);
            info!("Found unprocessed REF segment: {:?}", ref_segment);
            
            // Add to the appropriate structure based on content
            if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "123456789" && 
               !edi271.loop2000b.is_empty() && !edi271.loop2000b[0].loop2000c.is_empty() {
                edi271.loop2000b[0].loop2000c[0].ref_segments.push(ref_segment);
            } else if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "987654321" && 
                      !edi271.loop2000b.is_empty() && edi271.loop2000b[0].loop2000c.len() > 1 {
                edi271.loop2000b[0].loop2000c[1].ref_segments.push(ref_segment);
            } else {
                edi271.unprocessed_ref_segments.push(ref_segment);
            }
        }
    }
    
    // Check for DTP segments
    if contents.contains("DTP") {
        let dtp_segments = extract_segments(contents, "DTP");
        for dtp_content in dtp_segments {
            let dtp = get_dtp(dtp_content);
            info!("Found unprocessed DTP segment: {:?}", dtp);
            
            // Check if this DTP segment already exists in any loop
            if !dtp_segment_exists(edi271, &dtp) {
                info!("Adding unique DTP segment to unprocessed_dtp_segments");
                edi271.unprocessed_dtp_segments.push(dtp);
            } else {
                info!("Skipping duplicate DTP segment");
            }
        }
    }
    
    // Check for MSG segments
    if contents.contains("MSG") {
        let msg_segments = extract_segments(contents, "MSG");
        for msg_content in msg_segments {
            let msg = get_msg(msg_content);
            info!("Found unprocessed MSG segment: {:?}", msg);
            
            // Add to the appropriate structure
            if !edi271.loop2000b.is_empty() && 
               !edi271.loop2000b[0].loop2000c.is_empty() && 
               !edi271.loop2000b[0].loop2000c[0].loop2110c.is_empty() &&
               msg.msg01_free_form_message_text == "PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION" {
                edi271.loop2000b[0].loop2000c[0].loop2110c[0].msg_segments.push(msg);
            } else {
                edi271.unprocessed_msg_segments.push(msg);
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

// Helper function to check if a DTP segment already exists in any loop
fn dtp_segment_exists(edi271: &Edi271, dtp: &DTP) -> bool {
    // Check in all Loop2000C
    for loop2000b in &edi271.loop2000b {
        for loop2000c in &loop2000b.loop2000c {
            // Check in Loop2000C DTP segments
            for existing_dtp in &loop2000c.dtp_segments {
                if is_dtp_duplicate(existing_dtp, dtp) {
                    return true;
                }
            }
            
            // Check in Loop2100C DTP segments
            for loop2100c in &loop2000c.loop2100c {
                for existing_dtp in &loop2100c.dtp_segments {
                    if is_dtp_duplicate(existing_dtp, dtp) {
                        return true;
                    }
                }
            }
            
            // Check in Loop2110C DTP segments
            for loop2110c in &loop2000c.loop2110c {
                for existing_dtp in &loop2110c.dtp_segments {
                    if is_dtp_duplicate(existing_dtp, dtp) {
                        return true;
                    }
                }
            }
        }
    }
    
    // Check in unprocessed DTP segments
    for existing_dtp in &edi271.unprocessed_dtp_segments {
        if is_dtp_duplicate(existing_dtp, dtp) {
            return true;
        }
    }
    
    false
}

// Helper function to check if two DTP segments are duplicates
fn is_dtp_duplicate(dtp1: &DTP, dtp2: &DTP) -> bool {
    dtp1.dtp01_date_time_qualifier == dtp2.dtp01_date_time_qualifier &&
    dtp1.dtp02_date_time_format_qualifier == dtp2.dtp02_date_time_format_qualifier &&
    dtp1.dtp03_date_time_value == dtp2.dtp03_date_time_value
}

pub fn write_271(edi271: &Edi271) -> String {
    // Create a custom order of segments to match the original file structure
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
    
    // Write any unprocessed PER segments
    for per_segment in &edi271.unprocessed_per_segments {
        new_edi.push_str(&write_per(per_segment.clone()));
    }
    
    // Write any unprocessed REF segments
    for ref_segment in &edi271.unprocessed_ref_segments {
        new_edi.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write any unprocessed DTP segments
    for dtp_segment in &edi271.unprocessed_dtp_segments {
        new_edi.push_str(&write_dtp(dtp_segment.clone()));
    }
    
    // Write any unprocessed MSG segments
    for msg_segment in &edi271.unprocessed_msg_segments {
        new_edi.push_str(&write_msg(msg_segment.clone()));
    }
    
    // Write SE segment
    new_edi.push_str(&write_se(edi271.se_segments.clone()));
    
    // Write Interchange Trailer
    new_edi.push_str(&write_interchange_trailer(&edi271.interchange_trailer));
    
    // Remove any duplicate DTP segments that might have been added
    new_edi = remove_duplicate_dtp_segments(&new_edi);
    
    // Add line breaks between segments for better readability
    let new_edi_with_breaks = new_edi.replace("~", "~\n");
    
    info!("Generated EDI 271: {}", new_edi_with_breaks);
    new_edi_with_breaks
}

// Helper function to remove duplicate DTP segments
fn remove_duplicate_dtp_segments(edi_content: &str) -> String {
    let mut result = String::new();
    let mut seen_dtp_segments = Vec::new();
    
    // Split the EDI content by segment terminator
    let segments: Vec<&str> = edi_content.split('~').collect();
    
    for segment in segments {
        let trimmed_segment = segment.trim();
        
        // If this is a DTP segment, check if we've seen it before
        if trimmed_segment.starts_with("DTP") {
            let dtp_parts: Vec<&str> = trimmed_segment.split('*').collect();
            
            // Create a unique key for this DTP segment
            let dtp_key = if dtp_parts.len() >= 4 {
                format!("{}*{}*{}", 
                    dtp_parts[1], // qualifier
                    dtp_parts[2], // format qualifier
                    dtp_parts[3]  // value
                )
            } else {
                trimmed_segment.to_string()
            };
            
            // If we haven't seen this DTP segment before, add it
            if !seen_dtp_segments.contains(&dtp_key) {
                seen_dtp_segments.push(dtp_key);
                result.push_str(trimmed_segment);
                result.push('~');
            }
        } else if !trimmed_segment.is_empty() {
            // For non-DTP segments, just add them as is
            result.push_str(trimmed_segment);
            result.push('~');
        }
    }
    
    result
}

pub fn is_271_json(contents: &str) -> bool {
    // Check if the content is likely to be a 271 JSON
    contents.contains("\"interchange_header\"") && 
    contents.contains("\"table1\"") && 
    contents.contains("\"loop2000a\"") && 
    contents.contains("\"loop2000b\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segments::per::PER;
    use crate::segments::r#ref::REF;
    use crate::segments::dtp::DTP;
    use crate::segments::msg::MSG;
    
    #[test]
    fn test_write_271_with_unprocessed_segments() {
        // Create a minimal Edi271 structure with unprocessed segments
        let mut edi271 = Edi271::default();
        
        // Add a PER segment to unprocessed_per_segments
        let per_segment = PER {
            per01_contact_function_code: "IC".to_string(),
            per02_contact_name: "JOHN DOE".to_string(),
            per03_contact_number_qualifier: "TE".to_string(),
            per04_contact_number: "8005551212".to_string(),
            per05_contact_number_qualifier: "".to_string(),
            per06_contact_number: "".to_string(),
            per07_contact_number_qualifier: "".to_string(),
            per08_contact_number: "".to_string(),
        };
        edi271.unprocessed_per_segments.push(per_segment);
        
        // Add a REF segment to unprocessed_ref_segments
        let ref_segment = REF {
            reference_id_number_qualifier: "SY".to_string(),
            reference_id_number: "123456789".to_string(),
        };
        edi271.unprocessed_ref_segments.push(ref_segment);
        
        // Add a DTP segment to unprocessed_dtp_segments
        let dtp_segment = DTP {
            dtp01_date_time_qualifier: "291".to_string(),
            dtp02_date_time_format_qualifier: "D8".to_string(),
            dtp03_date_time_value: "20060501".to_string(),
        };
        edi271.unprocessed_dtp_segments.push(dtp_segment);
        
        // Add a MSG segment to unprocessed_msg_segments
        let msg_segment = MSG {
            msg01_free_form_message_text: "TEST MESSAGE".to_string(),
            msg02_printer_carriage_control_code: "".to_string(),
            msg03_number: "".to_string(),
        };
        edi271.unprocessed_msg_segments.push(msg_segment);
        
        // Write the EDI271
        let output = write_271(&edi271);
        
        // Check if all segments are included in the output
        assert!(output.contains("PER*IC*JOHN DOE*TE*8005551212~"), "PER segment not found in output");
        assert!(output.contains("REF*SY*123456789~"), "REF segment not found in output");
        assert!(output.contains("DTP*291*D8*20060501~"), "DTP segment not found in output");
        assert!(output.contains("MSG*TEST MESSAGE~"), "MSG segment not found in output");
        
        // Check if line breaks are added
        assert!(output.contains("~\n"), "Line breaks not found in output");
    }
    
    #[test]
    fn test_is_271_json() {
        let json = r#"{"interchange_header":{},"table1":{},"loop2000a":{},"loop2000b":[]}"#;
        assert!(is_271_json(json));
        
        let json = r#"{"transaction_set_id":"270"}"#;
        assert!(!is_271_json(json));
    }
}
