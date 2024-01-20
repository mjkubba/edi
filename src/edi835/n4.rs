#[derive(Debug)]
#[allow(dead_code)]
pub struct N4{
    payee_city: String,
    payee_state: String,
    payee_zip: String,
    payee_country_code: String,
    payee_country_sub_code: String,
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