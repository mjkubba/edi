use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

fn opt(s: String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn opt_str(o: &Option<String>) -> &str {
    match o {
        Some(s) => s.as_str(),
        None => "",
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NM1 {
    pub entity_id: String,
    pub entity_type: String,
    pub lastname: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_code_qualifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_number: Option<String>,
}

pub fn get_nm1(nm1_content: String) -> NM1 {
    let nm1_parts: Vec<&str> = nm1_content.split("*").collect();

    NM1 {
        entity_id: get_element(&nm1_parts, 0),
        entity_type: get_element(&nm1_parts, 1),
        lastname: get_element(&nm1_parts, 2),
        firstname: opt(get_element(&nm1_parts, 3)),
        middle_initial: opt(get_element(&nm1_parts, 4)),
        suffix: opt(get_element(&nm1_parts, 5)),
        title: opt(get_element(&nm1_parts, 6)),
        id_code_qualifier: opt(get_element(&nm1_parts, 7)),
        id_code: opt(get_element(&nm1_parts, 8)),
        member_number: opt(get_element(&nm1_parts, 9)),
    }
}

pub fn write_nm1(nm1: NM1) -> String {
    if nm1.entity_id.is_empty() {
        return String::new();
    }

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

    let mut nm1_content = format!(
        "NM1*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}",
        entity_id,
        nm1.entity_type,
        nm1.lastname,
        opt_str(&nm1.firstname),
        opt_str(&nm1.middle_initial),
        opt_str(&nm1.suffix),
        opt_str(&nm1.title),
        opt_str(&nm1.id_code_qualifier),
        opt_str(&nm1.id_code),
        opt_str(&nm1.member_number),
    );
    while nm1_content.ends_with('*') {
        nm1_content.pop();
    }
    nm1_content.push('~');
    nm1_content
}
