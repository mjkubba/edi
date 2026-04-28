use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct N3 {
    pub payee_address: String,
    pub payee_address2: String,
}

pub fn get_n3(n3_content: String) -> N3 {
    let n3_parts: Vec<&str> = n3_content.split("*").collect();
    if n3_parts.len() == 1 {
        N3 {
            payee_address: get_element(&n3_parts, 0),
            payee_address2: "".to_string(),
        }
    } else {
        N3 {
            payee_address: get_element(&n3_parts, 0),
            payee_address2: get_element(&n3_parts, 1),
        }
    }
}

pub fn write_n3(n3: N3) -> String {
    if n3.payee_address.is_empty() {
        return String::new();
    }
    build_segment(&["N3", &n3.payee_address, &n3.payee_address2])
}
