// EDI 835 TS3 - PROVIDER SUMMARY INFORMATION
#[derive(Debug, Default, PartialEq)]
#[allow(dead_code)]
pub struct TS3 {
    ts301_provider_identifier : String,
    ts302_facility_type_code : String,
    ts303_fiscal_period_date : String,
    ts304_total_claim_count : String,
    ts305_total_claim_charge_amount : String,
    ts313_total_msp_payer_amount : String,
    ts315_total_non_lab_charge_amount : String,
    ts317_total_hcpcs_reported_charge_amount : String,
    ts318_total_hcpcs_payable_amount : String,
    ts320_total_professional_component_amount : String,
    ts321_total_msp_patient_liability_met_amount : String,
    ts322_total_patient_reimbursement_amount : String,  
    ts323_total_pip_claim_count : String,
    ts324_total_pip_adjustment_amount : String,
    
}



pub fn get_ts3(ts3_content: String) -> TS3 {
    let ts3_parts: Vec<&str> = ts3_content.split("*").collect();
    let mut ts313_total_msp_payer_amount: String ="".to_string();
    let mut ts315_total_non_lab_charge_amount: String ="".to_string();
    let mut ts317_total_hcpcs_reported_charge_amount: String ="".to_string();
    let mut ts318_total_hcpcs_payable_amount: String ="".to_string();
    let mut ts320_total_professional_component_amount: String ="".to_string();
    let mut ts321_total_msp_patient_liability_met_amount: String ="".to_string();
    let mut ts322_total_patient_reimbursement_amount: String ="".to_string();
    let mut ts323_total_pip_claim_count: String ="".to_string();
    let mut ts324_total_pip_adjustment_amount: String ="".to_string();

    if ts3_parts.get(12).is_some() {
        ts313_total_msp_payer_amount = ts3_parts[12].to_string();
    }
    if ts3_parts.get(14).is_some() {
        ts315_total_non_lab_charge_amount = ts3_parts[14].to_string();
    }
    if ts3_parts.get(16).is_some() {
        ts317_total_hcpcs_reported_charge_amount = ts3_parts[16].to_string();
    }
    if ts3_parts.get(17).is_some() {
        ts318_total_hcpcs_payable_amount = ts3_parts[17].to_string();
    }
    if ts3_parts.get(19).is_some() {
        ts320_total_professional_component_amount = ts3_parts[19].to_string();
    }
    if ts3_parts.get(20).is_some() {
        ts321_total_msp_patient_liability_met_amount = ts3_parts[20].to_string();
    }
    if ts3_parts.get(21).is_some() {
        ts322_total_patient_reimbursement_amount = ts3_parts[21].to_string();
    }
    if ts3_parts.get(22).is_some() {
        ts323_total_pip_claim_count = ts3_parts[22].to_string();
    }
    if ts3_parts.get(23).is_some() {
        ts324_total_pip_adjustment_amount = ts3_parts[23].to_string();
    }
    TS3 {
        ts301_provider_identifier: ts3_parts.get(0).unwrap().to_string(),
        ts302_facility_type_code: ts3_parts.get(1).unwrap().to_string(),
        ts303_fiscal_period_date: ts3_parts.get(2).unwrap().to_string(),
        ts304_total_claim_count: ts3_parts.get(3).unwrap().to_string(),
        ts305_total_claim_charge_amount: ts3_parts.get(4).unwrap().to_string(),
        ts313_total_msp_payer_amount,
        ts315_total_non_lab_charge_amount,
        ts317_total_hcpcs_reported_charge_amount,
        ts318_total_hcpcs_payable_amount,
        ts320_total_professional_component_amount,
        ts321_total_msp_patient_liability_met_amount,
        ts322_total_patient_reimbursement_amount,
        ts323_total_pip_claim_count,
        ts324_total_pip_adjustment_amount,
        
    }

}