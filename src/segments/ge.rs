#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct GE {
    number_of_transitions: String,
    group_control_number: String,
}

pub fn get_ge(ge_content: String) -> GE {
    let ge_parts: Vec<&str> = ge_content.split("*").collect();
    GE {
        number_of_transitions: ge_parts[0].to_string(),
        group_control_number: ge_parts[1].to_string(),
    }
}