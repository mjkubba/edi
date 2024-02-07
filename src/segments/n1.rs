use serde::{Serialize, Deserialize};
use crate::helper::helper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct N1{
    pub payer_id_code: String,
    pub payee_name: String,
    pub payee_identification_code_qualifier: String,
    pub payee_identification_code: String,
}

pub fn get_n1(n1_content: String) -> N1 {
    let n1_parts: Vec<&str> = n1_content.split("*").collect();
    let mut payee_identification_code_qualifier: String ="".to_string();
    let mut payee_identification_code: String ="".to_string();
    if n1_parts.get(2).is_some() {
        payee_identification_code_qualifier = n1_parts[2].to_string();
    }
    if n1_parts.get(3).is_some() {
        payee_identification_code = n1_parts[3].to_string();
    }
    N1 {
        payer_id_code: n1_parts[0].to_string(),
        payee_name: n1_parts[1].to_string(),
        payee_identification_code_qualifier,
        payee_identification_code,
    }
}

pub fn write_n1(n1:N1) -> String {
    let mut n1_content: String = String::new();
    n1_content.push_str("N1*");
    n1_content.push_str(&n1.payer_id_code);
    n1_content.push_str("*");
    n1_content.push_str(&n1.payee_name);
    n1_content.push_str(&stiuational_element(n1.payee_identification_code_qualifier));
    n1_content.push_str(&stiuational_element(n1.payee_identification_code));

    // n1_content.push_str("*");
    // n1_content.push_str(&n1.payee_identification_code_qualifier);
    // n1_content.push_str("*");
    // n1_content.push_str(&n1.payee_identification_code);
    n1_content.push_str("~");
    n1_content
}