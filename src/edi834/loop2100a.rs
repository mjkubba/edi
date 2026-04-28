use log::info;
use serde::{Deserialize, Serialize};

use crate::segments::dmg::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::nm1::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100A {
    pub nm1: NM1,
    pub per_segments: Vec<PER>,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub ref_segments: Vec<REF>,
}

pub fn get_loop2100a(contents: &str) -> (Loop2100A, String) {
    let mut contents = contents.to_string();
    let mut loop2100a = Loop2100A::default();

    // Parse NM1 segment (required) - looking for IL (Insured or Subscriber)
    if let Some(nm1_start) = contents.find("NM1*IL*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 4..nm1_start + nm1_end];
            loop2100a.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    // Parse PER segments (optional)
    while let Some(per_start) = contents.find("PER*") {
        // Check if this PER is before the next major segment
        let next_major = [
            "N3*", "N4*", "DMG*", "REF*", "NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*",
        ]
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
            loop2100a
                .per_segments
                .push(get_per(per_content.to_string()));
            contents = contents[per_start + per_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse N3 segment (optional)
    if let Some(n3_start) = contents.find("N3*") {
        // Check if this N3 is before the next major segment
        let next_major = [
            "N4*", "DMG*", "REF*", "NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*",
        ]
        .iter()
        .filter_map(|seg| contents.find(seg))
        .min();

        if next_major.map_or(true, |m| n3_start < m) {
            if let Some(n3_end) = contents[n3_start..].find("~") {
                let n3_content = &contents[n3_start + 3..n3_start + n3_end];
                loop2100a.n3 = Some(get_n3(n3_content.to_string()));
                contents = contents[n3_start + n3_end + 1..].to_string();
            }
        }
    }

    // Parse N4 segment (optional)
    if let Some(n4_start) = contents.find("N4*") {
        // Check if this N4 is before the next major segment
        let next_major = ["DMG*", "REF*", "NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if next_major.map_or(true, |m| n4_start < m) {
            if let Some(n4_end) = contents[n4_start..].find("~") {
                let n4_content = &contents[n4_start + 3..n4_start + n4_end];
                loop2100a.n4 = Some(get_n4(n4_content.to_string()));
                contents = contents[n4_start + n4_end + 1..].to_string();
            }
        }
    }

    // Parse DMG segment (optional)
    if let Some(dmg_start) = contents.find("DMG*") {
        // Check if this DMG is before the next major segment
        let next_major = ["REF*", "NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if next_major.map_or(true, |m| dmg_start < m) {
            if let Some(dmg_end) = contents[dmg_start..].find("~") {
                let dmg_content = &contents[dmg_start + 4..dmg_start + dmg_end];
                loop2100a.dmg = Some(get_dmg(dmg_content.to_string()));
                contents = contents[dmg_start + dmg_end + 1..].to_string();
            }
        }
    }

    // Parse REF segments (optional)
    while let Some(ref_start) = contents.find("REF*") {
        // Check if this REF is before the next major segment
        let next_major = ["NM1*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
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
            loop2100a
                .ref_segments
                .push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }

    info!("Parsed Loop2100A: {:?}", loop2100a);
    (loop2100a, contents)
}

pub fn write_loop2100a(loop2100a: Loop2100A) -> String {
    let mut result = String::new();

    result.push_str(&write_nm1(loop2100a.nm1));
    result.push_str("\n");

    for per_segment in loop2100a.per_segments {
        result.push_str(&write_per(per_segment));
        result.push_str("\n");
    }

    if let Some(n3) = loop2100a.n3 {
        result.push_str(&write_n3(n3));
        result.push_str("\n");
    }

    if let Some(n4) = loop2100a.n4 {
        result.push_str(&write_n4(n4));
        result.push_str("\n");
    }

    if let Some(dmg) = loop2100a.dmg {
        result.push_str(&write_dmg(dmg));
        result.push_str("\n");
    }

    for ref_segment in loop2100a.ref_segments {
        result.push_str(&write_ref(ref_segment));
        result.push_str("\n");
    }

    result
}
