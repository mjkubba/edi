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

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
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
    pub loop2100c: Vec<Loop2100C>,
    pub loop2110c: Vec<Loop2110C>,
    pub loop2000d: Vec<Loop2000D>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100C {
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

pub fn get_loop_2000c(mut contents: String) -> EdiResult<(Loop2000C, String)> {
    let mut loop2000c = Loop2000C::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        if hl_content.is_empty() {
            return Err(EdiError::MissingSegment("HL".to_string()));
        }
        loop2000c.hl_segments = get_hl(hl_content);
        
        // Verify this is a Subscriber level HL segment (level code = 22)
        if loop2000c.hl_segments.hl03_hierarchical_level_code != "22" {
            return Err(EdiError::ValidationError(format!(
                "Expected HL03 code '22' for Subscriber level, got '{}'",
                loop2000c.hl_segments.hl03_hierarchical_level_code
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
            loop2000c.trn_segments = Some(get_trn(trn_content));
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
        loop2000c.nm1_segments = get_nm1(nm1_content);
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
        loop2000c.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2000c.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2000c.n4_segments = Some(get_n4(n4_content));
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
        loop2000c.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found");
        let dmg_content = get_segment_contents("DMG", &contents);
        if !dmg_content.is_empty() {
            loop2000c.dmg_segments = Some(get_dmg(dmg_content));
            info!("DMG segment parsed");
            contents = content_trim("DMG", contents);
        }
    }
    
    // Process INS segment (situational)
    if contents.contains("INS") {
        info!("INS segment found");
        let ins_content = get_segment_contents("INS", &contents);
        if !ins_content.is_empty() {
            loop2000c.ins_segments = Some(get_ins(ins_content));
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
        loop2000c.dtp_segments.push(dtp);
        contents = content_trim("DTP", contents);
    }
    
    // Process Loop 2100C segments (can be multiple)
    while contents.contains("NM1") && !is_next_loop_2110c(&contents) && !is_next_loop_2000d(&contents) {
        match get_loop_2100c(contents.clone()) {
            Ok((loop2100c, new_contents)) => {
                loop2000c.loop2100c.push(loop2100c);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    // Process Loop 2110C segments (can be multiple)
    while contents.contains("EB") {
        match get_loop_2110c(contents.clone()) {
            Ok((loop2110c, new_contents)) => {
                loop2000c.loop2110c.push(loop2110c);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    // Process Loop 2000D segments (can be multiple)
    while contents.contains("HL") && is_next_loop_2000d(&contents) {
        match get_loop_2000d(contents.clone()) {
            Ok((loop2000d, new_contents)) => {
                loop2000c.loop2000d.push(loop2000d);
                contents = new_contents;
            },
            Err(_) => break,
        }
    }
    
    info!("Loop 2000C parsed");
    Ok((loop2000c, contents))
}

pub fn write_loop_2000c(loop2000c: &Loop2000C) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000c.hl_segments.clone()));
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000c.nm1_segments.clone()));
    
    // Write REF segments
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
    
    // Write TRN segment if present
    if let Some(trn) = &loop2000c.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    // Write AAA segments
    for aaa in &loop2000c.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop2000c.ins_segments {
        contents.push_str(&write_ins(ins.clone()));
    }
    
    // Write DTP segments
    for dtp in &loop2000c.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write all Loop 2100C segments
    for loop2100c in &loop2000c.loop2100c {
        contents.push_str(&write_loop_2100c(loop2100c));
    }
    
    // Write all Loop 2000D segments
    for loop2000d in &loop2000c.loop2000d {
        contents.push_str(&write_loop_2000d(loop2000d));
    }
    
    // Write all Loop 2110C segments - in original file, EB segments come after Loop 2000D
    for loop2110c in &loop2000c.loop2110c {
        contents.push_str(&write_loop_2110c(loop2110c));
    }
    
    contents
}

pub fn get_loop_2100c(mut contents: String) -> EdiResult<(Loop2100C, String)> {
    let mut loop2100c = Loop2100C::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2100C");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        loop2100c.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed for Loop 2100C");
        contents = content_trim("NM1", contents);
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found for Loop 2100C");
        let ref_content = get_segment_contents("REF", &contents);
        if ref_content.is_empty() {
            break;
        }
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed for Loop 2100C");
        loop2100c.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found for Loop 2100C");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2100c.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed for Loop 2100C");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found for Loop 2100C");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2100c.n4_segments = Some(get_n4(n4_content));
            info!("N4 segment parsed for Loop 2100C");
            contents = content_trim("N4", contents);
        }
    }
    
    // Process AAA segments (situational, can be multiple)
    while contents.starts_with("AAA") {
        info!("AAA segment found for Loop 2100C");
        let aaa_content = get_segment_contents("AAA", &contents);
        if aaa_content.is_empty() {
            break;
        }
        let aaa = get_aaa(aaa_content);
        info!("AAA segment parsed for Loop 2100C");
        loop2100c.aaa_segments.push(aaa);
        contents = content_trim("AAA", contents);
    }
    
    // Process PRV segment (situational)
    if contents.contains("PRV") {
        info!("PRV segment found for Loop 2100C");
        let prv_content = get_segment_contents("PRV", &contents);
        if !prv_content.is_empty() {
            loop2100c.prv_segments = Some(get_prv(prv_content));
            info!("PRV segment parsed for Loop 2100C");
            contents = content_trim("PRV", contents);
        }
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found for Loop 2100C");
        let dmg_content = get_segment_contents("DMG", &contents);
        if !dmg_content.is_empty() {
            loop2100c.dmg_segments = Some(get_dmg(dmg_content));
            info!("DMG segment parsed for Loop 2100C");
            contents = content_trim("DMG", contents);
        }
    }
    
    // Process INS segment (situational)
    if contents.contains("INS") {
        info!("INS segment found for Loop 2100C");
        let ins_content = get_segment_contents("INS", &contents);
        if !ins_content.is_empty() {
            loop2100c.ins_segments = Some(get_ins(ins_content));
            info!("INS segment parsed for Loop 2100C");
            contents = content_trim("INS", contents);
        }
    }
    
    // Process DTP segments (situational, can be multiple)
    while contents.starts_with("DTP") {
        info!("DTP segment found for Loop 2100C");
        let dtp_content = get_segment_contents("DTP", &contents);
        if dtp_content.is_empty() {
            break;
        }
        let dtp = get_dtp(dtp_content);
        info!("DTP segment parsed for Loop 2100C");
        loop2100c.dtp_segments.push(dtp);
        contents = content_trim("DTP", contents);
    }
    
    info!("Loop 2100C parsed");
    Ok((loop2100c, contents))
}

pub fn write_loop_2100c(loop2100c: &Loop2100C) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2100c.nm1_segments.clone()));
    
    // Write all REF segments
    for ref_segment in &loop2100c.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2100c.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2100c.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write all AAA segments
    for aaa in &loop2100c.aaa_segments {
        contents.push_str(&write_aaa(aaa.clone()));
    }
    
    // Write PRV segment if present
    if let Some(prv) = &loop2100c.prv_segments {
        contents.push_str(&write_prv(prv.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2100c.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop2100c.ins_segments {
        contents.push_str(&write_ins(ins.clone()));
    }
    
    // Write all DTP segments
    for dtp in &loop2100c.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    contents
}

// Placeholder for PRV segment functions until we implement them
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PRV {
    pub provider_code: String,
    pub reference_id_qualifier: String,
    pub reference_id: String,
}

pub fn get_prv(prv_content: String) -> PRV {
    PRV::default()
}

pub fn write_prv(prv: PRV) -> String {
    format!("PRV*{}*{}*{}~", prv.provider_code, prv.reference_id_qualifier, prv.reference_id)
}

// Helper function to check if the next segment starts a new 2110C loop
fn is_next_loop_2110c(contents: &str) -> bool {
    contents.contains("EB*")
}

// Helper function to check if the next segment starts a new 2000D loop
fn is_next_loop_2000d(contents: &str) -> bool {
    if let Some(hl_content) = get_full_segment_contents("HL", contents) {
        let parts: Vec<&str> = hl_content.split('*').collect();
        if parts.len() > 3 && parts[3] == "23" {
            return true;
        }
    }
    false
}

// Import Loop2110C and Loop2000D to avoid circular dependency
use crate::edi271::loop2110c::*;
use crate::edi271::loop2000d::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_next_loop_2000d() {
        let contents = "HL*4*3*23*0~NM1*IL*1*DOE*JANE****MI*98765432101~".to_string();
        assert!(is_next_loop_2000d(&contents));
        
        let contents = "HL*3*2*22*1~NM1*IL*1*DOE*JOHN****MI*12345678901~".to_string();
        assert!(!is_next_loop_2000d(&contents));
    }
}
