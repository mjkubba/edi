use serde::{Serialize, Deserialize};
use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CLP{
    pub clp01_patient_control_number: String,
    pub clp02_claim_status_code: String,
    pub clp03_total_claim_charge_amount: String,
    pub clp04_total_claim_payment_amount: String,
    pub clp05_patient_responsibility_amount: String,
    pub clp06_claim_filing_indicator_code: String,
    pub clp07_payer_claim_control_number: String,
    pub clp08_facility_type_code: String,
    pub clp09_claim_frequency_code: String,
    // patient_status_code: String,
    pub clp11_diagnosis_related_group: String,
    pub clp12_drg_weight: String,
    pub clp13_percent_discharge_fraction: String,
}

// CLP05 is optional, when patient’s responsibility is greater than zero
// CLP08, CLP09 is optional, when info was recieved. if 08 exist 09 must be there
// CLP11 is optional when claim is adjudicated using a DRG
// CLP12 same as CLP11
// CLP13 is optional, when a discharge fraction was applied

pub fn get_clp(bpr_content: String) -> CLP {
    let clp_parts: Vec<&str> = bpr_content.split("*").collect();
    let mut clp05_patient_responsibility_amount: String ="".to_string();
    let mut clp08_facility_type_code: String ="".to_string();
    let mut clp09_claim_frequency_code: String ="".to_string();
    let mut clp11_diagnosis_related_group: String ="".to_string();
    let mut clp12_drg_weight: String ="".to_string();
    let mut clp13_percent_discharge_fraction: String ="".to_string();
    if clp_parts.get(4).is_some() {
        clp05_patient_responsibility_amount = clp_parts[4].to_string();
    }
    if clp_parts.get(7).is_some() {
        clp08_facility_type_code = clp_parts[7].to_string();
    }
    if clp_parts.get(8).is_some() {
        clp09_claim_frequency_code = clp_parts[8].to_string();
    }
    if clp_parts.get(9).is_some() {
        clp11_diagnosis_related_group = clp_parts[9].to_string();
    }
    if clp_parts.get(10).is_some() {
        clp12_drg_weight = clp_parts[10].to_string();
    }
    if clp_parts.get(13).is_some() {
        clp13_percent_discharge_fraction = clp_parts[13].to_string();
    }

    CLP {
        clp01_patient_control_number: clp_parts[0].to_string(),
        clp02_claim_status_code: clp_parts[1].to_string(),
        clp03_total_claim_charge_amount: clp_parts[2].to_string(),
        clp04_total_claim_payment_amount: clp_parts[3].to_string(),
        clp05_patient_responsibility_amount,
        clp06_claim_filing_indicator_code: clp_parts[5].to_string(),
        clp07_payer_claim_control_number: clp_parts[6].to_string(),
        clp08_facility_type_code,
        clp09_claim_frequency_code,
        // patient_status_code: clp_parts[10].to_string(),
        clp11_diagnosis_related_group,
        clp12_drg_weight,
        clp13_percent_discharge_fraction,
    }
}

pub fn write_clp(clp:CLP) -> String {
    if clp.clp01_patient_control_number.is_empty() {
        return String::new();
    }
    let mut clp_content: String = String::new();
    clp_content.push_str("CLP*");
    clp_content.push_str(&clp.clp01_patient_control_number);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp02_claim_status_code);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp03_total_claim_charge_amount);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp04_total_claim_payment_amount);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp05_patient_responsibility_amount);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp06_claim_filing_indicator_code);
    clp_content.push_str("*");
    clp_content.push_str(&clp.clp07_payer_claim_control_number);
    clp_content.push_str(&stiuational_element(clp.clp08_facility_type_code));
    clp_content.push_str(&stiuational_element(clp.clp09_claim_frequency_code));
    clp_content.push_str(&stiuational_element(clp.clp11_diagnosis_related_group));
    clp_content.push_str(&stiuational_element(clp.clp12_drg_weight));
    clp_content.push_str(&stiuational_element(clp.clp13_percent_discharge_fraction));
    clp_content.push_str("~");
    clp_content
}




// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_clp() {
        let bpr_content = "EXAMPLE9*3*500*100**12*05090256390*11*1".to_string();
        let clp = get_clp(bpr_content);
        assert_eq!(clp.clp01_patient_control_number, "EXAMPLE9");
        assert_eq!(clp.clp02_claim_status_code, "3");
        assert_eq!(clp.clp03_total_claim_charge_amount, "500");
        assert_eq!(clp.clp04_total_claim_payment_amount, "100");
        assert_eq!(clp.clp05_patient_responsibility_amount, "");
        assert_eq!(clp.clp06_claim_filing_indicator_code, "12");
        assert_eq!(clp.clp07_payer_claim_control_number, "05090256390");
        assert_eq!(clp.clp08_facility_type_code, "11");
        assert_eq!(clp.clp09_claim_frequency_code, "1");
        assert_eq!(clp.clp11_diagnosis_related_group, "");
        assert_eq!(clp.clp12_drg_weight, "");
        assert_eq!(clp.clp13_percent_discharge_fraction, "");
    }
}