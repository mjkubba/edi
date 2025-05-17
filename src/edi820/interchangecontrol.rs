use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa_segments: ISA,
    pub gs_segments: GS,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISA {
    pub information_qualifier: String,
    pub authorization_information: String,
    pub security_information_qualifier: String,
    pub security_information: String,
    pub sender_id_qualifier: String,
    pub sender_id: String,
    pub receiver_id_qualifier: String,
    pub receiver_id: String,
    pub date: String,
    pub time: String,
    pub control_number_identifier: String,
    pub control_version_number: String,
    pub control_number: String,
    pub ack_indicator: String,
    pub usage_indicator: String,
    pub component_element_separator: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GS {
    pub functional_id_code: String,
    pub app_sender_id: String,
    pub app_receiver_id: String,
    pub date: String,
    pub time: String,
    pub group_control_number: String,
    pub responsible_agency: String,
    pub version_number: String,
}

pub fn get_interchange_header(mut contents: String) -> (InterchangeHeader, String) {
    let mut interchange_header = InterchangeHeader::default();
    
    // Parse ISA segment
    if contents.contains("ISA") {
        info!("ISA segment found, ");
        let isa_content = get_segment_contents("ISA", &contents);
        info!("segment_content: {}", isa_content);
        
        let isa_parts: Vec<&str> = isa_content.split('*').collect();
        
        if isa_parts.len() >= 16 {
            interchange_header.isa_segments = ISA {
                information_qualifier: isa_parts[1].to_string(),
                authorization_information: isa_parts[2].to_string(),
                security_information_qualifier: isa_parts[3].to_string(),
                security_information: isa_parts[4].to_string(),
                sender_id_qualifier: isa_parts[5].to_string(),
                sender_id: isa_parts[6].to_string(),
                receiver_id_qualifier: isa_parts[7].to_string(),
                receiver_id: isa_parts[8].to_string(),
                date: isa_parts[9].to_string(),
                time: isa_parts[10].to_string(),
                control_number_identifier: isa_parts[11].to_string(),
                control_version_number: isa_parts[12].to_string(),
                control_number: isa_parts[13].to_string(),
                ack_indicator: isa_parts[14].to_string(),
                usage_indicator: isa_parts[15].to_string(),
                component_element_separator: if isa_parts.len() > 16 { isa_parts[16].to_string() } else { String::new() },
            };
        }
        
        info!("ISA segment parsed");
        contents = content_trim("ISA", contents);
    }
    
    // Parse GS segment
    if contents.contains("GS") {
        info!("GS segment found, ");
        let gs_content = get_segment_contents("GS", &contents);
        info!("segment_content: {}", gs_content);
        
        let gs_parts: Vec<&str> = gs_content.split('*').collect();
        
        if gs_parts.len() >= 8 {
            interchange_header.gs_segments = GS {
                functional_id_code: gs_parts[1].to_string(),
                app_sender_id: gs_parts[2].to_string(),
                app_receiver_id: gs_parts[3].to_string(),
                date: gs_parts[4].to_string(),
                time: gs_parts[5].to_string(),
                group_control_number: gs_parts[6].to_string(),
                responsible_agency: gs_parts[7].to_string(),
                version_number: if gs_parts.len() > 8 { gs_parts[8].to_string() } else { String::new() },
            };
        }
        
        info!("GS segment parsed");
        contents = content_trim("GS", contents);
    }
    
    info!("Interchange Control parsed\n");
    
    return (interchange_header, contents);
}

pub fn write_interchange_control(interchange_header: InterchangeHeader) -> String {
    let mut result = String::new();
    
    // Write ISA segment
    result.push_str("ISA*");
    result.push_str(&interchange_header.isa_segments.information_qualifier);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.authorization_information);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.security_information_qualifier);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.security_information);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.sender_id_qualifier);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.sender_id);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.receiver_id_qualifier);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.receiver_id);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.date);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.time);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.control_number_identifier);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.control_version_number);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.control_number);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.ack_indicator);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.usage_indicator);
    result.push_str("*");
    result.push_str(&interchange_header.isa_segments.component_element_separator);
    result.push_str("~\n");
    
    // Write GS segment
    result.push_str("GS*");
    result.push_str(&interchange_header.gs_segments.functional_id_code);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.app_sender_id);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.app_receiver_id);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.date);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.time);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.group_control_number);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.responsible_agency);
    result.push_str("*");
    result.push_str(&interchange_header.gs_segments.version_number);
    result.push_str("~\n");
    
    return result;
}
