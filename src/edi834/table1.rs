use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::st::*;
use crate::segments::bgn::*;
use crate::segments::r#ref::*;
use crate::segments::dtp::*;
use crate::segments::qty::*;
use crate::segments::n1::*;
use crate::error::EdiResult;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1 {
    pub st: ST,
    pub bgn: BGN,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub qty_segments: Vec<QTY>,
    pub n1_segments: Vec<N1>,
}

pub fn get_table1(mut contents: String) -> EdiResult<(Table1, String)> {
    let mut table1 = Table1::default();
    
    // Parse ST segment
    if let Some(st_start) = contents.find("ST*") {
        if let Some(st_end) = contents[st_start..].find("~") {
            let st_content = &contents[st_start + 3..st_start + st_end];
            table1.st = get_st(st_content.to_string());
            contents = contents[st_start + st_end + 1..].to_string();
        }
    }
    
    // Parse BGN segment
    if let Some(bgn_start) = contents.find("BGN*") {
        if let Some(bgn_end) = contents[bgn_start..].find("~") {
            let bgn_content = &contents[bgn_start + 4..bgn_start + bgn_end];
            table1.bgn = get_bgn(bgn_content.to_string());
            contents = contents[bgn_start + bgn_end + 1..].to_string();
        }
    }
    
    // Parse REF segments
    while let Some(ref_start) = contents.find("REF*") {
        // Check if this REF is before the next major segment
        let next_major = ["N1*", "DTP*", "QTY*", "Loop1000A", "Loop1000B", "Loop2000"]
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
            table1.ref_segments.push(get_ref(ref_content.to_string()));
            contents = contents[ref_start + ref_end + 1..].to_string();
        } else {
            break;
        }
    }
    
    // Parse DTP segments
    while let Some(dtp_start) = contents.find("DTP*") {
        // Check if this DTP is before the next major segment
        let next_major = ["N1*", "QTY*", "Loop1000A", "Loop1000B", "Loop2000"]
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
            table1.dtp_segments.push(get_dtp(dtp_content.to_string()));
            contents = contents[dtp_start + dtp_end + 1..].to_string();
        } else {
            break;
        }
    }
    
    // Parse QTY segments
    while let Some(qty_start) = contents.find("QTY*") {
        // Check if this QTY is before the next major segment
        let next_major = ["N1*", "Loop1000A", "Loop1000B", "Loop2000"]
            .iter()
            .filter_map(|seg| contents.find(seg))
            .min();
            
        if let Some(next_pos) = next_major {
            if qty_start > next_pos {
                break;
            }
        }
        
        if let Some(qty_end) = contents[qty_start..].find("~") {
            let qty_content = &contents[qty_start + 4..qty_start + qty_end];
            table1.qty_segments.push(get_qty(qty_content.to_string()));
            contents = contents[qty_start + qty_end + 1..].to_string();
        } else {
            break;
        }
    }
    
    info!("Parsed Table1: {:?}", table1);
    Ok((table1, contents))
}

pub fn write_table1(table1: Table1) -> String {
    let mut result = String::new();
    
    result.push_str(&write_st(table1.st));
    result.push_str("\n");
    result.push_str(&write_bgn(table1.bgn));
    result.push_str("\n");
    
    // Write REF segments
    for ref_segment in table1.ref_segments {
        result.push_str(&write_ref(ref_segment));
        result.push_str("\n");
    }
    
    // Write DTP segments
    for dtp_segment in table1.dtp_segments {
        result.push_str(&write_dtp(dtp_segment));
        result.push_str("\n");
    }
    
    // Write QTY segments
    for qty_segment in table1.qty_segments {
        result.push_str(&write_qty(qty_segment));
        result.push_str("\n");
    }
    
    result
}