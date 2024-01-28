use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LX {
    pub lx01_claim_sequence_number: String,
}

pub fn get_lx(lx_content: String) -> LX {
    let lx_parts: Vec<&str> = lx_content.split("*").collect();
    LX {
        lx01_claim_sequence_number: lx_parts[0].to_string(),
    }
}