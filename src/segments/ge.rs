use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GE {
    pub number_of_transitions: String,
    pub group_control_number: String,
}

pub fn get_ge(ge_content: String) -> GE {
    let ge_parts: Vec<&str> = ge_content.split("*").collect();
    GE {
        number_of_transitions: ge_parts[0].to_string(),
        group_control_number: ge_parts[1].to_string(),
    }
}