use log::info;
use serde::{Deserialize, Serialize};

use crate::segments::ge::*;
use crate::segments::gs::*;
use crate::segments::iea::*;
use crate::segments::isa::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa: ISA,
    pub gs: GS,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub ge: GE,
    pub iea: IEA,
}

pub fn get_interchange_header(mut contents: String) -> (InterchangeHeader, String) {
    let mut interchange_header = InterchangeHeader::default();

    // Parse ISA segment
    if let Some(isa_start) = contents.find("ISA*") {
        if let Some(isa_end) = contents[isa_start..].find("~") {
            let isa_content = &contents[isa_start + 4..isa_start + isa_end];
            interchange_header.isa = get_isa(isa_content.to_string());
            contents = contents[isa_start + isa_end + 1..].to_string();
        }
    }

    // Parse GS segment
    if let Some(gs_start) = contents.find("GS*") {
        if let Some(gs_end) = contents[gs_start..].find("~") {
            let gs_content = &contents[gs_start + 3..gs_start + gs_end];
            interchange_header.gs = get_gs(gs_content.to_string());
            contents = contents[gs_start + gs_end + 1..].to_string();
        }
    }

    info!("Parsed interchange header: {:?}", interchange_header);
    (interchange_header, contents)
}

pub fn get_interchange_trailer(mut contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();

    // Parse GE segment
    if let Some(ge_start) = contents.find("GE*") {
        if let Some(ge_end) = contents[ge_start..].find("~") {
            let ge_content = &contents[ge_start + 3..ge_start + ge_end];
            interchange_trailer.ge = get_ge(ge_content.to_string());
            contents = contents[ge_start + ge_end + 1..].to_string();
        }
    }

    // Parse IEA segment
    if let Some(iea_start) = contents.find("IEA*") {
        if let Some(iea_end) = contents[iea_start..].find("~") {
            let iea_content = &contents[iea_start + 4..iea_start + iea_end];
            interchange_trailer.iea = get_iea(iea_content.to_string());
            contents = contents[iea_start + iea_end + 1..].to_string();
        }
    }

    info!("Parsed interchange trailer: {:?}", interchange_trailer);
    (interchange_trailer, contents)
}

pub fn write_interchange_header(interchange_header: InterchangeHeader) -> String {
    let mut result = String::new();

    result.push_str(&write_isa(interchange_header.isa));
    result.push_str("\n");
    result.push_str(&write_gs(interchange_header.gs));
    result.push_str("\n");

    result
}

pub fn write_interchange_trailer(interchange_trailer: InterchangeTrailer) -> String {
    let mut result = String::new();

    result.push_str(&write_ge(interchange_trailer.ge));
    result.push_str("\n");
    result.push_str(&write_iea(interchange_trailer.iea));

    result
}
