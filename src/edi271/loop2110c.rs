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

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2110C {
    pub eb_segments: EB,
    pub hsd_segments: Vec<HSD>,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub aaa_segments: Vec<AAA>,
    pub msg_segments: Vec<MSG>,
    pub loop2115c: Vec<Loop2115C>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2115C {
    pub iii_segments: III,
    pub ls_segments: Option<LS>,
    pub loop2120c: Vec<Loop2120C>,
    pub le_segments: Option<LE>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2120C {
    pub nm1_segments: NM1,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub per_segments: Vec<PER>,
    pub prv_segments: Option<PRV>,
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
    while contents.starts_with("DTP") {
        info!("DTP segment found");
        let dtp_content = get_segment_contents("DTP", &contents);
        if dtp_content.is_empty() {
            break;
        }
        let dtp = get_dtp(dtp_content);
        info!("DTP segment parsed");
        loop2110c.dtp_segments.push(dtp);
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
    
    // Process Loop 2115C segments (can be multiple)
    while contents.contains("III") {
        match get_loop_2115c(contents.clone()) {
            Ok((loop2115c, new_contents)) => {
                loop2110c.loop2115c.push(loop2115c);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2110C parsed");
    Ok((loop2110c, contents))
}

pub fn get_loop_2115c(mut contents: String) -> EdiResult<(Loop2115C, String)> {
    let mut loop2115c = Loop2115C::default();
    
    // Process III segment (required)
    if contents.contains("III") {
        info!("III segment found");
        let iii_content = get_segment_contents("III", &contents);
        if iii_content.is_empty() {
            return Err(EdiError::MissingSegment("III".to_string()));
        }
        loop2115c.iii_segments = get_iii(iii_content);
        info!("III segment parsed");
        contents = content_trim("III", contents);
    } else {
        return Err(EdiError::MissingSegment("III".to_string()));
    }
    
    // Process LS segment (situational)
    if contents.contains("LS") {
        info!("LS segment found");
        let ls_content = get_segment_contents("LS", &contents);
        if !ls_content.is_empty() {
            loop2115c.ls_segments = Some(get_ls(ls_content));
            info!("LS segment parsed");
            contents = content_trim("LS", contents);
            
            // Process Loop 2120C segments (can be multiple)
            while contents.contains("NM1") && !contents.contains("LE") {
                match get_loop_2120c(contents.clone()) {
                    Ok((loop2120c, new_contents)) => {
                        loop2115c.loop2120c.push(loop2120c);
                        contents = new_contents;
                    },
                    Err(_) => break,
                }
            }
            
            // Process LE segment (required if LS is present)
            if contents.contains("LE") {
                info!("LE segment found");
                let le_content = get_segment_contents("LE", &contents);
                if !le_content.is_empty() {
                    loop2115c.le_segments = Some(get_le(le_content));
                    info!("LE segment parsed");
                    contents = content_trim("LE", contents);
                }
            }
        }
    }
    
    info!("Loop 2115C parsed");
    Ok((loop2115c, contents))
}

pub fn get_loop_2120c(mut contents: String) -> EdiResult<(Loop2120C, String)> {
    let mut loop2120c = Loop2120C::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2120C");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2120c.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed for Loop 2120C");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found for Loop 2120C");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2120c.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed for Loop 2120C");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found for Loop 2120C");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2120c.n4_segments = Some(get_n4(n4_content));
            info!("N4 segment parsed for Loop 2120C");
            contents = content_trim("N4", contents);
        }
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found for Loop 2120C");
        let per_content = get_segment_contents("PER", &contents);
        if per_content.is_empty() {
            break;
        }
        let per = get_per(per_content);
        info!("PER segment parsed for Loop 2120C");
        loop2120c.per_segments.push(per);
        contents = content_trim("PER", contents);
    }
    
    // Process PRV segment (situational)
    if contents.contains("PRV") {
        info!("PRV segment found for Loop 2120C");
        let prv_content = get_segment_contents("PRV", &contents);
        if !prv_content.is_empty() {
            loop2120c.prv_segments = Some(get_prv(prv_content));
            info!("PRV segment parsed for Loop 2120C");
            contents = content_trim("PRV", contents);
        }
    }
    
    info!("Loop 2120C parsed");
    Ok((loop2120c, contents))
}

pub fn write_loop_2110c(loop2110c: &Loop2110C) -> String {
    let mut contents = String::new();
    
    // Write EB segment
    contents.push_str(&write_eb(loop2110c.eb_segments.clone()));
    
    // Write all HSD segments
    for hsd in &loop2110c.hsd_segments {
        contents.push_str(&write_hsd(hsd.clone()));
    }
    
    // Write all REF segments
    for ref_segment in &loop2110c.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all DTP segments
    for dtp in &loop2110c.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2110c.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write all MSG segments
    for msg in &loop2110c.msg_segments {
        contents.push_str(&write_msg(msg.clone()));
    }
    
    // Write all Loop 2115C segments
    for loop2115c in &loop2110c.loop2115c {
        contents.push_str(&write_loop_2115c(loop2115c));
    }
    
    contents
}

pub fn write_loop_2115c(loop2115c: &Loop2115C) -> String {
    let mut contents = String::new();
    
    // Write III segment
    contents.push_str(&write_iii(loop2115c.iii_segments.clone()));
    
    // Write LS segment if present
    if let Some(ls) = &loop2115c.ls_segments {
        contents.push_str(&write_ls(ls.clone()));
        
        // Write all Loop 2120C segments
        for loop2120c in &loop2115c.loop2120c {
            contents.push_str(&write_loop_2120c(loop2120c));
        }
        
        // Write LE segment if present
        if let Some(le) = &loop2115c.le_segments {
            contents.push_str(&write_le(le.clone()));
        }
    }
    
    contents
}

pub fn write_loop_2120c(loop2120c: &Loop2120C) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2120c.nm1_segments.clone()));
    
    // Write N3 segment if present
    if let Some(n3) = &loop2120c.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2120c.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write all PER segments
    for per in &loop2120c.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    // Write PRV segment if present
    if let Some(prv) = &loop2120c.prv_segments {
        contents.push_str(&write_prv(prv.clone()));
    }
    
    contents
}

// Placeholder for III segment functions until we implement them
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct III {
    pub code_list_qualifier_code: String,
    pub industry_code: String,
    pub code_category: String,
    pub free_form_message_text: String,
}

pub fn get_iii(iii_content: String) -> III {
    III::default()
}

pub fn write_iii(iii: III) -> String {
    format!("III*{}*{}*{}*{}~", iii.code_list_qualifier_code, iii.industry_code, 
            iii.code_category, iii.free_form_message_text)
}

// Placeholder for LS segment functions until we implement them
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LS {
    pub loop_identifier_code: String,
}

pub fn get_ls(ls_content: String) -> LS {
    LS::default()
}

pub fn write_ls(ls: LS) -> String {
    format!("LS*{}~", ls.loop_identifier_code)
}

// Placeholder for LE segment functions until we implement them
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LE {
    pub loop_identifier_code: String,
}

pub fn get_le(le_content: String) -> LE {
    LE::default()
}

pub fn write_le(le: LE) -> String {
    format!("LE*{}~", le.loop_identifier_code)
}

// Import required segments
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::segments::nm1::*;
use crate::edi271::loop2000c::*;
