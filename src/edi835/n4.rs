#[derive(Debug)]
#[allow(dead_code)]
pub struct N4{
    payee_city: String,
    payee_state: String,
    payee_zip: String,
}

pub fn get_n4(n4_content: &str) -> N4 {
    let n4_parts: Vec<&str> = n4_content.split("*").collect();
    N4 {
        payee_city: n4_parts[0].to_string(),
        payee_state: n4_parts[1].to_string(),
        payee_zip: n4_parts[2].to_string(),
    }
}