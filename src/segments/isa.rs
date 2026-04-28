use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

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

// function to get the ISA struct
pub fn get_isa(isa_content: String) -> ISA {
    let isa_parts: Vec<&str> = isa_content.split("*").collect();
    ISA {
        information_qualifier: get_element(&isa_parts, 0),
        authorization_information: get_element(&isa_parts, 1),
        security_information_qualifier: get_element(&isa_parts, 2),
        security_information: get_element(&isa_parts, 3),
        sender_id_qualifier: get_element(&isa_parts, 4),
        sender_id: get_element(&isa_parts, 5),
        receiver_id_qualifier: get_element(&isa_parts, 6),
        receiver_id: get_element(&isa_parts, 7),
        date: get_element(&isa_parts, 8),
        time: get_element(&isa_parts, 9),
        control_number_identifier: get_element(&isa_parts, 10),
        control_version_number: get_element(&isa_parts, 11),
        control_number: get_element(&isa_parts, 12),
        ack_indicator: get_element(&isa_parts, 13),
        usage_indicator: get_element(&isa_parts, 14),
        component_element_separator: get_element(&isa_parts, 15),
    }
}

pub fn write_isa(isa: ISA) -> String {
    let mut isa_content = String::new();
    isa_content.push_str("ISA*");
    isa_content.push_str(&isa.information_qualifier);
    isa_content.push_str("*");
    isa_content.push_str(&isa.authorization_information);
    isa_content.push_str("*");
    isa_content.push_str(&isa.security_information_qualifier);
    isa_content.push_str("*");
    isa_content.push_str(&isa.security_information);
    isa_content.push_str("*");
    isa_content.push_str(&isa.sender_id_qualifier);
    isa_content.push_str("*");
    isa_content.push_str(&isa.sender_id);
    isa_content.push_str("*");
    isa_content.push_str(&isa.receiver_id_qualifier);
    isa_content.push_str("*");
    isa_content.push_str(&isa.receiver_id);
    isa_content.push_str("*");
    isa_content.push_str(&isa.date);
    isa_content.push_str("*");
    isa_content.push_str(&isa.time);
    isa_content.push_str("*");
    isa_content.push_str(&isa.control_number_identifier);
    isa_content.push_str("*");
    isa_content.push_str(&isa.control_version_number);
    isa_content.push_str("*");
    isa_content.push_str(&isa.control_number);
    isa_content.push_str("*");
    isa_content.push_str(&isa.ack_indicator);
    isa_content.push_str("*");
    isa_content.push_str(&isa.usage_indicator);
    isa_content.push_str("*");
    isa_content.push_str(&isa.component_element_separator);
    isa_content.push_str("~");
    isa_content
}
