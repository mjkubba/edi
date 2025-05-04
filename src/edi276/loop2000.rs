use serde::{Serialize, Deserialize};
use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::trn::*;
use crate::segments::r#ref::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::segments::dmg::*;
use crate::segments::ins::*;
use crate::segments::dtp::*;
use crate::edi276::loop2100::*;
use crate::edi276::loop2200::*;

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
    pub dmg: Option<DMG>,
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
    pub dmg: Option<DMG>,
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

// Process Loop 2100A segments
pub fn get_loop_2100a_vec(contents: String) -> (Vec<Loop2100A>, String) {
    let mut loop_2100a_vec = Vec::new();
    let mut remaining_content = contents.clone();
    
    // Process NM1 segments for Loop 2100A
    while let Some(nm1_segment_start) = remaining_content.find("NM1") {
        let nm1_segment_end = remaining_content[nm1_segment_start..].find('~').unwrap_or(remaining_content.len() - nm1_segment_start);
        let nm1_segment = &remaining_content[nm1_segment_start..nm1_segment_start + nm1_segment_end];
        
        let nm1_elements: Vec<&str> = nm1_segment.split('*').collect();
        
        // Check if this is a Loop 2100A NM1 segment
        if nm1_elements.len() >= 2 && (nm1_elements[1] == "PR" || nm1_elements[1] == "IL") {
            let mut loop_2100a = Loop2100A::default();
            loop_2100a.nm1 = get_nm1(nm1_segment.to_string());
            
            // Remove the NM1 segment from the remaining content
            remaining_content = remaining_content[nm1_segment_start + nm1_segment_end + 1..].to_string();
            
            // Process REF segments
            while let Some(ref_segment_start) = remaining_content.find("REF") {
                let ref_segment_end = remaining_content[ref_segment_start..].find('~').unwrap_or(remaining_content.len() - ref_segment_start);
                let ref_segment = &remaining_content[ref_segment_start..ref_segment_start + ref_segment_end];
                
                // Check if the next segment is not a new loop
                if !remaining_content[ref_segment_start + ref_segment_end + 1..].starts_with("NM1") &&
                   !remaining_content[ref_segment_start + ref_segment_end + 1..].starts_with("HL") {
                    loop_2100a.ref_segments.push(get_ref(ref_segment.to_string()));
                    remaining_content = remaining_content[ref_segment_start + ref_segment_end + 1..].to_string();
                } else {
                    break;
                }
            }
            
            // Process N3 segment
            if let Some(n3_segment_start) = remaining_content.find("N3") {
                let n3_segment_end = remaining_content[n3_segment_start..].find('~').unwrap_or(remaining_content.len() - n3_segment_start);
                let n3_segment = &remaining_content[n3_segment_start..n3_segment_start + n3_segment_end];
                
                loop_2100a.n3 = Some(get_n3(n3_segment.to_string()));
                remaining_content = remaining_content[n3_segment_start + n3_segment_end + 1..].to_string();
            }
            
            // Process N4 segment
            if let Some(n4_segment_start) = remaining_content.find("N4") {
                let n4_segment_end = remaining_content[n4_segment_start..].find('~').unwrap_or(remaining_content.len() - n4_segment_start);
                let n4_segment = &remaining_content[n4_segment_start..n4_segment_start + n4_segment_end];
                
                loop_2100a.n4 = Some(get_n4(n4_segment.to_string()));
                remaining_content = remaining_content[n4_segment_start + n4_segment_end + 1..].to_string();
            }
            
            // Process PER segments
            while let Some(per_segment_start) = remaining_content.find("PER") {
                let per_segment_end = remaining_content[per_segment_start..].find('~').unwrap_or(remaining_content.len() - per_segment_start);
                let per_segment = &remaining_content[per_segment_start..per_segment_start + per_segment_end];
                
                // Check if the next segment is not a new loop
                if !remaining_content[per_segment_start + per_segment_end + 1..].starts_with("NM1") &&
                   !remaining_content[per_segment_start + per_segment_end + 1..].starts_with("HL") {
                    loop_2100a.per_segments.push(get_per(per_segment.to_string()));
                    remaining_content = remaining_content[per_segment_start + per_segment_end + 1..].to_string();
                } else {
                    break;
                }
            }
            
            loop_2100a_vec.push(loop_2100a);
        } else {
            // If not a Loop 2100A NM1 segment, break the loop
            break;
        }
    }
    
    (loop_2100a_vec, remaining_content)
}

