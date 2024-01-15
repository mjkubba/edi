#[derive(Debug)]
#[allow(dead_code)]
pub struct TRN {
    trace_type_code: String,
    reference_id: String,
    originating_company_id: String,
}

pub fn get_trn(trn_content: &str) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    TRN {
        trace_type_code: trn_parts[0].to_string(),
        reference_id: trn_parts[1].to_string(),
        originating_company_id: trn_parts[2].to_string(),
    }
}