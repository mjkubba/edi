use log::info;
use serde::{Deserialize, Serialize};

use crate::helper::edihelper::*;
use crate::segments::gs::*;
use crate::segments::isa::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa_segments: ISA,
    pub gs_segments: GS,
}

pub fn get_interchange_header(contents: &str) -> (InterchangeHeader, String) {
    let mut contents = contents.to_string();
    let mut interchange_header = InterchangeHeader::default();

    if contents.contains("ISA") {
        info!("ISA segment found");
        let isa_content = get_segment_contents("ISA", &contents);
        interchange_header.isa_segments = get_isa(isa_content);
        info!("ISA segment parsed");
        contents = content_trim("ISA", &contents);
    } else {
        info!("Warning: Required ISA segment not found");
    }

    if contents.contains("GS") {
        info!("GS segment found");
        let gs_content = get_segment_contents("GS", &contents);
        interchange_header.gs_segments = get_gs(gs_content);
        info!("GS segment parsed");
        contents = content_trim("GS", &contents);
    } else {
        info!("Warning: Required GS segment not found");
    }

    info!("Interchange Control parsed\n");
    (interchange_header, contents)
}

pub fn write_interchange_control(interchange_header: &InterchangeHeader) -> String {
    let mut contents = String::new();
    contents.push_str(&write_isa(interchange_header.isa_segments.clone()));
    contents.push_str(&write_gs(interchange_header.gs_segments.clone()));
    contents
}
