use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::isa::*;
use crate::segments::gs::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa_segments: ISA,
    pub gs_segments: GS,
}

pub fn get_interchange_header(mut contents: String) -> (InterchangeHeader, String) {
    let mut interchange_header = InterchangeHeader::default();
    
    // Process ISA segment (required)
    if contents.contains("ISA") {
        info!("ISA segment found");
        let isa_content = get_segment_contents("ISA", &contents);
        interchange_header.isa_segments = get_isa(isa_content);
        info!("ISA segment parsed");
        contents = content_trim("ISA", contents);
    } else {
        info!("Warning: Required ISA segment not found");
    }
    
    // Process GS segment (required)
    if contents.contains("GS") {
        info!("GS segment found");
        let gs_content = get_segment_contents("GS", &contents);
        interchange_header.gs_segments = get_gs(gs_content);
        info!("GS segment parsed");
        contents = content_trim("GS", contents);
    } else {
        info!("Warning: Required GS segment not found");
    }
    
    info!("Interchange Control parsed\n");
    (interchange_header, contents)
}

pub fn write_interchange_control(interchange_header: &InterchangeHeader) -> String {
    let mut contents = String::new();
    
    // Write ISA segment
    contents.push_str(&write_isa(interchange_header.isa_segments.clone()));
    
    // Write GS segment
    contents.push_str(&write_gs(interchange_header.gs_segments.clone()));
    
    contents
}
