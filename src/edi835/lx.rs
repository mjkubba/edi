#[derive(Debug)]
#[allow(dead_code)]
pub struct LX {
    claim_sequence_number: String,
}

pub fn get_lx(lx_content: String) -> LX {
    let lx_parts: Vec<&str> = lx_content.split("*").collect();
    LX {
        claim_sequence_number: lx_parts[0].to_string(),
    }
}