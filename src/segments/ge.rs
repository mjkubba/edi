use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GE {
    pub number_of_transitions: String,
    pub group_control_number: String,
}

pub fn get_ge(ge_content: String) -> GE {
    let ge_parts: Vec<&str> = ge_content.split("*").collect();
    GE {
        number_of_transitions: get_element(&ge_parts, 0),
        group_control_number: get_element(&ge_parts, 1),
    }
}

pub fn write_ge(ge: GE) -> String {
    let mut ge_string = String::new();
    ge_string.push_str("GE*");
    ge_string.push_str(&ge.number_of_transitions);
    ge_string.push_str("*");
    ge_string.push_str(&ge.group_control_number);
    ge_string.push_str("~");
    ge_string
}
