use serde::{Serialize, Deserialize};
use crate::helper::helper::stiuational_element;

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
    let mut per02_contact_name: String ="".to_string();
    let mut per03_contact_number_qualifier: String ="".to_string();
    let mut per04_contact_number: String ="".to_string();
    let mut per05_contact_number_qualifier: String ="".to_string();
    let mut per06_contact_number: String ="".to_string();
    let mut per07_contact_number_qualifier: String ="".to_string();
    let mut per08_contact_number: String ="".to_string();
    if per_parts.get(1).is_some() {
        per02_contact_name = per_parts[1].to_string();
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
        per01_contact_function_code: per_parts[0].to_string(),
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
    // per_content.push_str("*");
    // per_content.push_str(&per.per02_contact_name);
    // per_content.push_str("*");
    // per_content.push_str(&per.per03_contact_number_qualifier);
    // per_content.push_str("*");
    // per_content.push_str(&per.per04_contact_number);
    // per_content.push_str("*");
    // per_content.push_str(&per.per05_contact_number_qualifier);
    // per_content.push_str("*");
    // per_content.push_str(&per.per06_contact_number);
    // per_content.push_str("*");
    // per_content.push_str(&per.per07_contact_number_qualifier);
    // per_content.push_str("*");
    // per_content.push_str(&per.per08_contact_number);
    per_content.push_str("~");
    per_content
}


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_per() {
        let per_content = "PER*PER02*PER03*PER04*PER05*PER06*PER07*PER08";
        let per = get_per(per_content.to_string());
        assert_eq!(per.per01_contact_function_code, "PER");
        assert_eq!(per.per02_contact_name, "PER02");
        assert_eq!(per.per03_contact_number_qualifier, "PER03");
        assert_eq!(per.per04_contact_number, "PER04");
        assert_eq!(per.per05_contact_number_qualifier, "PER05");
        assert_eq!(per.per06_contact_number, "PER06");
        assert_eq!(per.per07_contact_number_qualifier, "PER07");
        assert_eq!(per.per08_contact_number, "PER08");
    }
}