use serde::{Serialize, Deserialize};
use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::trn::*;
use crate::segments::r#ref::*;
use crate::segments::stc::*;
use crate::edi277::loop2100::*;
use crate::edi277::loop2200::*;

// Loop 2000A - Information Source
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100a: Vec<Loop2100A>,
}

// Loop 2000B - Information Receiver
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100b: Vec<Loop2100B>,
}

// Loop 2000C - Service Provider
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub stc_segments: Vec<STC>,
    pub loop2100c: Vec<Loop2100C>,
    pub loop2200c: Vec<Loop2200C>,
}

// Loop 2000D - Subscriber
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000D {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub stc_segments: Vec<STC>,
    pub loop2100d: Vec<Loop2100D>,
    pub loop2200d: Vec<Loop2200D>,
}

// Loop 2000E - Dependent
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000E {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub stc_segments: Vec<STC>,
    pub loop2100e: Vec<Loop2100E>,
    pub loop2200e: Vec<Loop2200E>,
}

pub fn get_loop_2000a(contents: String) -> (Loop2000A, String) {
    let mut loop_2000a = Loop2000A::default();
    let mut remaining_content = contents.clone();
    
    // Process HL segment for Information Source
    if let Some(hl_segment_start) = contents.find("HL") {
        let hl_segment_end = contents[hl_segment_start..].find('~').unwrap_or(contents.len() - hl_segment_start);
        let hl_segment = &contents[hl_segment_start..hl_segment_start + hl_segment_end];
        
        let hl_elements: Vec<&str> = hl_segment.split('*').collect();
        
        if hl_elements.len() >= 4 && hl_elements[3] == "20" {  // 20 is the code for Information Source
            loop_2000a.hl = get_hl(hl_segment.to_string());
            
            // Remove the HL segment from the remaining content
            remaining_content = contents[hl_segment_start + hl_segment_end + 1..].to_string();
            
            // Process NM1 segment
            if let Some(nm1_segment_start) = remaining_content.find("NM1") {
                let nm1_segment_end = remaining_content[nm1_segment_start..].find('~').unwrap_or(remaining_content.len() - nm1_segment_start);
                let nm1_segment = &remaining_content[nm1_segment_start..nm1_segment_start + nm1_segment_end];
                
                loop_2000a.nm1 = get_nm1(nm1_segment.to_string());
                
                // Remove the NM1 segment from the remaining content
                remaining_content = remaining_content[nm1_segment_start + nm1_segment_end + 1..].to_string();
                
                // Process Loop 2100A
                let (loop_2100a_vec, new_remaining) = get_loop_2100a_vec(remaining_content.clone());
                loop_2000a.loop2100a = loop_2100a_vec;
                remaining_content = new_remaining;
            }
        }
    }
    
    (loop_2000a, remaining_content)
}

pub fn get_loop_2000b_vec(contents: String) -> (Vec<Loop2000B>, String) {
    let mut loop_2000b_vec = Vec::new();
    let mut remaining_content = contents.clone();
    
    // Process multiple Loop 2000B
    while let Some(hl_segment_start) = remaining_content.find("HL") {
        let hl_segment_end = remaining_content[hl_segment_start..].find('~').unwrap_or(remaining_content.len() - hl_segment_start);
        let hl_segment = &remaining_content[hl_segment_start..hl_segment_start + hl_segment_end];
        
        let hl_elements: Vec<&str> = hl_segment.split('*').collect();
        
        if hl_elements.len() >= 4 && hl_elements[3] == "21" {  // 21 is the code for Information Receiver
            let mut loop_2000b = Loop2000B::default();
            loop_2000b.hl = get_hl(hl_segment.to_string());
            
            // Remove the HL segment from the remaining content
            remaining_content = remaining_content[hl_segment_start + hl_segment_end + 1..].to_string();
            
            // Process NM1 segment
            if let Some(nm1_segment_start) = remaining_content.find("NM1") {
                let nm1_segment_end = remaining_content[nm1_segment_start..].find('~').unwrap_or(remaining_content.len() - nm1_segment_start);
                let nm1_segment = &remaining_content[nm1_segment_start..nm1_segment_start + nm1_segment_end];
                
                loop_2000b.nm1 = get_nm1(nm1_segment.to_string());
                
                // Remove the NM1 segment from the remaining content
                remaining_content = remaining_content[nm1_segment_start + nm1_segment_end + 1..].to_string();
                
                // Process Loop 2100B
                let (loop_2100b_vec, new_remaining) = get_loop_2100b_vec(remaining_content.clone());
                loop_2000b.loop2100b = loop_2100b_vec;
                remaining_content = new_remaining;
            }
            
            loop_2000b_vec.push(loop_2000b);
        } else {
            // If not a Loop 2000B, break the loop
            break;
        }
    }
    
    (loop_2000b_vec, remaining_content)
}

