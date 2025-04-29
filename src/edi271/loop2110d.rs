use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::eb::*;
use crate::segments::hsd::*;
use crate::segments::r#ref::*;
use crate::segments::dtp::*;
use crate::segments::aaa::*;
use crate::segments::msg::*;
use crate::segments::iii::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2110D {
    pub eb_segments: EB,
    pub hsd_segments: Vec<HSD>,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub aaa_segments: Vec<AAA>,
    pub msg_segments: Vec<MSG>,
    pub loop2115d: Vec<Loop2115D>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2115D {
    pub iii_segments: III,
}

pub fn get_loop_2110d(mut contents: String) -> EdiResult<(Loop2110D, String)> {
    let mut loop2110d = Loop2110D::default();
    
    // Process EB segment (required)
    if contents.contains("EB") {
        info!("EB segment found");
        let eb_content = get_segment_contents("EB", &contents);
        if eb_content.is_empty() {
            return Err(EdiError::MissingSegment("EB".to_string()));
        }
        loop2110d.eb_segments = get_eb(eb_content);
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
        loop2110d.hsd_segments.push(hsd);
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
        loop2110d.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process DTP segments (situational, can be multiple)
    while contents.starts_with("DTP") {
        info!("DTP segment found");
        let dtp_content = get_segment_contents("DTP", &contents);
        if dtp_content.is_empty() {
            break;
        }
        let dtp = get_dtp(dtp_content);
        info!("DTP segment parsed");
        loop2110d.dtp_segments.push(dtp);
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
        loop2110d.aaa_segments.push(aaa);
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
        loop2110d.msg_segments.push(msg);
        contents = content_trim("MSG", contents);
    }
    
    // Process Loop 2115D segments (can be multiple)
    while contents.contains("III") {
        match get_loop_2115d(contents.clone()) {
            Ok((loop2115d_item, new_contents)) => {
                loop2110d.loop2115d.push(loop2115d_item);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2110D parsed");
    Ok((loop2110d, contents))
}

pub fn get_loop_2115d(mut contents: String) -> EdiResult<(Loop2115D, String)> {
    let mut loop2115d = Loop2115D::default();
    
    // Process III segment (required)
    if contents.contains("III") {
        info!("III segment found");
        let iii_content = get_segment_contents("III", &contents);
        if iii_content.is_empty() {
            return Err(EdiError::MissingSegment("III".to_string()));
        }
        loop2115d.iii_segments = get_iii(iii_content);
        info!("III segment parsed");
        contents = content_trim("III", contents);
    } else {
        return Err(EdiError::MissingSegment("III".to_string()));
    }
    
    info!("Loop 2115D parsed");
    Ok((loop2115d, contents))
}

pub fn write_loop_2110d(loop2110d: &Loop2110D) -> String {
    let mut contents = String::new();
    
    // Write EB segment
    contents.push_str(&write_eb(loop2110d.eb_segments.clone()));
    
    // Write HSD segments
    for hsd in &loop2110d.hsd_segments {
        contents.push_str(&write_hsd(hsd.clone()));
    }
    
    // Write REF segments
    for ref_segment in &loop2110d.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write DTP segments
    for dtp in &loop2110d.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write AAA segments
    for aaa in &loop2110d.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write MSG segments
    for msg in &loop2110d.msg_segments {
        contents.push_str(&write_msg(msg.clone()));
    }
    
    // Write Loop 2115D segments
    for loop2115d in &loop2110d.loop2115d {
        contents.push_str(&write_loop_2115d(loop2115d));
    }
    
    contents
}

pub fn write_loop_2115d(loop2115d: &Loop2115D) -> String {
    let mut contents = String::new();
    
    // Write III segment
    contents.push_str(&write_iii(loop2115d.iii_segments.clone()));
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2115d() {
        let contents = "III*ZZ*ELIGIBILITY*Y*ADDITIONAL INFORMATION~".to_string();
        let result = get_loop_2115d(contents);
        
        assert!(result.is_ok());
        let (loop2115d, _) = result.unwrap();
        
        assert_eq!(loop2115d.iii_segments.code_list_qualifier_code, "ZZ");
        assert_eq!(loop2115d.iii_segments.industry_code, "ELIGIBILITY");
        assert_eq!(loop2115d.iii_segments.code_category, "Y");
        assert_eq!(loop2115d.iii_segments.free_form_message_text, "ADDITIONAL INFORMATION");
    }
    
    #[test]
    fn test_write_loop_2115d() {
        let loop2115d = Loop2115D {
            iii_segments: III {
                code_list_qualifier_code: "ZZ".to_string(),
                industry_code: "ELIGIBILITY".to_string(),
                code_category: "Y".to_string(),
                free_form_message_text: "ADDITIONAL INFORMATION".to_string(),
            },
        };
        
        let contents = write_loop_2115d(&loop2115d);
        assert_eq!(contents, "III*ZZ*ELIGIBILITY*Y*ADDITIONAL INFORMATION~");
    }
}
