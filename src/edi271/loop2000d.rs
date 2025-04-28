use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::trn::*;
use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::aaa::*;
use crate::segments::dmg::*;
use crate::segments::ins::*;
use crate::segments::dtp::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};
use crate::edi271::loop2000c::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000D {
    pub hl_segments: HL,
    pub trn_segments: Option<TRN>,
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub aaa_segments: Vec<AAA>,
    pub dmg_segments: Option<DMG>,
    pub ins_segments: Option<INS>,
    pub dtp_segments: Vec<DTP>,
    pub loop2100d: Vec<Loop2100D>,
    pub loop2110d: Vec<Loop2110D>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100D {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub aaa_segments: Vec<AAA>,
    pub prv_segments: Option<PRV>,
    pub dmg_segments: Option<DMG>,
    pub ins_segments: Option<INS>,
    pub dtp_segments: Vec<DTP>,
}

pub fn get_loop_2000d(mut contents: String) -> EdiResult<(Loop2000D, String)> {
    let mut loop2000d = Loop2000D::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        if hl_content.is_empty() {
            return Err(EdiError::MissingSegment("HL".to_string()));
        }
        loop2000d.hl_segments = get_hl(hl_content);
        
        // Verify this is a Dependent level HL segment (level code = 23)
        if loop2000d.hl_segments.hl03_hierarchical_level_code != "23" {
            return Err(EdiError::ValidationError(format!(
                "Expected HL03 code '23' for Dependent level, got '{}'",
                loop2000d.hl_segments.hl03_hierarchical_level_code
            )));
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents);
    } else {
        return Err(EdiError::MissingSegment("HL".to_string()));
    }
    
    // Process TRN segment (situational)
    if contents.contains("TRN") {
        info!("TRN segment found");
        let trn_content = get_segment_contents("TRN", &contents);
        if !trn_content.is_empty() {
            loop2000d.trn_segments = Some(get_trn(trn_content));
            info!("TRN segment parsed");
            contents = content_trim("TRN", contents);
        }
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2000d.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
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
        loop2000d.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2000d.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2000d.n4_segments = Some(get_n4(n4_content));
            info!("N4 segment parsed");
            contents = content_trim("N4", contents);
        }
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
        loop2000d.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found");
        let dmg_content = get_segment_contents("DMG", &contents);
        if !dmg_content.is_empty() {
            loop2000d.dmg_segments = Some(get_dmg(dmg_content));
            info!("DMG segment parsed");
            contents = content_trim("DMG", contents);
        }
    }
    
    // Process INS segment (situational)
    if contents.contains("INS") {
        info!("INS segment found");
        let ins_content = get_segment_contents("INS", &contents);
        if !ins_content.is_empty() {
            loop2000d.ins_segments = Some(get_ins(ins_content));
            info!("INS segment parsed");
            contents = content_trim("INS", contents);
        }
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
        loop2000d.dtp_segments.push(dtp);
        contents = content_trim("DTP", contents);
    }
    
    // Process Loop 2100D segments (can be multiple)
    while contents.contains("NM1") && !is_next_loop_2110d(&contents) {
        match get_loop_2100d(contents.clone()) {
            Ok((loop2100d, new_contents)) => {
                loop2000d.loop2100d.push(loop2100d);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    // Process Loop 2110D segments (can be multiple)
    while contents.contains("EB") {
        match get_loop_2110d(contents.clone()) {
            Ok((loop2110d, new_contents)) => {
                loop2000d.loop2110d.push(loop2110d);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2000D parsed");
    Ok((loop2000d, contents))
}

// Helper function to check if the next segment starts a new 2110D loop
fn is_next_loop_2110d(contents: &str) -> bool {
    contents.contains("EB*")
}

pub fn get_loop_2100d(mut contents: String) -> EdiResult<(Loop2100D, String)> {
    let mut loop2100d = Loop2100D::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2100D");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2100d.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed for Loop 2100D");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found for Loop 2100D");
        let ref_content = get_segment_contents("REF", &contents);
        if ref_content.is_empty() {
            break;
        }
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed for Loop 2100D");
        loop2100d.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found for Loop 2100D");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2100d.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed for Loop 2100D");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found for Loop 2100D");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2100d.n4_segments = Some(get_n4(n4_content));
            info!("N4 segment parsed for Loop 2100D");
            contents = content_trim("N4", contents);
        }
    }
    
    // Process AAA segments (situational, can be multiple)
    while contents.starts_with("AAA") {
        info!("AAA segment found for Loop 2100D");
        let aaa_content = get_segment_contents("AAA", &contents);
        if aaa_content.is_empty() {
            break;
        }
        let aaa = get_aaa(aaa_content);
        info!("AAA segment parsed for Loop 2100D");
        loop2100d.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process PRV segment (situational)
    if contents.contains("PRV") {
        info!("PRV segment found for Loop 2100D");
        let prv_content = get_segment_contents("PRV", &contents);
        if !prv_content.is_empty() {
            loop2100d.prv_segments = Some(get_prv(prv_content));
            info!("PRV segment parsed for Loop 2100D");
            contents = content_trim("PRV", contents);
        }
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found for Loop 2100D");
        let dmg_content = get_segment_contents("DMG", &contents);
        if !dmg_content.is_empty() {
            loop2100d.dmg_segments = Some(get_dmg(dmg_content));
            info!("DMG segment parsed for Loop 2100D");
            contents = content_trim("DMG", contents);
        }
    }
    
    // Process INS segment (situational)
    if contents.contains("INS") {
        info!("INS segment found for Loop 2100D");
        let ins_content = get_segment_contents("INS", &contents);
        if !ins_content.is_empty() {
            loop2100d.ins_segments = Some(get_ins(ins_content));
            info!("INS segment parsed for Loop 2100D");
            contents = content_trim("INS", contents);
        }
    }
    
    // Process DTP segments (situational, can be multiple)
    while contents.starts_with("DTP") {
        info!("DTP segment found for Loop 2100D");
        let dtp_content = get_segment_contents("DTP", &contents);
        if dtp_content.is_empty() {
            break;
        }
        let dtp = get_dtp(dtp_content);
        info!("DTP segment parsed for Loop 2100D");
        loop2100d.dtp_segments.push(dtp);
        contents = content_trim("DTP", contents);
    }
    
    info!("Loop 2100D parsed");
    Ok((loop2100d, contents))
}

pub fn write_loop_2000d(loop2000d: &Loop2000D) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000d.hl_segments.clone()));
    
    // Write TRN segment if present
    if let Some(trn) = &loop2000d.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000d.nm1_segments.clone()));
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000d.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000d.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000d.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop2000d.ins_segments {
        contents.push_str(&write_ins(ins.clone()));
    }
    
    // Write all DTP segments
    for dtp in &loop2000d.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write all Loop 2100D segments
    for loop2100d in &loop2000d.loop2100d {
        contents.push_str(&write_loop_2100d(loop2100d));
    }
    
    // Write all REF segments
    for ref_segment in &loop2000d.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2000d.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write all Loop 2110D segments
    for loop2110d in &loop2000d.loop2110d {
        contents.push_str(&write_loop_2110d(loop2110d));
    }
    
    contents
}

pub fn write_loop_2100d(loop2100d: &Loop2100D) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2100d.nm1_segments.clone()));
    
    // Write all REF segments
    for ref_segment in &loop2100d.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2100d.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2100d.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2100d.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write PRV segment if present
    if let Some(prv) = &loop2100d.prv_segments {
        contents.push_str(&write_prv(prv.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2100d.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop2100d.ins_segments {
        contents.push_str(&write_ins(ins.clone()));
    }
    
    // Write all DTP segments
    for dtp in &loop2100d.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    contents
}

// Import Loop2110D to avoid circular dependency
use crate::edi271::loop2110d::*;
