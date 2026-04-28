use log::info;
use serde::{Deserialize, Serialize};

use crate::edi834::loop2100_generic::*;
use crate::edi834::loop2100a::*;
use crate::edi834::loop2300::*;
use crate::segments::dtp::*;
use crate::segments::ins::*;
use crate::segments::r#ref::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000 {
    pub ins: INS,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub loop2100a: Option<Loop2100A>,     // Member Name
    pub loop2100b: Option<Loop2100B>,     // Incorrect Member Name
    pub loop2100c: Option<Loop2100C>,     // Member Mailing Address
    pub loop2100d: Option<Loop2100D>,     // Member Employer
    pub loop2100e: Option<Loop2100E>,     // Member School
    pub loop2100f: Option<Loop2100F>,     // Custodial Parent
    pub loop2100g: Option<Loop2100G>,     // Responsible Person
    pub loop2100h: Option<Loop2100H>,     // Drop Off Location
    pub loop2300_segments: Vec<Loop2300>, // Health Coverage (contains Loop2320/2330)
}

/// Check if segment exists within current member boundary (before next INS* or SE*)
fn in_current_member(contents: &str, segment: &str) -> bool {
    let ins_pos = contents.find("INS*").unwrap_or(contents.len());
    let se_pos = contents.find("SE*").unwrap_or(contents.len());
    let boundary = ins_pos.min(se_pos);
    contents[..boundary].contains(segment)
}

pub fn get_loop2000(contents: &str) -> (Loop2000, String) {
    let mut contents = contents.to_string();
    let mut loop2000 = Loop2000::default();

    // Parse INS segment (required)
    if let Some(ins_start) = contents.find("INS*") {
        if let Some(ins_end) = contents[ins_start..].find("~") {
            let ins_content = &contents[ins_start + 4..ins_start + ins_end];
            loop2000.ins = get_ins(ins_content.to_string());
            contents = contents[ins_start + ins_end + 1..].to_string();
        }
    }

    // Parse REF segments
    while let Some(ref_start) = contents.find("REF*") {
        // Check if this REF is before the next major segment
        let next_major = ["DTP*", "NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if let Some(next_pos) = next_major {
            if ref_start > next_pos {
                break;
            }
        }

        if let Some(ref_end) = contents[ref_start..].find("~") {
            let ref_content = &contents[ref_start + 4..ref_start + ref_end];
            loop2000.ref_segments.push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse DTP segments
    while let Some(dtp_start) = contents.find("DTP*") {
        // Check if this DTP is before the next major segment
        let next_major = ["NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if let Some(next_pos) = next_major {
            if dtp_start > next_pos {
                break;
            }
        }

        if let Some(dtp_end) = contents[dtp_start..].find("~") {
            let dtp_content = &contents[dtp_start + 4..dtp_start + dtp_end];
            loop2000.dtp_segments.push(get_dtp(dtp_content.to_string()));
            contents = contents[dtp_start + dtp_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse Loop2100A (Member Name)
    if in_current_member(&contents, "NM1*IL*") {
        let (loop2100a, new_contents) = get_loop2100a(&contents);
        loop2000.loop2100a = Some(loop2100a);
        contents = new_contents;
    }

    // Parse Loop2100B (Incorrect Member Name)
    if in_current_member(&contents, "NM1*70*") {
        let (loop2100b, new_contents) = get_loop2100b(&contents);
        loop2000.loop2100b = Some(loop2100b);
        contents = new_contents;
    }

    // Parse Loop2100C (Member Mailing Address)
    if in_current_member(&contents, "NM1*31*") {
        let (loop2100c, new_contents) = get_loop2100c(&contents);
        loop2000.loop2100c = Some(loop2100c);
        contents = new_contents;
    }

    // Parse Loop2100D (Member Employer)
    if in_current_member(&contents, "NM1*36*") {
        let (loop2100d, new_contents) = get_loop2100d(&contents);
        loop2000.loop2100d = Some(loop2100d);
        contents = new_contents;
    }

    // Parse Loop2100E (Member School)
    if in_current_member(&contents, "NM1*M8*") {
        let (loop2100e, new_contents) = get_loop2100e(&contents);
        loop2000.loop2100e = Some(loop2100e);
        contents = new_contents;
    }

    // Parse Loop2100F (Custodial Parent)
    if in_current_member(&contents, "NM1*S1*") {
        let (loop2100f, new_contents) = get_loop2100f(&contents);
        loop2000.loop2100f = Some(loop2100f);
        contents = new_contents;
    }

    // Parse Loop2100G (Responsible Person)
    if in_current_member(&contents, "NM1*6Y*") {
        let (loop2100g, new_contents) = get_loop2100g(&contents);
        loop2000.loop2100g = Some(loop2100g);
        contents = new_contents;
    }

    // Parse Loop2100H (Drop Off Location)
    if in_current_member(&contents, "NM1*9K*") {
        let (loop2100h, new_contents) = get_loop2100h(&contents);
        loop2000.loop2100h = Some(loop2100h);
        contents = new_contents;
    }

    // Parse Loop2300 segments (Health Coverage — contains nested Loop2320/2330)
    while in_current_member(&contents, "HD*") {
        let (loop2300, new_contents) = get_loop2300(&contents);
        loop2000.loop2300_segments.push(loop2300);
        contents = new_contents;
    }

    info!("Parsed Loop2000: {:?}", loop2000);
    (loop2000, contents)
}

pub fn write_loop2000(loop2000: Loop2000) -> String {
    let mut result = String::new();

    result.push_str(&write_ins(loop2000.ins));
    result.push_str("\n");

    for ref_segment in loop2000.ref_segments {
        result.push_str(&write_ref(ref_segment));
        result.push_str("\n");
    }

    for dtp_segment in loop2000.dtp_segments {
        result.push_str(&write_dtp(dtp_segment));
        result.push_str("\n");
    }

    if let Some(loop2100a) = loop2000.loop2100a {
        result.push_str(&write_loop2100a(loop2100a));
    }

    if let Some(loop2100b) = loop2000.loop2100b {
        result.push_str(&write_loop2100b(loop2100b));
    }

    if let Some(loop2100c) = loop2000.loop2100c {
        result.push_str(&write_loop2100c(loop2100c));
    }

    if let Some(loop2100d) = loop2000.loop2100d {
        result.push_str(&write_loop2100d(loop2100d));
    }

    if let Some(loop2100e) = loop2000.loop2100e {
        result.push_str(&write_loop2100e(loop2100e));
    }

    if let Some(loop2100f) = loop2000.loop2100f {
        result.push_str(&write_loop2100f(loop2100f));
    }

    if let Some(loop2100g) = loop2000.loop2100g {
        result.push_str(&write_loop2100g(loop2100g));
    }

    if let Some(loop2100h) = loop2000.loop2100h {
        result.push_str(&write_loop2100h(loop2100h));
    }

    for loop2300 in loop2000.loop2300_segments {
        result.push_str(&write_loop2300(loop2300));
    }

    result
}
