use crate::helper::edihelper::build_segment;
use serde::{Deserialize, Serialize};

// EDI 835 TS3 - PROVIDER SUMMARY INFORMATION
// X12 spec defines 24 elements (TS301-TS324)
// 835 TR3 marks TS306-TS312, TS314, TS316, TS319 as NOT USED
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TS3 {
    pub ts301_provider_identifier: String,
    pub ts302_facility_type_code: String,
    pub ts303_fiscal_period_date: String,
    pub ts304_total_claim_count: String,
    pub ts305_total_claim_charge_amount: String,
    pub ts313_total_msp_payer_amount: String,
    pub ts315_total_non_lab_charge_amount: String,
    pub ts317_total_hcpcs_reported_charge_amount: String,
    pub ts318_total_hcpcs_payable_amount: String,
    pub ts320_total_professional_component_amount: String,
    pub ts321_total_msp_patient_liability_met_amount: String,
    pub ts322_total_patient_reimbursement_amount: String,
    pub ts323_total_pip_claim_count: String,
    pub ts324_total_pip_adjustment_amount: String,
}

pub fn get_ts3(ts3_content: String) -> TS3 {
    let ts3_parts: Vec<&str> = ts3_content.split("*").collect();
    let get = |i: usize| ts3_parts.get(i).unwrap_or(&"").to_string();

    TS3 {
        ts301_provider_identifier: get(0),
        ts302_facility_type_code: get(1),
        ts303_fiscal_period_date: get(2),
        ts304_total_claim_count: get(3),
        ts305_total_claim_charge_amount: get(4),
        // TS306-TS312 are NOT USED (positions 5-11)
        ts313_total_msp_payer_amount: get(12),
        // TS314 NOT USED (position 13)
        ts315_total_non_lab_charge_amount: get(14),
        // TS316 NOT USED (position 15)
        ts317_total_hcpcs_reported_charge_amount: get(16),
        ts318_total_hcpcs_payable_amount: get(17),
        // TS319 NOT USED (position 18)
        ts320_total_professional_component_amount: get(19),
        ts321_total_msp_patient_liability_met_amount: get(20),
        ts322_total_patient_reimbursement_amount: get(21),
        ts323_total_pip_claim_count: get(22),
        ts324_total_pip_adjustment_amount: get(23),
    }
}

pub fn write_ts3(ts3: TS3) -> String {
    if ts3.ts301_provider_identifier.is_empty() {
        return String::new();
    }
    build_segment(&[
        "TS3",
        &ts3.ts301_provider_identifier,       // 01
        &ts3.ts302_facility_type_code,        // 02
        &ts3.ts303_fiscal_period_date,        // 03
        &ts3.ts304_total_claim_count,         // 04
        &ts3.ts305_total_claim_charge_amount, // 05
        "",
        "",
        "",
        "",
        "",
        "",
        "",                                                // 06-12 NOT USED
        &ts3.ts313_total_msp_payer_amount,                 // 13
        "",                                                // 14 NOT USED
        &ts3.ts315_total_non_lab_charge_amount,            // 15
        "",                                                // 16 NOT USED
        &ts3.ts317_total_hcpcs_reported_charge_amount,     // 17
        &ts3.ts318_total_hcpcs_payable_amount,             // 18
        "",                                                // 19 NOT USED
        &ts3.ts320_total_professional_component_amount,    // 20
        &ts3.ts321_total_msp_patient_liability_met_amount, // 21
        &ts3.ts322_total_patient_reimbursement_amount,     // 22
        &ts3.ts323_total_pip_claim_count,                  // 23
        &ts3.ts324_total_pip_adjustment_amount,            // 24
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ts3() {
        // Full 24-element TS3 with data at correct positions
        let ts3_content = "PROV1*11*20021231*10*130957.66*******\
            *MSP_AMT**NON_LAB**HCPCS_RPT*HCPCS_PAY**PROF*MSP_LIAB*REIMB*PIP_CNT*PIP_ADJ";
        let ts3 = get_ts3(ts3_content.to_string());
        assert_eq!(ts3.ts301_provider_identifier, "PROV1");
        assert_eq!(ts3.ts305_total_claim_charge_amount, "130957.66");
        assert_eq!(ts3.ts313_total_msp_payer_amount, "MSP_AMT");
        assert_eq!(ts3.ts315_total_non_lab_charge_amount, "NON_LAB");
        assert_eq!(ts3.ts317_total_hcpcs_reported_charge_amount, "HCPCS_RPT");
        assert_eq!(ts3.ts318_total_hcpcs_payable_amount, "HCPCS_PAY");
        assert_eq!(ts3.ts320_total_professional_component_amount, "PROF");
    }

    #[test]
    fn test_get_ts3_minimal() {
        // Only mandatory elements, per TR3 example
        let ts3_content = "123456*11*20021031*10*130957.66";
        let ts3 = get_ts3(ts3_content.to_string());
        assert_eq!(ts3.ts301_provider_identifier, "123456");
        assert_eq!(ts3.ts305_total_claim_charge_amount, "130957.66");
        assert_eq!(ts3.ts313_total_msp_payer_amount, "");
    }

    #[test]
    fn test_write_ts3_minimal() {
        let mut ts3 = TS3::default();
        ts3.ts301_provider_identifier = "123456".to_string();
        ts3.ts302_facility_type_code = "11".to_string();
        ts3.ts303_fiscal_period_date = "20021031".to_string();
        ts3.ts304_total_claim_count = "10".to_string();
        ts3.ts305_total_claim_charge_amount = "130957.66".to_string();
        let result = write_ts3(ts3);
        assert_eq!(result, "TS3*123456*11*20021031*10*130957.66~");
    }

    #[test]
    fn test_write_ts3_with_ts313() {
        let mut ts3 = TS3::default();
        ts3.ts301_provider_identifier = "123456".to_string();
        ts3.ts302_facility_type_code = "11".to_string();
        ts3.ts303_fiscal_period_date = "20021031".to_string();
        ts3.ts304_total_claim_count = "10".to_string();
        ts3.ts305_total_claim_charge_amount = "130957.66".to_string();
        ts3.ts313_total_msp_payer_amount = "5000".to_string();
        let result = write_ts3(ts3);
        // 7 empty separators for TS306-TS312, then TS313
        assert_eq!(result, "TS3*123456*11*20021031*10*130957.66********5000~");
    }
}
