#[derive(Debug)]
#[allow(dead_code)]
pub struct N1{
    payer_id_code: String,
    payee_name: String,
    // payee_identification_code_qualifier: String,
    // payee_identification_code: String,
}

pub fn get_n1(n1_content: &str) -> N1 {
    let n1_parts: Vec<&str> = n1_content.split("*").collect();
    N1 {
        payer_id_code: n1_parts[0].to_string(),
        payee_name: n1_parts[1].to_string(),
        // payee_identification_code_qualifier: n1_parts[2].to_string(),
        // payee_identification_code: n1_parts[3].to_string(),
    }
}