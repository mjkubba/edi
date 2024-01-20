#[derive(Debug)]
#[allow(dead_code)]
pub struct N1{
    payer_id_code: String,
    payee_name: String,
    payee_identification_code_qualifier: String,
    payee_identification_code: String,
}

pub fn get_n1(n1_content: String) -> N1 {
    let n1_parts: Vec<&str> = n1_content.split("*").collect();
    let mut payee_identification_code_qualifier: String ="".to_string();
    let mut payee_identification_code: String ="".to_string();
    if n1_parts.get(2).is_some() {
        payee_identification_code_qualifier = n1_parts[2].to_string();
    }
    if n1_parts.get(3).is_some() {
        payee_identification_code = n1_parts[3].to_string();
    }
    N1 {
        payer_id_code: n1_parts[0].to_string(),
        payee_name: n1_parts[1].to_string(),
        payee_identification_code_qualifier,
        payee_identification_code,
    }
}