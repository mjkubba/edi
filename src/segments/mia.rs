use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

// EDI 835 MSI - PROVIDER SUMMARY INFORMATION
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct MIA {
    pub mia01_covered_days_or_visits_count: String,
    pub mia02_pps_operating_outlier_amount: String,
    pub mia03_lifetime_psychiatric_days_count: String,
    pub mia04_claim_drg_amount: String,
    pub mia05_claim_payment_remark_code: String,
    pub mia06_claim_disproportionate_share_amount: String,
    pub mia07_claim_msp_passthrough_amount: String,
    pub mia08_claim_pps_capital_amount: String,
    pub mia09_pps_capital_fsp_drg_amount: String,
    pub mia10_pps_capital_hsp_drg_amount: String,
    pub mia11_pps_capital_dsh_drg_amount: String,
    pub mia12_old_capital_amount: String,
    pub mia13_pps_capital_ime_amount: String,
    pub mia14_pps_operating_hospital_specific_drg_amount: String,
    pub mia15_cost_report_day_count: String,
    pub mia16_pps_operating_federal_specific_drg_amount: String,
    pub mia17_claim_pps_capital_outlier_amount: String,
    pub mia18_claim_indirect_teaching_amount: String,
    pub mia19_nonpayable_professional_component_amount: String,
    pub mia20_claim_payment_remark_code: String,
    pub mia21_claim_payment_remark_code: String,
    pub mia22_claim_payment_remark_code: String,
    pub mia23_claim_payment_remark_code: String,
    pub mia24_pps_capital_exception_amount: String,
}

pub fn get_mia(mia_content: String) -> MIA {
    let mia_parts: Vec<&str> = mia_content.split("*").collect();
    let mut mia02_pps_operating_outlier_amount: String = "".to_string();
    let mut mia03_lifetime_psychiatric_days_count: String = "".to_string();
    let mut mia04_claim_drg_amount: String = "".to_string();
    let mut mia05_claim_payment_remark_code: String = "".to_string();
    let mut mia06_claim_disproportionate_share_amount: String = "".to_string();
    let mut mia07_claim_msp_passthrough_amount: String = "".to_string();
    let mut mia08_claim_pps_capital_amount: String = "".to_string();
    let mut mia09_pps_capital_fsp_drg_amount: String = "".to_string();
    let mut mia10_pps_capital_hsp_drg_amount: String = "".to_string();
    let mut mia11_pps_capital_dsh_drg_amount: String = "".to_string();
    let mut mia12_old_capital_amount: String = "".to_string();
    let mut mia13_pps_capital_ime_amount: String = "".to_string();
    let mut mia14_pps_operating_hospital_specific_drg_amount: String = "".to_string();
    let mut mia15_cost_report_day_count: String = "".to_string();
    let mut mia16_pps_operating_federal_specific_drg_amount: String = "".to_string();
    let mut mia17_claim_pps_capital_outlier_amount: String = "".to_string();
    let mut mia18_claim_indirect_teaching_amount: String = "".to_string();
    let mut mia19_nonpayable_professional_component_amount: String = "".to_string();
    let mut mia20_claim_payment_remark_code: String = "".to_string();
    let mut mia21_claim_payment_remark_code: String = "".to_string();
    let mut mia22_claim_payment_remark_code: String = "".to_string();
    let mut mia23_claim_payment_remark_code: String = "".to_string();
    let mut mia24_pps_capital_exception_amount: String = "".to_string();

    if mia_parts.get(1).is_some() {
        mia02_pps_operating_outlier_amount = get_element(&mia_parts, 1);
    }
    if mia_parts.get(2).is_some() {
        mia03_lifetime_psychiatric_days_count = get_element(&mia_parts, 2);
    }
    if mia_parts.get(3).is_some() {
        mia04_claim_drg_amount = get_element(&mia_parts, 3);
    }
    if mia_parts.get(4).is_some() {
        mia05_claim_payment_remark_code = get_element(&mia_parts, 4);
    }
    if mia_parts.get(5).is_some() {
        mia06_claim_disproportionate_share_amount = get_element(&mia_parts, 5);
    }
    if mia_parts.get(6).is_some() {
        mia07_claim_msp_passthrough_amount = get_element(&mia_parts, 6);
    }
    if mia_parts.get(7).is_some() {
        mia08_claim_pps_capital_amount = get_element(&mia_parts, 7);
    }
    if mia_parts.get(8).is_some() {
        mia09_pps_capital_fsp_drg_amount = get_element(&mia_parts, 8);
    }
    if mia_parts.get(9).is_some() {
        mia10_pps_capital_hsp_drg_amount = get_element(&mia_parts, 9);
    }
    if mia_parts.get(10).is_some() {
        mia11_pps_capital_dsh_drg_amount = get_element(&mia_parts, 10);
    }
    if mia_parts.get(11).is_some() {
        mia12_old_capital_amount = get_element(&mia_parts, 11);
    }
    if mia_parts.get(12).is_some() {
        mia13_pps_capital_ime_amount = get_element(&mia_parts, 12);
    }
    if mia_parts.get(13).is_some() {
        mia14_pps_operating_hospital_specific_drg_amount = get_element(&mia_parts, 13);
    }
    if mia_parts.get(14).is_some() {
        mia15_cost_report_day_count = get_element(&mia_parts, 14);
    }
    if mia_parts.get(15).is_some() {
        mia16_pps_operating_federal_specific_drg_amount = get_element(&mia_parts, 15);
    }
    if mia_parts.get(16).is_some() {
        mia17_claim_pps_capital_outlier_amount = get_element(&mia_parts, 16);
    }
    if mia_parts.get(17).is_some() {
        mia18_claim_indirect_teaching_amount = get_element(&mia_parts, 17);
    }
    if mia_parts.get(18).is_some() {
        mia19_nonpayable_professional_component_amount = get_element(&mia_parts, 18);
    }
    if mia_parts.get(19).is_some() {
        mia20_claim_payment_remark_code = get_element(&mia_parts, 19);
    }
    if mia_parts.get(20).is_some() {
        mia21_claim_payment_remark_code = get_element(&mia_parts, 20);
    }
    if mia_parts.get(21).is_some() {
        mia22_claim_payment_remark_code = get_element(&mia_parts, 21);
    }
    if mia_parts.get(22).is_some() {
        mia23_claim_payment_remark_code = get_element(&mia_parts, 22);
    }
    if mia_parts.get(23).is_some() {
        mia24_pps_capital_exception_amount = get_element(&mia_parts, 23);
    }
    MIA {
        mia01_covered_days_or_visits_count: get_element(&mia_parts, 0),
        mia02_pps_operating_outlier_amount,
        mia03_lifetime_psychiatric_days_count,
        mia04_claim_drg_amount,
        mia05_claim_payment_remark_code,
        mia06_claim_disproportionate_share_amount,
        mia07_claim_msp_passthrough_amount,
        mia08_claim_pps_capital_amount,
        mia09_pps_capital_fsp_drg_amount,
        mia10_pps_capital_hsp_drg_amount,
        mia11_pps_capital_dsh_drg_amount,
        mia12_old_capital_amount,
        mia13_pps_capital_ime_amount,
        mia14_pps_operating_hospital_specific_drg_amount,
        mia15_cost_report_day_count,
        mia16_pps_operating_federal_specific_drg_amount,
        mia17_claim_pps_capital_outlier_amount,
        mia18_claim_indirect_teaching_amount,
        mia19_nonpayable_professional_component_amount,
        mia20_claim_payment_remark_code,
        mia21_claim_payment_remark_code,
        mia22_claim_payment_remark_code,
        mia23_claim_payment_remark_code,
        mia24_pps_capital_exception_amount,
    }
}

