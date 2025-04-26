use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::st::*;
use crate::segments::ak1::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1s {
    pub st_segments: ST,
    pub ak1_segments: AK1,
}

pub fn get_first_table_header(mut contents: String) -> (ST, AK1, String) {
    let mut st_segments = ST::default();
    let mut ak1_segments = AK1::default();
   
    // Process ST segment (required)
    if contents.contains("ST") {
        info!("ST segment found");
        let st_content = get_segment_contents("ST", &contents);
        st_segments = get_st(st_content);
        info!("ST segment parsed");
        contents = content_trim("ST", contents);
    } else {
        info!("Warning: Required ST segment not found");
    }

    // Process AK1 segment (required)
    if contents.contains("AK1") {
        info!("AK1 segment found");
        let ak1_content = get_segment_contents("AK1", &contents);
        ak1_segments = get_ak1(ak1_content);
        info!("AK1 segment parsed");
        contents = content_trim("AK1", contents);
    } else {
        info!("Warning: Required AK1 segment not found");
    }

    info!("Table 1 parsed\n");
    (st_segments, ak1_segments, contents)
}

pub fn get_table1s(contents: String) -> (Table1s, String) {
    let (st_segments, ak1_segments, contents) = get_first_table_header(contents);
    let header = Table1s {
        st_segments,
        ak1_segments,
    };
    (header, contents)
}

pub fn write_table1(table1: &Table1s) -> String {
    let mut contents = String::new();
    
    // Write ST segment
    contents.push_str(&write_st(table1.st_segments.clone()));
    
    // Write AK1 segment
    contents.push_str(&write_ak1(table1.ak1_segments.clone()));
    
    contents
}
