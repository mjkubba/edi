use serde::{Serialize, Deserialize};
use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PER {
    pub per01_contact_function_code: String,
    pub per02_contact_name: String,
    pub per03_contact_number_qualifier: String,
    pub per04_contact_number: String,
    pub per05_contact_number_qualifier: String,
    pub per06_contact_number: String,
    pub per07_contact_number_qualifier: String,
    pub per08_contact_number: String,
}

pub fn get_per(per_content: String) -> PER {
    let per_parts: Vec<&str> = per_content.split("*").collect();
    
    // Ensure we have at least one part (the segment ID)
    if per_parts.is_empty() {
        return PER::default();
    }
    
    // Extract the actual function code (skip the segment ID)
    let per01_contact_function_code = if per_parts.len() > 1 { per_parts[1].to_string() } else { String::new() };
    
    // Extract remaining fields with bounds checking
    let per02_contact_name = if per_parts.len() > 2 { per_parts[2].to_string() } else { String::new() };
    let per03_contact_number_qualifier = if per_parts.len() > 3 { per_parts[3].to_string() } else { String::new() };
    let per04_contact_number = if per_parts.len() > 4 { per_parts[4].to_string() } else { String::new() };
    let per05_contact_number_qualifier = if per_parts.len() > 5 { per_parts[5].to_string() } else { String::new() };
    let per06_contact_number = if per_parts.len() > 6 { per_parts[6].to_string() } else { String::new() };
    let per07_contact_number_qualifier = if per_parts.len() > 7 { per_parts[7].to_string() } else { String::new() };
    let per08_contact_number = if per_parts.len() > 8 { per_parts[8].to_string() } else { String::new() };

    PER {
        per01_contact_function_code,
        per02_contact_name,
        per03_contact_number_qualifier,
        per04_contact_number,
        per05_contact_number_qualifier,
        per06_contact_number,
        per07_contact_number_qualifier,
        per08_contact_number,        
    }
}

pub fn write_per(per:PER) -> String {
    if per.per01_contact_function_code.is_empty() {
        return String::new();
    }
    let mut per_content: String = String::new();
    per_content.push_str("PER*");
    per_content.push_str(&per.per01_contact_function_code);
    per_content.push_str(&stiuational_element(per.per02_contact_name));
    per_content.push_str(&stiuational_element(per.per03_contact_number_qualifier));
    per_content.push_str(&stiuational_element(per.per04_contact_number));
    per_content.push_str(&stiuational_element(per.per05_contact_number_qualifier));
    per_content.push_str(&stiuational_element(per.per06_contact_number));
    per_content.push_str(&stiuational_element(per.per07_contact_number_qualifier));
    per_content.push_str(&stiuational_element(per.per08_contact_number));
    per_content.push_str("~");
    per_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_per() {
        let per_content = "PER*IC*CUSTOMER SERVICE*TE*8005557722".to_string();
        let per = get_per(per_content);
        
        assert_eq!(per.per01_contact_function_code, "IC");
        assert_eq!(per.per02_contact_name, "CUSTOMER SERVICE");
        assert_eq!(per.per03_contact_number_qualifier, "TE");
        assert_eq!(per.per04_contact_number, "8005557722");
    }
    
    #[test]
    fn test_write_per() {
        let per = PER {
            per01_contact_function_code: "IC".to_string(),
            per02_contact_name: "CUSTOMER SERVICE".to_string(),
            per03_contact_number_qualifier: "TE".to_string(),
            per04_contact_number: "8005557722".to_string(),
            per05_contact_number_qualifier: "".to_string(),
            per06_contact_number: "".to_string(),
            per07_contact_number_qualifier: "".to_string(),
            per08_contact_number: "".to_string(),
        };
        
        let per_content = write_per(per);
        assert_eq!(per_content, "PER*IC*CUSTOMER SERVICE*TE*8005557722~");
    }
}
