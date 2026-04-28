use log::info;
use serde::{Deserialize, Serialize};

use crate::edi834::loop2320::*;
use crate::segments::amt::*;
use crate::segments::dtp::*;
use crate::segments::hd::*;
use crate::segments::r#ref::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    pub hd: HD,
    pub dtp_segments: Vec<DTP>,
    pub amt_segments: Vec<AMT>,
    pub ref_segments: Vec<REF>,
    pub loop2320_segments: Vec<Loop2320>,
}

pub fn get_loop2300(mut contents: String) -> (Loop2300, String) {
    let mut loop2300 = Loop2300::default();

    // Parse HD segment (required)
    if let Some(hd_start) = contents.find("HD*") {
        if let Some(hd_end) = contents[hd_start..].find("~") {
            let hd_content = &contents[hd_start + 3..hd_start + hd_end];
            loop2300.hd = get_hd(hd_content.to_string());
            contents = contents[hd_start + hd_end + 1..].to_string();
        }
    }

    // Parse DTP segments (optional)
    while let Some(dtp_start) = contents.find("DTP*") {
        // Check if this DTP is before the next major segment
        let next_major = ["AMT*", "REF*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
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
            loop2300.dtp_segments.push(get_dtp(dtp_content.to_string()));
            contents = contents[dtp_start + dtp_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse AMT segments (optional)
    while let Some(amt_start) = contents.find("AMT*") {
        // Check if this AMT is before the next major segment
        let next_major = ["REF*", "HD*", "COB*", "DSB*", "INS*", "SE*"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();

        if let Some(next_pos) = next_major {
            if amt_start > next_pos {
                break;
            }
        }

        if let Some(amt_end) = contents[amt_start..].find("~") {
            let amt_content = &contents[amt_start + 4..amt_start + amt_end];
            loop2300.amt_segments.push(get_amt(amt_content.to_string()));
            contents = contents[amt_start + amt_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse REF segments (optional)
    while let Some(ref_start) = contents.find("REF*") {
        // Check if this REF is before the next major segment
        let next_major = ["HD*", "COB*", "DSB*", "INS*", "SE*"]
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
            loop2300.ref_segments.push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }

    // Parse Loop2320 segments (Coordination of Benefits) — nested inside Loop2300
    while contents.contains("COB*") {
        let boundary = ["HD*", "INS*", "SE*", "LC*"]
            .iter()
            .filter_map(|s| contents.find(s))
            .min()
            .unwrap_or(contents.len());
        let cob_pos = match contents.find("COB*") {
            Some(pos) => pos,
            None => break,
        };
        if cob_pos > boundary {
            break;
        }
        let (loop2320, new_contents) = get_loop2320(contents);
        loop2300.loop2320_segments.push(loop2320);
        contents = new_contents;
    }

    info!("Parsed Loop2300: {:?}", loop2300);
    (loop2300, contents)
}

pub fn write_loop2300(loop2300: Loop2300) -> String {
    let mut result = String::new();

    result.push_str(&write_hd(loop2300.hd));
    result.push_str("\n");

    for dtp_segment in loop2300.dtp_segments {
        result.push_str(&write_dtp(dtp_segment));
        result.push_str("\n");
    }

    for amt_segment in loop2300.amt_segments {
        result.push_str(&write_amt(amt_segment));
        result.push_str("\n");
    }

    for ref_segment in loop2300.ref_segments {
        result.push_str(&write_ref(ref_segment));
        result.push_str("\n");
    }

    for loop2320 in loop2300.loop2320_segments {
        result.push_str(&write_loop2320(loop2320));
    }

    result
}