// Placeholder functions for loop processing
pub fn get_loop_2100a_vec(contents: String) -> (Vec<Loop2100A>, String) {
    // Implementation will be added later
    (Vec::new(), contents)
}

pub fn get_loop_2100b_vec(contents: String) -> (Vec<Loop2100B>, String) {
    // Implementation will be added later
    (Vec::new(), contents)
}

pub fn write_loop_2000a(loop_2000a: &Loop2000A) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&write_hl(loop_2000a.hl.clone()));
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2000a.nm1.clone()));
    
    // Write Loop 2100A
    for loop_2100a in &loop_2000a.loop2100a {
        result.push_str(&write_loop_2100a(loop_2100a));
    }
    
    result
}

pub fn write_loop_2000b_vec(loop_2000b_vec: &[Loop2000B]) -> String {
    let mut result = String::new();
    
    for loop_2000b in loop_2000b_vec {
        // Write HL segment
        result.push_str(&write_hl(loop_2000b.hl.clone()));
        
        // Write NM1 segment
        result.push_str(&write_nm1(loop_2000b.nm1.clone()));
        
        // Write Loop 2100B
        for loop_2100b in &loop_2000b.loop2100b {
            result.push_str(&write_loop_2100b(loop_2100b));
        }
    }
    
    result
}

// Placeholder functions for loop writing
pub fn write_loop_2100a(_loop_2100a: &Loop2100A) -> String {
    // Implementation will be added later
    String::new()
}

pub fn write_loop_2100b(_loop_2100b: &Loop2100B) -> String {
    // Implementation will be added later
    String::new()
}
// Function to write Loop 2000C
pub fn write_loop_2000c(loop_2000c: &Loop2000C) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&write_hl(loop_2000c.hl.clone()));
    result.push('\n');
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2000c.nm1.clone()));
    result.push('\n');
    
    // Write TRN segment
    result.push_str(&write_trn(loop_2000c.trn.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2000c.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write STC segments
    for stc in &loop_2000c.stc_segments {
        result.push_str(&write_stc(stc));
        result.push('\n');
    }
    
    // Write Loop 2100C
    for loop_2100c in &loop_2000c.loop2100c {
        result.push_str(&write_loop_2100c(loop_2100c));
    }
    
    // Write Loop 2200C
    for loop_2200c in &loop_2000c.loop2200c {
        result.push_str(&write_loop_2200c(loop_2200c));
    }
    
    result
}

// Function to write Loop 2000D
pub fn write_loop_2000d(loop_2000d: &Loop2000D) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&write_hl(loop_2000d.hl.clone()));
    result.push('\n');
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2000d.nm1.clone()));
    result.push('\n');
    
    // Write TRN segment
    result.push_str(&write_trn(loop_2000d.trn.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2000d.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write STC segments
    for stc in &loop_2000d.stc_segments {
        result.push_str(&write_stc(stc));
        result.push('\n');
    }
    
    // Write Loop 2100D
    for loop_2100d in &loop_2000d.loop2100d {
        result.push_str(&write_loop_2100d(loop_2100d));
    }
    
    // Write Loop 2200D
    for loop_2200d in &loop_2000d.loop2200d {
        result.push_str(&write_loop_2200d(loop_2200d));
    }
    
    result
}

// Function to write Loop 2000E
pub fn write_loop_2000e(loop_2000e: &Loop2000E) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&write_hl(loop_2000e.hl.clone()));
    result.push('\n');
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2000e.nm1.clone()));
    result.push('\n');
    
    // Write TRN segment
    result.push_str(&write_trn(loop_2000e.trn.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2000e.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write STC segments
    for stc in &loop_2000e.stc_segments {
        result.push_str(&write_stc(stc));
        result.push('\n');
    }
    
    // Write Loop 2100E
    for loop_2100e in &loop_2000e.loop2100e {
        result.push_str(&write_loop_2100e(loop_2100e));
    }
    
    // Write Loop 2200E
    for loop_2200e in &loop_2000e.loop2200e {
        result.push_str(&write_loop_2200e(loop_2200e));
    }
    
    result
}
