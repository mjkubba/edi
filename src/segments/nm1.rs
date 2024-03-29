use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NM1{
    pub entity_id: String,
    pub entity_type: String,
    pub lastname: String,
    pub firstname: String,
    pub middle_initial: String,
    pub suffix: String,
    pub title: String,
    pub id_code: String,
    pub member_number: String,
}

pub fn get_nm1(nm1_content: String) -> NM1 {
    let nm1_parts: Vec<&str> = nm1_content.split("*").collect();
    NM1 {
        entity_id: nm1_parts[0].to_string(),
        entity_type: nm1_parts[1].to_string(),
        lastname: nm1_parts[2].to_string(),
        firstname: nm1_parts[3].to_string(),
        middle_initial: nm1_parts[4].to_string(),
        suffix: nm1_parts[5].to_string(),
        title: nm1_parts[6].to_string(),
        id_code: nm1_parts[7].to_string(),
        member_number: nm1_parts[8].to_string(),
    }
}

pub fn write_nm1(nm1:NM1) -> String {
    if nm1.entity_id.is_empty() {
        return String::new();
    }
    let mut nm1_content: String = String::new();
    nm1_content.push_str("NM1*");
    nm1_content.push_str(&nm1.entity_id);
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
    nm1_content.push_str(&nm1.id_code);
    nm1_content.push_str("*");
    nm1_content.push_str(&nm1.member_number);
    nm1_content.push_str("~");
    nm1_content
    
}