pub fn write_mia(mia: MIA) -> String {
    if mia.mia01_covered_days_or_visits_count.is_empty() {
        return String::new();
    }
    build_segment(&[
        "MIA",
        &mia.mia01_covered_days_or_visits_count,
        &mia.mia02_pps_operating_outlier_amount,
        &mia.mia03_lifetime_psychiatric_days_count,
        &mia.mia04_claim_drg_amount,
        &mia.mia05_claim_payment_remark_code,
        &mia.mia06_claim_disproportionate_share_amount,
        &mia.mia07_claim_msp_passthrough_amount,
        &mia.mia08_claim_pps_capital_amount,
        &mia.mia09_pps_capital_fsp_drg_amount,
        &mia.mia10_pps_capital_hsp_drg_amount,
        &mia.mia11_pps_capital_dsh_drg_amount,
        &mia.mia12_old_capital_amount,
        &mia.mia13_pps_capital_ime_amount,
        &mia.mia14_pps_operating_hospital_specific_drg_amount,
        &mia.mia15_cost_report_day_count,
        &mia.mia16_pps_operating_federal_specific_drg_amount,
        &mia.mia17_claim_pps_capital_outlier_amount,
        &mia.mia18_claim_indirect_teaching_amount,
        &mia.mia19_nonpayable_professional_component_amount,
        &mia.mia20_claim_payment_remark_code,
        &mia.mia21_claim_payment_remark_code,
        &mia.mia22_claim_payment_remark_code,
        &mia.mia23_claim_payment_remark_code,
        &mia.mia24_pps_capital_exception_amount,
    ])
}

// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_mia() {
        let mia_content =
            "1*2*3*4*5*6*7*8*9*10*11*12*13*14*15*16*17*18*19*20*21*22*23*24".to_string();
        let mia = get_mia(mia_content);
        assert_eq!(mia.mia01_covered_days_or_visits_count, "1".to_string());
        assert_eq!(mia.mia02_pps_operating_outlier_amount, "2".to_string());
        assert_eq!(mia.mia03_lifetime_psychiatric_days_count, "3".to_string());
        assert_eq!(mia.mia04_claim_drg_amount, "4".to_string());
        assert_eq!(mia.mia05_claim_payment_remark_code, "5".to_string());
    }
}
