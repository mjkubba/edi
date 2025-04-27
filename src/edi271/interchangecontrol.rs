use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::isa::*;
use crate::segments::gs::*;
use crate::segments::ge::*;
use crate::segments::iea::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa_segments: ISA,
    pub gs_segments: GS,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub ge_segments: GE,
    pub iea_segments: IEA,
}

pub fn get_interchange_header(mut contents: String) -> (InterchangeHeader, String) {
    let mut interchange_header = InterchangeHeader::default();
    
    // Process ISA segment
    if contents.contains("ISA") {
        info!("ISA segment found");
        let isa_content = get_segment_contents("ISA", &contents);
        interchange_header.isa_segments = get_isa(isa_content);
        info!("ISA segment parsed");
        contents = content_trim("ISA", contents);
    } else {
        info!("Warning: Required ISA segment not found");
    }
    
    // Process GS segment
    if contents.contains("GS") {
        info!("GS segment found");
        let gs_content = get_segment_contents("GS", &contents);
        interchange_header.gs_segments = get_gs(gs_content);
        info!("GS segment parsed");
        contents = content_trim("GS", contents);
    } else {
        info!("Warning: Required GS segment not found");
    }
    
    info!("Interchange Control parsed\n    ");
    (interchange_header, contents)
}

pub fn get_interchange_trailer(mut contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();
    
    // Process GE segment
    if contents.contains("GE") {
        info!("GE segment found");
        let ge_content = get_segment_contents("GE", &contents);
        interchange_trailer.ge_segments = get_ge(ge_content);
        info!("GE segment parsed");
        contents = content_trim("GE", contents);
    } else {
        info!("Warning: Required GE segment not found");
    }
    
    // Process IEA segment
    if contents.contains("IEA") {
        info!("IEA segment found");
        let iea_content = get_segment_contents("IEA", &contents);
        interchange_trailer.iea_segments = get_iea(iea_content);
        info!("IEA segment parsed");
        contents = content_trim("IEA", contents);
    } else {
        info!("Warning: Required IEA segment not found");
    }
    
    info!("Interchange Control Trailer parsed\n    ");
    (interchange_trailer, contents)
}

pub fn write_interchange_control(interchange_header: &InterchangeHeader) -> String {
    let mut contents = String::new();
    
    // Write ISA segment
    contents.push_str(&write_isa(interchange_header.isa_segments.clone()));
    
    // Write GS segment
    contents.push_str(&write_gs(interchange_header.gs_segments.clone()));
    
    contents
}

pub fn write_interchange_trailer(interchange_trailer: &InterchangeTrailer) -> String {
    let mut contents = String::new();
    
    // Write GE segment
    contents.push_str(&write_ge(interchange_trailer.ge_segments.clone()));
    
    // Write IEA segment
    contents.push_str(&write_iea(interchange_trailer.iea_segments.clone()));
    
    contents
}
