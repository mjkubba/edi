use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub se_segments: SE,
    pub ge_segments: GE,
    pub iea_segments: IEA,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SE {
    pub number_of_segment: String,
    pub transaction_set_control_number: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GE {
    pub number_of_transitions: String,
    pub group_control_number: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IEA {
    pub number_of_included_group: String,
    pub interchange_control_number: String,
}

pub fn get_interchange_trailer(mut contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();
    
    // Parse SE segment
    if contents.contains("SE") {
        info!("Warning: Required SE segment not found");
        let se_content = get_segment_contents("SE", &contents);
        info!("segment_content: {}", se_content);
        
        let se_parts: Vec<&str> = se_content.split('*').collect();
        
        if se_parts.len() >= 2 {
            interchange_trailer.se_segments = SE {
                number_of_segment: se_parts[1].to_string(),
                transaction_set_control_number: if se_parts.len() > 2 { se_parts[2].to_string() } else { String::new() },
            };
        }
        
        info!("SE segment parsed");
        contents = content_trim("SE", contents);
    }
    
    // Parse GE segment
    if contents.contains("GE") {
        info!("Warning: Required GE segment not found");
        let ge_content = get_segment_contents("GE", &contents);
        info!("segment_content: {}", ge_content);
        
        let ge_parts: Vec<&str> = ge_content.split('*').collect();
        
        if ge_parts.len() >= 2 {
            interchange_trailer.ge_segments = GE {
                number_of_transitions: ge_parts[1].to_string(),
                group_control_number: if ge_parts.len() > 2 { ge_parts[2].to_string() } else { String::new() },
            };
        }
        
        info!("GE segment parsed");
        contents = content_trim("GE", contents);
    }
    
    // Parse IEA segment
    if contents.contains("IEA") {
        info!("Warning: Required IEA segment not found");
        let iea_content = get_segment_contents("IEA", &contents);
        info!("segment_content: {}", iea_content);
        
        let iea_parts: Vec<&str> = iea_content.split('*').collect();
        
        if iea_parts.len() >= 2 {
            interchange_trailer.iea_segments = IEA {
                number_of_included_group: iea_parts[1].to_string(),
                interchange_control_number: if iea_parts.len() > 2 { iea_parts[2].to_string() } else { String::new() },
            };
        }
        
        info!("IEA segment parsed");
        contents = content_trim("IEA", contents);
    }
    
    info!("Interchange Control Trailer parsed\n");
    
    return (interchange_trailer, contents);
}

pub fn write_interchange_trailer(interchange_trailer: InterchangeTrailer) -> String {
    let mut result = String::new();
    
    // Write SE segment
    result.push_str("SE*");
    result.push_str(&interchange_trailer.se_segments.number_of_segment);
    result.push_str("*");
    result.push_str(&interchange_trailer.se_segments.transaction_set_control_number);
    result.push_str("~\n");
    
    // Write GE segment
    result.push_str("GE*");
    result.push_str(&interchange_trailer.ge_segments.number_of_transitions);
    result.push_str("*");
    result.push_str(&interchange_trailer.ge_segments.group_control_number);
    result.push_str("~\n");
    
    // Write IEA segment
    result.push_str("IEA*");
    result.push_str(&interchange_trailer.iea_segments.number_of_included_group);
    result.push_str("*");
    result.push_str(&interchange_trailer.iea_segments.interchange_control_number);
    result.push_str("~\n");
    
    return result;
}
