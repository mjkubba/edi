use log::info;
use serde::{Deserialize, Serialize};

use crate::segments::n1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop1000B {
    pub n1: N1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
}

pub fn get_loop1000b(contents: &str) -> (Loop1000B, String) {
    let mut contents = contents.to_string();
    let mut loop1000b = Loop1000B::default();

    // Parse N1 segment (required)
    if let Some(n1_start) = contents.find("N1*") {
        if let Some(n1_end) = contents[n1_start..].find("~") {
            let n1_content = &contents[n1_start + 3..n1_start + n1_end];
            loop1000b.n1 = get_n1(n1_content.to_string());
            contents = contents[n1_start + n1_end + 1..].to_string();
        }
    }

    // Parse N3 segment (optional)
    if let Some(n3_start) = contents.find("N3*") {
        // Check if this N3 is before the next major segment
        let next_major = ["N1*", "N4*", "PER*", "REF*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if next_major.map_or(true, |m| n3_start < m) {
            if let Some(n3_end) = contents[n3_start..].find("~") {
                let n3_content = &contents[n3_start + 3..n3_start + n3_end];
                loop1000b.n3 = Some(get_n3(n3_content.to_string()));
                contents = contents[n3_start + n3_end + 1..].to_string();
            }
        }
    }

    // Parse N4 segment (optional)
    if let Some(n4_start) = contents.find("N4*") {
        // Check if this N4 is before the next major segment
        let next_major = ["N1*", "PER*", "REF*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if next_major.map_or(true, |m| n4_start < m) {
            if let Some(n4_end) = contents[n4_start..].find("~") {
                let n4_content = &contents[n4_start + 3..n4_start + n4_end];
                loop1000b.n4 = Some(get_n4(n4_content.to_string()));
                contents = contents[n4_start + n4_end + 1..].to_string();
            }
        }
    }

    // Parse PER segments (optional)
    while let Some(per_start) = contents.find("PER*") {
        // Check if this PER is before the next major segment
        let next_major = ["N1*", "REF*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if let Some(next_pos) = next_major {
            if per_start > next_pos {
                break;
            }
        }

        if let Some(per_end) = contents[per_start..].find("~") {
            let per_content = &contents[per_start + 4..per_start + per_end];
            loop1000b
                .per_segments
                .push(get_per(per_content.to_string()));
            contents = contents[per_start + per_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse REF segments (optional)
    while let Some(ref_start) = contents.find("REF*") {
        // Check if this REF is before the next major segment
        let next_major = ["N1*", "INS*", "SE*"]
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
            loop1000b
                .ref_segments
                .push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }

    info!("Parsed Loop1000B: {:?}", loop1000b);
    (loop1000b, contents)
}

pub fn write_loop1000b(loop1000b: Loop1000B) -> String {
    let mut result = String::new();

    result.push_str(&write_n1(loop1000b.n1));
    result.push_str("\n");

    if let Some(n3) = loop1000b.n3 {
        result.push_str(&write_n3(n3));
        result.push_str("\n");
    }

    if let Some(n4) = loop1000b.n4 {
        result.push_str(&write_n4(n4));
        result.push_str("\n");
    }

    for per_segment in loop1000b.per_segments {
        result.push_str(&write_per(per_segment));
        result.push_str("\n");
    }

    for ref_segment in loop1000b.ref_segments {
        result.push_str(&write_ref(ref_segment));
        result.push_str("\n");
    }

    result
}
