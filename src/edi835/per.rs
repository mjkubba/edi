#[derive(Debug)]
#[allow(dead_code)]
pub struct PER {
    contact_function_code: String,
    contact_name: String,
    per03_contact_number_qualifier: String,
    per04_contact_number: String,
    per05_contact_number_qualifier: String,
    per06_contact_number: String,
    per07_contact_number_qualifier: String,
    per08_contact_number: String,
}

pub fn get_per(per_content: String) -> PER {
    let per_parts: Vec<&str> = per_content.split("*").collect();
    let mut contact_name: String ="".to_string();
    let mut per03_contact_number_qualifier: String ="".to_string();
    let mut per04_contact_number: String ="".to_string();
    let mut per05_contact_number_qualifier: String ="".to_string();
    let mut per06_contact_number: String ="".to_string();
    let mut per07_contact_number_qualifier: String ="".to_string();
    let mut per08_contact_number: String ="".to_string();
    if per_parts.get(1).is_some() {
        contact_name = per_parts[1].to_string();
    }
    if per_parts.get(2).is_some() {
        per03_contact_number_qualifier = per_parts[2].to_string();
    }
    if per_parts.get(3).is_some() {
        per04_contact_number = per_parts[3].to_string();
    }
    if per_parts.get(4).is_some() {
        per05_contact_number_qualifier = per_parts[4].to_string();
    }
    if per_parts.get(5).is_some() {
        per06_contact_number = per_parts[5].to_string();
    }
    if per_parts.get(6).is_some() {
        per07_contact_number_qualifier = per_parts[6].to_string();
    }
    if per_parts.get(7).is_some() {
        per08_contact_number = per_parts[7].to_string();
    }

    PER {
        contact_function_code: per_parts[0].to_string(),
        contact_name,
        per03_contact_number_qualifier,
        per04_contact_number,
        per05_contact_number_qualifier,
        per06_contact_number,
        per07_contact_number_qualifier,
        per08_contact_number,        
    }
}