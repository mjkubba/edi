use serde::{Serialize, Deserialize};
use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct N4{
    pub payee_city: String,
    pub payee_state: String,
    pub payee_zip: String,
    pub payee_country_code: String,
    pub payee_country_sub_code: String,
}

pub fn get_n4(n4_content: String) -> N4 {
    let n4_parts: Vec<&str> = n4_content.split("*").collect();
    let mut payee_state: String ="".to_string();
    let mut payee_zip: String ="".to_string();
    let mut payee_country_code: String ="".to_string();
    let mut payee_country_sub_code: String ="".to_string();
    if n4_parts.get(1).is_some() {
        payee_state = n4_parts[1].to_string();
    }
    if n4_parts.get(2).is_some() {
        payee_zip = n4_parts[2].to_string();
    }
    if n4_parts.get(3).is_some() {
        payee_country_code = n4_parts[3].to_string();
    } 
    if n4_parts.get(4).is_some() {
        payee_country_sub_code = n4_parts[4].to_string();
    }

    N4 {
        payee_city: n4_parts[0].to_string(),
        payee_state,
        payee_zip,
        payee_country_code,
        payee_country_sub_code,
    }
}

pub fn write_n4(n4:N4) -> String {
    if n4.payee_city.is_empty() {
        return String::new();
    }
    let mut n4_content: String = String::new();
    n4_content.push_str("N4*");
    n4_content.push_str(&n4.payee_city);
    n4_content.push_str(&stiuational_element(n4.payee_state));
    n4_content.push_str(&stiuational_element(n4.payee_zip));
    n4_content.push_str(&stiuational_element(n4.payee_country_code));
    // n4_content.push_str("*");
    // n4_content.push_str(&n4.payee_state);
    // n4_content.push_str("*");
    // n4_content.push_str(&n4.payee_zip);
    // n4_content.push_str("*");
    // n4_content.push_str(&n4.payee_country_code);
    n4_content.push_str("~");
    n4_content
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_n4() {
        let n4_content = "BOSTON*MA*02111*US*US".to_string();
        let n4 = get_n4(n4_content);
        assert_eq!(n4.payee_city, "BOSTON");
        assert_eq!(n4.payee_state, "MA");
        assert_eq!(n4.payee_zip, "02111");
        assert_eq!(n4.payee_country_code, "US");
        assert_eq!(n4.payee_country_sub_code, "US");
    }
}