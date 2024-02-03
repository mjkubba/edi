use serde::{Serialize, Deserialize};
// EDI 835 SE segment
#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SE{
    pub number_of_segment: String,
    pub transaction_set_control_number: String,
}

pub fn get_se(se_content: String) -> SE {
    let se_parts: Vec<&str> = se_content.split("*").collect();
    SE {
        number_of_segment: se_parts[0].to_string(),
        transaction_set_control_number: se_parts[1].to_string(),
    }
}

pub fn write_se(se: SE) -> String {
    let mut se_string = String::new();
    se_string.push_str("SE*");
    se_string.push_str(&se.number_of_segment);
    se_string.push_str("*");
    se_string.push_str(&se.transaction_set_control_number);
    se_string.push_str("~");
    se_string
}