use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NM1 {
    pub entity_id: String,
    pub entity_type: String,
    pub lastname: String,
    pub firstname: String,
    pub middle_initial: String,
    pub suffix: String,
    pub title: String,
    pub id_code_qualifier: String,
    pub id_code: String,
    pub member_number: String,
}

pub fn get_nm1(nm1_content: String) -> NM1 {
    let nm1_parts: Vec<&str> = nm1_content.split("*").collect();

    NM1 {
        entity_id: get_element(&nm1_parts, 0),
        entity_type: get_element(&nm1_parts, 1),
        lastname: get_element(&nm1_parts, 2),
        firstname: get_element(&nm1_parts, 3),
        middle_initial: get_element(&nm1_parts, 4),
        suffix: get_element(&nm1_parts, 5),
        title: get_element(&nm1_parts, 6),
        id_code_qualifier: get_element(&nm1_parts, 7),
        id_code: get_element(&nm1_parts, 8),
        member_number: get_element(&nm1_parts, 9),
    }
}

pub fn write_nm1(nm1: NM1) -> String {
    if nm1.entity_id.is_empty() {
        return String::new();
    }

    // Don't include the segment ID in the entity_id field
    let entity_id = if nm1.entity_id == "NM1" {
        match nm1.entity_type.as_str() {
            "PR" => "PR",
            "41" => "41",
            "1P" => "1P",
            "IL" => "IL",
            "QC" => "QC",
            _ => &nm1.entity_id,
        }
    } else {
        &nm1.entity_id
    };

    let mut nm1_content: String = String::new();
    nm1_content.push_str("NM1*");
    nm1_content.push_str(entity_id);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.entity_type);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.lastname);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.firstname);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.middle_initial);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.suffix);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.title);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.id_code_qualifier);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.id_code);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.member_number);
    while nm1_content.ends_with('*') {
        nm1_content.pop();
    }
    nm1_content.push('~');
    nm1_content
}
