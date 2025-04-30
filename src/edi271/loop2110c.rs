use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::eb::*;
use crate::segments::hsd::*;
use crate::segments::r#ref::*;
use crate::segments::dtp::*;
use crate::segments::aaa::*;
use crate::segments::msg::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};
use crate::edi271::loop2115c::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2110C {
    pub eb_segments: EB,
    pub hsd_segments: Vec<HSD>,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub aaa_segments: Vec<AAA>,
    pub msg_segments: Vec<MSG>,
    pub loop2115c: Vec<Loop2115C>,
    pub ls: Option<LS>,
    pub le: Option<LE>,
}

pub fn get_loop_2110c(mut contents: String) -> EdiResult<(Loop2110C, String)> {
    let mut loop2110c = Loop2110C::default();
    
    // Process EB segment (required)
    if contents.contains("EB") {
        info!("EB segment found");
        let eb_content = get_segment_contents("EB", &contents);
        if eb_content.is_empty() {
            return Err(EdiError::MissingSegment("EB".to_string()));
        }
        loop2110c.eb_segments = get_eb(eb_content);
        info!("EB segment parsed");
        contents = content_trim("EB", contents);
    } else {
        return Err(EdiError::MissingSegment("EB".to_string()));
    }
    
    // Process HSD segments (situational, can be multiple)
    while contents.starts_with("HSD") {
        info!("HSD segment found");
        let hsd_content = get_segment_contents("HSD", &contents);
        if hsd_content.is_empty() {
            break;
        }
        let hsd = get_hsd(hsd_content);
        info!("HSD segment parsed");
        loop2110c.hsd_segments.push(hsd);
        contents = content_trim("HSD", contents);
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
        loop2110c.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process DTP segments (situational, can be multiple)
    // Only process DTP segments that are specific to this loop
    // This helps prevent duplicate DTP segments in the output
    while contents.starts_with("DTP") {
        info!("DTP segment found");
        let dtp_content = get_segment_contents("DTP", &contents);
        if dtp_content.is_empty() {
            break;
        }
        
        // Parse the DTP segment
        let dtp = get_dtp(dtp_content);
        
        // Only add DTP segments with specific qualifiers to this loop
        // This prevents duplicates from being added in process_remaining_segments
        if dtp.dtp01_date_time_qualifier == "291" || 
           dtp.dtp01_date_time_qualifier == "348" {
            info!("DTP segment parsed and added to Loop2110C");
            loop2110c.dtp_segments.push(dtp);
        } else {
            info!("DTP segment parsed but not added to Loop2110C (will be handled elsewhere)");
        }
        
        contents = content_trim("DTP", contents);
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
        loop2110c.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process MSG segments (situational, can be multiple)
    while contents.starts_with("MSG") {
        info!("MSG segment found");
        let msg_content = get_segment_contents("MSG", &contents);
        if msg_content.is_empty() {
            break;
        }
        let msg = get_msg(msg_content);
        info!("MSG segment parsed");
        loop2110c.msg_segments.push(msg);
        contents = content_trim("MSG", contents);
    }
    
    // Process LS segment and its content (situational)
    if contents.starts_with("LS") {
        info!("LS segment found");
        let ls_content = get_full_segment_contents("LS", &contents).unwrap_or_default();
        
        // Extract the loop identifier code from the LS segment
        let ls_parts: Vec<&str> = ls_content.split('*').collect();
        let loop_id = if ls_parts.len() > 1 {
            ls_parts[1].trim_end_matches('~').to_string()
        } else {
            "2120".to_string() // Default value if not found
        };
        
        // Create the LS segment
        let ls = LS {
            loop_identifier_code: loop_id.clone(),
        };
        loop2110c.ls = Some(ls);
        
        // Remove the LS segment from contents
        contents = content_trim("LS", contents);
        
        // Find the corresponding LE segment
        let le_position = contents.find("LE*");
        
        if let Some(le_pos) = le_position {
            // Extract content between LS and LE
            let ls_le_content = &contents[..le_pos];
            
            // Process NM1*P3 segments within the LS/LE loop
            let mut remaining_content = ls_le_content.to_string();
            while remaining_content.contains("NM1*P3") {
                // Find the NM1*P3 segment
                if let Some(nm1_pos) = remaining_content.find("NM1*P3") {
                    // Extract from NM1 to the next segment or end
                    let nm1_content = &remaining_content[nm1_pos..];
                    let end_pos = nm1_content.find('~').unwrap_or(nm1_content.len());
                    let nm1_segment = &nm1_content[..end_pos+1];
                    
                    // Process this segment as Loop2115C
                    match get_loop_2115c(nm1_segment.to_string()) {
                        Ok((loop2115c, _)) => {
                            loop2110c.loop2115c.push(loop2115c);
                        },
                        Err(e) => {
                            info!("Error parsing Loop 2115C: {:?}", e);
                        }
                    }
                    
                    // Remove the processed segment from remaining content
                    remaining_content = remaining_content[nm1_pos + end_pos + 1..].to_string();
                } else {
                    break;
                }
            }
            
            // Process the LE segment
            let le_content = get_full_segment_contents("LE", &contents[le_pos..]).unwrap_or_default();
            
            // Extract the loop identifier code from the LE segment (should match LS)
            let le_parts: Vec<&str> = le_content.split('*').collect();
            let le_loop_id = if le_parts.len() > 1 {
                le_parts[1].trim_end_matches('~').to_string()
            } else {
                loop_id.clone() // Use the same as LS if not found
            };
            
            // Create the LE segment
            let le = LE {
                loop_identifier_code: le_loop_id,
            };
            loop2110c.le = Some(le);
            
            // Remove everything up to and including the LE segment
            if let Some(le_end) = contents[le_pos..].find('~') {
                contents = contents[le_pos + le_end + 1..].to_string();
            }
        }
    }
    
    info!("Loop 2110C parsed");
    Ok((loop2110c, contents))
}

pub fn write_loop_2110c(loop2110c: &Loop2110C) -> String {
    let mut contents = String::new();
    
    // Write EB segment
    contents.push_str(&write_eb(loop2110c.eb_segments.clone()));
    
    // Write HSD segments
    for hsd in &loop2110c.hsd_segments {
        contents.push_str(&write_hsd(hsd.clone()));
    }
    
    // Write REF segments
    for ref_segment in &loop2110c.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write DTP segments
    for dtp in &loop2110c.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write AAA segments
    for aaa in &loop2110c.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write MSG segments
    for msg in &loop2110c.msg_segments {
        contents.push_str(&write_msg(msg.clone()));
    }
    
    // Write LS and LE segments with NM1 in between - in original file, LS/LE wrap around NM1
    if let Some(ls) = &loop2110c.ls {
        contents.push_str(&write_ls(ls.clone()));
        
        // Write Loop 2115C segments - these should include the NM1 segments
        for loop2115c in &loop2110c.loop2115c {
            contents.push_str(&write_loop_2115c(loop2115c));
        }
        
        if let Some(le) = &loop2110c.le {
            contents.push_str(&write_le(le.clone()));
        }
    } else {
        // If no LS/LE, just write the Loop 2115C segments normally
        for loop2115c in &loop2110c.loop2115c {
            contents.push_str(&write_loop_2115c(loop2115c));
        }
    }
    
    contents
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LS {
    pub loop_identifier_code: String,
}

pub fn get_ls(ls_content: String) -> LS {
    let ls_parts: Vec<&str> = ls_content.split("*").collect();
    
    // Safely access elements with bounds checking
    let get_element = |index: usize| -> String {
        if index < ls_parts.len() {
            // Remove any trailing ~ character
            ls_parts[index].trim_end_matches('~').to_string()
        } else {
            String::new()
        }
    };
    
    LS {
        loop_identifier_code: get_element(1),
    }
}

pub fn write_ls(ls: LS) -> String {
    // Ensure we have a valid loop identifier code
    let code = if ls.loop_identifier_code.is_empty() {
        "2120".to_string() // Default to 2120 if empty
    } else {
        ls.loop_identifier_code.clone()
    };
    
    format!("LS*{}~", code)
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LE {
    pub loop_identifier_code: String,
}

pub fn get_le(le_content: String) -> LE {
    let le_parts: Vec<&str> = le_content.split("*").collect();
    
    // Safely access elements with bounds checking
    let get_element = |index: usize| -> String {
        if index < le_parts.len() {
            // Remove any trailing ~ character
            le_parts[index].trim_end_matches('~').to_string()
        } else {
            String::new()
        }
    };
    
    LE {
        loop_identifier_code: get_element(1),
    }
}

pub fn write_le(le: LE) -> String {
    // Ensure we have a valid loop identifier code
    // The LE code should match the LS code
    let code = if le.loop_identifier_code.is_empty() {
        "2120".to_string() // Default to 2120 if empty
    } else {
        le.loop_identifier_code.clone()
    };
    
    format!("LE*{}~", code)
}
