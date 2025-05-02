use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ge::*;
use crate::segments::iea::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub ge_segments: GE,
    pub iea_segments: IEA,
}

pub fn get_interchange_trailer(mut contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();
    
    // Process GE segment (required)
    if contents.contains("GE") {
        info!("GE segment found");
        let ge_content = get_segment_contents("GE", &contents);
        interchange_trailer.ge_segments = get_ge(ge_content);
        info!("GE segment parsed");
        contents = content_trim("GE", contents);
    } else {
        info!("Warning: Required GE segment not found");
    }
    
    // Process IEA segment (required)
    if contents.contains("IEA") {
        info!("IEA segment found");
        let iea_content = get_segment_contents("IEA", &contents);
        interchange_trailer.iea_segments = get_iea(iea_content);
        info!("IEA segment parsed");
        contents = content_trim("IEA", contents);
    } else {
        info!("Warning: Required IEA segment not found");
    }
    
    info!("Interchange Control Trailer parsed\n");
    (interchange_trailer, contents)
}

pub fn write_interchange_trailer(interchange_trailer: &InterchangeTrailer) -> String {
    let mut contents = String::new();
    
    // Write GE segment with proper values
    let ge = GE {
        number_of_transitions: "1".to_string(),  // Use a reasonable default value
        group_control_number: interchange_trailer.ge_segments.group_control_number.clone(),
    };
    contents.push_str(&write_ge(ge));
    
    // Write IEA segment with proper values
    let iea = IEA {
        number_of_included_group: "1".to_string(),  // Use a reasonable default value
        interchange_control_number: interchange_trailer.iea_segments.interchange_control_number.clone(),
    };
    contents.push_str(&write_iea(iea));
    
    contents
}
