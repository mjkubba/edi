#[derive(Debug)]
#[allow(dead_code)]
pub struct BPR {
    payer_id: String,
    payer_name: String,
    payer_address: String,
    payer_city: String,
    payer_state: String,
    payer_zip: String,
}

pub fn get_bpr(bpr_content: &str) -> BPR {
    let bpr_parts: Vec<&str> = bpr_content.split("*").collect();
    BPR {
        payer_id: bpr_parts[0].to_string(),
        payer_name: bpr_parts[1].to_string(),
        payer_address: bpr_parts[2].to_string(),
        payer_city: bpr_parts[3].to_string(),
        payer_state: bpr_parts[4].to_string(),
        payer_zip: bpr_parts[5].to_string(),
    }
}