// Process Loop 2100B segments
pub fn get_loop_2100b_vec(contents: String) -> (Vec<Loop2100B>, String) {
    let mut loop_2100b_vec = Vec::new();
    let mut remaining_content = contents.clone();
    
    // Process NM1 segments for Loop 2100B
    while let Some(nm1_segment_start) = remaining_content.find("NM1") {
        let nm1_segment_end = remaining_content[nm1_segment_start..].find('~').unwrap_or(remaining_content.len() - nm1_segment_start);
        let nm1_segment = &remaining_content[nm1_segment_start..nm1_segment_start + nm1_segment_end];
        
        let nm1_elements: Vec<&str> = nm1_segment.split('*').collect();
        
        // Check if this is a Loop 2100B NM1 segment
        if nm1_elements.len() >= 2 && (nm1_elements[1] == "41" || nm1_elements[1] == "1P") {
            let mut loop_2100b = Loop2100B::default();
            loop_2100b.nm1 = get_nm1(nm1_segment.to_string());
            
            // Remove the NM1 segment from the remaining content
            remaining_content = remaining_content[nm1_segment_start + nm1_segment_end + 1..].to_string();
            
            // Process REF segments
            while let Some(ref_segment_start) = remaining_content.find("REF") {
                let ref_segment_end = remaining_content[ref_segment_start..].find('~').unwrap_or(remaining_content.len() - ref_segment_start);
                let ref_segment = &remaining_content[ref_segment_start..ref_segment_start + ref_segment_end];
                
                // Check if the next segment is not a new loop
                if !remaining_content[ref_segment_start + ref_segment_end + 1..].starts_with("NM1") &&
                   !remaining_content[ref_segment_start + ref_segment_end + 1..].starts_with("HL") {
                    loop_2100b.ref_segments.push(get_ref(ref_segment.to_string()));
                    remaining_content = remaining_content[ref_segment_start + ref_segment_end + 1..].to_string();
                } else {
                    break;
                }
            }
            
            // Process N3 segment
            if let Some(n3_segment_start) = remaining_content.find("N3") {
                let n3_segment_end = remaining_content[n3_segment_start..].find('~').unwrap_or(remaining_content.len() - n3_segment_start);
                let n3_segment = &remaining_content[n3_segment_start..n3_segment_start + n3_segment_end];
                
                loop_2100b.n3 = Some(get_n3(n3_segment.to_string()));
                remaining_content = remaining_content[n3_segment_start + n3_segment_end + 1..].to_string();
            }
            
            // Process N4 segment
            if let Some(n4_segment_start) = remaining_content.find("N4") {
                let n4_segment_end = remaining_content[n4_segment_start..].find('~').unwrap_or(remaining_content.len() - n4_segment_start);
                let n4_segment = &remaining_content[n4_segment_start..n4_segment_start + n4_segment_end];
                
                loop_2100b.n4 = Some(get_n4(n4_segment.to_string()));
                remaining_content = remaining_content[n4_segment_start + n4_segment_end + 1..].to_string();
            }
            
            // Process PER segments
            while let Some(per_segment_start) = remaining_content.find("PER") {
                let per_segment_end = remaining_content[per_segment_start..].find('~').unwrap_or(remaining_content.len() - per_segment_start);
                let per_segment = &remaining_content[per_segment_start..per_segment_start + per_segment_end];
                
                // Check if the next segment is not a new loop
                if !remaining_content[per_segment_start + per_segment_end + 1..].starts_with("NM1") &&
                   !remaining_content[per_segment_start + per_segment_end + 1..].starts_with("HL") {
                    loop_2100b.per_segments.push(get_per(per_segment.to_string()));
                    remaining_content = remaining_content[per_segment_start + per_segment_end + 1..].to_string();
                } else {
                    break;
                }
            }
            
            loop_2100b_vec.push(loop_2100b);
        } else {
            // If not a Loop 2100B NM1 segment, break the loop
            break;
        }
    }
    
    (loop_2100b_vec, remaining_content)
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
    
    // Write DMG segment if present
    if let Some(dmg) = &loop_2000d.dmg {
        result.push_str(&write_dmg(dmg.clone()));
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
    
    // Write DMG segment if present
    if let Some(dmg) = &loop_2000e.dmg {
        result.push_str(&write_dmg(dmg.clone()));
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

pub fn write_loop_2000a(loop_2000a: &Loop2000A) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&write_hl(loop_2000a.hl.clone()));
    result.push('\n');
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2000a.nm1.clone()));
    result.push('\n');
    
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
        result.push('\n');
        
        // Write NM1 segment
        result.push_str(&write_nm1(loop_2000b.nm1.clone()));
        result.push('\n');
        
        // Write Loop 2100B
        for loop_2100b in &loop_2000b.loop2100b {
            result.push_str(&write_loop_2100b(loop_2100b));
        }
    }
    
    result
}

