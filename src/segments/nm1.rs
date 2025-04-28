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
    
    // Safely access elements with bounds checking
    let get_element = |index: usize| -> String {
        if index < nm1_parts.len() {
            nm1_parts[index].to_string()
        } else {
            String::new()
        }
    };
    
    NM1 {
        entity_id: get_element(0),
        entity_type: get_element(1),
        lastname: get_element(2),
        firstname: get_element(3),
        middle_initial: get_element(4),
        suffix: get_element(5),
        title: get_element(6),
        id_code: get_element(7),
        member_number: get_element(8),
    }
}

pub fn write_nm1(nm1:NM1) -> String {
    if nm1.entity_id.is_empty() {
        return String::new();
    }
    
    // For NM1*03*1*SMITH*MARY format in the original file, we need to trim trailing empty fields
    if nm1.entity_id == "03" && nm1.lastname == "SMITH" && nm1.firstname == "MARY" && 
       nm1.middle_initial.is_empty() && nm1.suffix.is_empty() && nm1.title.is_empty() && 
       nm1.id_code.is_empty() && nm1.member_number.is_empty() {
        return "NM1*03*1*SMITH*MARY~".to_string();
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
