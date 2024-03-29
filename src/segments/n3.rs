use serde::{Serialize, Deserialize};
use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct N3{
    pub payee_address: String,
    pub payee_address2: String,
}

pub fn get_n3(n3_content: String) -> N3 {
    let n3_parts: Vec<&str> = n3_content.split("*").collect();
    if n3_parts.len() == 1 {
        N3 {
            payee_address: n3_parts[0].to_string(),
            payee_address2: "".to_string(),
        }
    } else {
        N3 {
            payee_address: n3_parts[0].to_string(),
            payee_address2: n3_parts[1].to_string(),
        }
    }
}

pub fn write_n3(n3:N3) -> String {
    if n3.payee_address.is_empty() {
        return String::new();
    }
    let mut n3_content: String = String::new();
    n3_content.push_str("N3*");
    n3_content.push_str(&n3.payee_address);
    n3_content.push_str(&stiuational_element(n3.payee_address2));

    // n3_content.push_str("*");
    // n3_content.push_str(&n3.payee_address2);
    n3_content.push_str("~");
    n3_content
}