// Function to write Loop 2100D
pub fn write_loop_2100d(loop_2100d: &Loop2100D) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2100d.nm1.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2100d.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop_2100d.n3 {
        result.push_str(&write_n3(n3.clone()));
        result.push('\n');
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop_2100d.n4 {
        result.push_str(&write_n4(n4.clone()));
        result.push('\n');
    }
    
    // Write PER segments
    for per in &loop_2100d.per_segments {
        result.push_str(&write_per(per.clone()));
        result.push('\n');
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop_2100d.dmg {
        result.push_str(&write_dmg(dmg.clone()));
        result.push('\n');
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop_2100d.ins {
        result.push_str(&write_ins(ins.clone()));
        result.push('\n');
    }
    
    // Write DTP segments
    for dtp in &loop_2100d.dtp_segments {
        result.push_str(&write_dtp(dtp.clone()));
        result.push('\n');
    }
    
    result
}

// Function to write Loop 2100E
pub fn write_loop_2100e(loop_2100e: &Loop2100E) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2100e.nm1.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2100e.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop_2100e.n3 {
        result.push_str(&write_n3(n3.clone()));
        result.push('\n');
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop_2100e.n4 {
        result.push_str(&write_n4(n4.clone()));
        result.push('\n');
    }
    
    // Write PER segments
    for per in &loop_2100e.per_segments {
        result.push_str(&write_per(per.clone()));
        result.push('\n');
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop_2100e.dmg {
        result.push_str(&write_dmg(dmg.clone()));
        result.push('\n');
    }
    
    // Write INS segment if present
    if let Some(ins) = &loop_2100e.ins {
        result.push_str(&write_ins(ins.clone()));
        result.push('\n');
    }
    
    // Write DTP segments
    for dtp in &loop_2100e.dtp_segments {
        result.push_str(&write_dtp(dtp.clone()));
        result.push('\n');
    }
    
    result
}

pub fn write_loop_2100a(loop_2100a: &Loop2100A) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2100a.nm1.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2100a.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop_2100a.n3 {
        result.push_str(&write_n3(n3.clone()));
        result.push('\n');
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop_2100a.n4 {
        result.push_str(&write_n4(n4.clone()));
        result.push('\n');
    }
    
    // Write PER segments
    for per in &loop_2100a.per_segments {
        result.push_str(&write_per(per.clone()));
        result.push('\n');
    }
    
    result
}

pub fn write_loop_2100b(loop_2100b: &Loop2100B) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&write_nm1(loop_2100b.nm1.clone()));
    result.push('\n');
    
    // Write REF segments
    for ref_seg in &loop_2100b.ref_segments {
        result.push_str(&write_ref(ref_seg.clone()));
        result.push('\n');
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop_2100b.n3 {
        result.push_str(&write_n3(n3.clone()));
        result.push('\n');
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop_2100b.n4 {
        result.push_str(&write_n4(n4.clone()));
        result.push('\n');
    }
    
    // Write PER segments
    for per in &loop_2100b.per_segments {
        result.push_str(&write_per(per.clone()));
        result.push('\n');
    }
    
    result
}
