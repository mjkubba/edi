use log::info;
use serde::{Deserialize, Serialize};

use crate::helper::edihelper::*;
use crate::segments::ge::*;
use crate::segments::iea::*;
use crate::segments::se::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub se_segments: SE,
    pub ge_segments: GE,
    pub iea_segments: IEA,
}

pub fn get_interchange_trailer(contents: &str) -> (InterchangeTrailer, String) {
    let mut contents = contents.to_string();
    let mut interchange_trailer = InterchangeTrailer::default();

    if contents.contains("SE*") {
        info!("SE segment found");
        let se_content = get_segment_contents("SE", &contents);
        interchange_trailer.se_segments = get_se(se_content);
        info!("SE segment parsed");
        contents = content_trim("SE", &contents);
    } else {
        info!("Warning: Required SE segment not found");
    }

    if contents.contains("GE*") {
        info!("GE segment found");
        let ge_content = get_segment_contents("GE", &contents);
        interchange_trailer.ge_segments = get_ge(ge_content);
        info!("GE segment parsed");
        contents = content_trim("GE", &contents);
    } else {
        info!("Warning: Required GE segment not found");
    }

    if contents.contains("IEA") {
        info!("IEA segment found");
        let iea_content = get_segment_contents("IEA", &contents);
        interchange_trailer.iea_segments = get_iea(iea_content);
        info!("IEA segment parsed");
        contents = content_trim("IEA", &contents);
    } else {
        info!("Warning: Required IEA segment not found");
    }

    info!("Interchange Control Trailer parsed\n");
    (interchange_trailer, contents)
}

pub fn write_interchange_trailer(interchange_trailer: &InterchangeTrailer) -> String {
    let mut contents = String::new();
    contents.push_str(&write_se(interchange_trailer.se_segments.clone()));
    contents.push_str(&write_ge(interchange_trailer.ge_segments.clone()));
    contents.push_str(&write_iea(interchange_trailer.iea_segments.clone()));
    contents
}
