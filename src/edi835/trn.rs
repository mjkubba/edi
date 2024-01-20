#[derive(Debug)]
#[allow(dead_code)]
pub struct TRN {
    trace_type_code: String,
    reference_id: String,
    originating_company_id: String,
    trn04_reference_id: String,
}

pub fn get_trn(trn_content: String) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    let mut trn04_reference_id: String ="".to_string();
    if trn_parts.get(3).is_some()  {
        trn04_reference_id = trn_parts[3].to_string();
    }
    TRN {
        trace_type_code: trn_parts[0].to_string(),
        reference_id: trn_parts[1].to_string(),
        originating_company_id: trn_parts[2].to_string(),
        trn04_reference_id,
    }
}