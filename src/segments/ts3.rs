use serde::{Serialize, Deserialize};

// EDI 835 TS3 - PROVIDER SUMMARY INFORMATION
#[derive(Debug, Default, PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TS3 {
    pub ts301_provider_identifier : String,
    pub ts302_facility_type_code : String,
    pub ts303_fiscal_period_date : String,
    pub ts304_total_claim_count : String,
    pub ts305_total_claim_charge_amount : String,
    pub ts313_total_msp_payer_amount : String,
    pub ts315_total_non_lab_charge_amount : String,
    pub ts317_total_hcpcs_reported_charge_amount : String,
    pub ts318_total_hcpcs_payable_amount : String,
    pub ts320_total_professional_component_amount : String,
    pub ts321_total_msp_patient_liability_met_amount : String,
    pub ts322_total_patient_reimbursement_amount : String,  
    pub ts323_total_pip_claim_count : String,
    pub ts324_total_pip_adjustment_amount : String,
    
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

    if ts3_parts.get(5).is_some() {
        ts313_total_msp_payer_amount = ts3_parts[5].to_string();
    }
    if ts3_parts.get(6).is_some() {
        ts315_total_non_lab_charge_amount = ts3_parts[6].to_string();
    }
    if ts3_parts.get(7).is_some() {
        ts317_total_hcpcs_reported_charge_amount = ts3_parts[7].to_string();
    }
    if ts3_parts.get(8).is_some() {
        ts318_total_hcpcs_payable_amount = ts3_parts[8].to_string();
    }
    if ts3_parts.get(9).is_some() {
        ts320_total_professional_component_amount = ts3_parts[9].to_string();
    }
    if ts3_parts.get(10).is_some() {
        ts321_total_msp_patient_liability_met_amount = ts3_parts[10].to_string();
    }
    if ts3_parts.get(11).is_some() {
        ts322_total_patient_reimbursement_amount = ts3_parts[11].to_string();
    }
    if ts3_parts.get(12).is_some() {
        ts323_total_pip_claim_count = ts3_parts[12].to_string();
    }
    if ts3_parts.get(13).is_some() {
        ts324_total_pip_adjustment_amount = ts3_parts[13].to_string();
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



pub fn write_ts3(ts3:TS3) -> String {
    let mut ts3_content: String = String::new();
    ts3_content.push_str("TS3*");
    ts3_content.push_str(&ts3.ts301_provider_identifier);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts302_facility_type_code);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts303_fiscal_period_date);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts304_total_claim_count);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts305_total_claim_charge_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts313_total_msp_payer_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts315_total_non_lab_charge_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts317_total_hcpcs_reported_charge_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts318_total_hcpcs_payable_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts320_total_professional_component_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts321_total_msp_patient_liability_met_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts322_total_patient_reimbursement_amount);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts323_total_pip_claim_count);
    ts3_content.push_str("*");
    ts3_content.push_str(&ts3.ts324_total_pip_adjustment_amount);
    ts3_content.push_str("~");
    ts3_content
    
}


// unit test


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_ts3() {
        let ts3_content = "1*1*202206*1*2*3*4*5*6*7*8*9*10*11".to_string();
        let ts3 = get_ts3(ts3_content);
        assert_eq!(ts3.ts301_provider_identifier, "1");
        assert_eq!(ts3.ts302_facility_type_code, "1");
        assert_eq!(ts3.ts303_fiscal_period_date, "202206");
        assert_eq!(ts3.ts304_total_claim_count, "1");
        assert_eq!(ts3.ts305_total_claim_charge_amount, "2");
        assert_eq!(ts3.ts313_total_msp_payer_amount, "3");
        assert_eq!(ts3.ts315_total_non_lab_charge_amount, "4");
        assert_eq!(ts3.ts317_total_hcpcs_reported_charge_amount, "5");
        assert_eq!(ts3.ts318_total_hcpcs_payable_amount, "6");
        assert_eq!(ts3.ts320_total_professional_component_amount, "7");
        assert_eq!(ts3.ts321_total_msp_patient_liability_met_amount, "8");
        assert_eq!(ts3.ts322_total_patient_reimbursement_amount, "9");
        assert_eq!(ts3.ts323_total_pip_claim_count, "10");
        assert_eq!(ts3.ts324_total_pip_adjustment_amount, "11");
    }
}