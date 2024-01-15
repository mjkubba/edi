#[derive(Debug)]
#[allow(dead_code)]
pub struct CLP{
    patient_control_number: String,
    claim_code: String,
    claim_status: String,
    total_claim_charge_amount: String,
    total_claim_payment_amount: String,
    patient_responsibility_amount: String,
    claim_filing_indicator_code: String,
    payer_claim_control_number: String,
    facility_type_code: String,
    claim_frequency_code: String,
    // patient_status_code: String,
    diagnosis_related_group: String,
    drg_weight: String,
    // percent_discharge_fraction: String,
}


pub fn get_clp(bpr_content: &str) -> CLP {
    let clp_parts: Vec<&str> = bpr_content.split("*").collect();
    CLP {
        patient_control_number: clp_parts[0].to_string(),
        claim_code: clp_parts[1].to_string(),
        claim_status: clp_parts[2].to_string(),
        total_claim_charge_amount: clp_parts[3].to_string(),
        total_claim_payment_amount: clp_parts[4].to_string(),
        patient_responsibility_amount: clp_parts[5].to_string(),
        claim_filing_indicator_code: clp_parts[6].to_string(),
        payer_claim_control_number: clp_parts[7].to_string(),
        facility_type_code: clp_parts[8].to_string(),
        claim_frequency_code: clp_parts[9].to_string(),
        // patient_status_code: clp_parts[10].to_string(),
        diagnosis_related_group: clp_parts[11].to_string(),
        drg_weight: clp_parts[12].to_string(),
        // percent_discharge_fraction: clp_parts[13].to_string(),
    }
}