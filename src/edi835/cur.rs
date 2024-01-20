// EDI 835 CUR segment
#[derive(Debug)]
#[allow(dead_code)]

pub struct CUR {
    identity_identifier_code: String,
    currency_code: String,
}

pub fn get_cur(cur_content: String) -> CUR {
    let cur_parts: Vec<&str> = cur_content.split("*").collect();
    CUR {
        identity_identifier_code: cur_parts[0].to_string(),
        currency_code: cur_parts[1].to_string(),
    }
}