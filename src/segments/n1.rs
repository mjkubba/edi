use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct N1 {
    pub payer_id_code: String,
    pub payee_name: String,
    pub payee_identification_code_qualifier: String,
    pub payee_identification_code: String,
}

pub fn get_n1(n1_content: String) -> N1 {
    let n1_parts: Vec<&str> = n1_content.split("*").collect();
    let mut payee_identification_code_qualifier: String = "".to_string();
    let mut payee_identification_code: String = "".to_string();
    if n1_parts.get(2).is_some() {
        payee_identification_code_qualifier = get_element(&n1_parts, 2);
    }
    if n1_parts.get(3).is_some() {
        payee_identification_code = get_element(&n1_parts, 3);
    }
    N1 {
        payer_id_code: get_element(&n1_parts, 0),
        payee_name: get_element(&n1_parts, 1),
        payee_identification_code_qualifier,
        payee_identification_code,
    }
}

pub fn write_n1(n1: N1) -> String {
    build_segment(&[
        "N1",
        &n1.payer_id_code,
        &n1.payee_name,
        &n1.payee_identification_code_qualifier,
        &n1.payee_identification_code,
    ])
}
