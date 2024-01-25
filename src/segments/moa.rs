// EDI 835 MOA - PROVIDER SUMMARY INFORMATION
#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]
pub struct MOA {
    pub moa01_reimbursement_rate: String,
    pub moa02_claim_hcpcs_payable_amount: String,
    pub moa03_claim_payment_remark_code: String,
    pub moa04_claim_payment_remark_code: String,
    pub moa05_claim_payment_remark_code: String,
    pub moa06_claim_payment_remark_code: String,
    pub moa07_claim_payment_remark_code: String,
    pub moa08_claim_esrd_payment_amount: String,
    pub moa09_nonpayable_professional_component_amount: String,
}



pub fn get_moa(moa_content: String) -> MOA {
    let moa_parts: Vec<&str> = moa_content.split("*").collect();
    let mut moa01_reimbursement_rate: String ="".to_string();
    let mut moa02_claim_hcpcs_payable_amount: String ="".to_string();
    let mut moa03_claim_payment_remark_code: String ="".to_string();
    let mut moa04_claim_payment_remark_code: String ="".to_string();
    let mut moa05_claim_payment_remark_code: String ="".to_string();
    let mut moa06_claim_payment_remark_code: String ="".to_string();
    let mut moa07_claim_payment_remark_code: String ="".to_string();
    let mut moa08_claim_esrd_payment_amount: String ="".to_string();
    let mut moa09_nonpayable_professional_component_amount: String ="".to_string();

    if moa_parts.get(0).is_some() {
        moa01_reimbursement_rate = moa_parts[0].to_string();
    }
    if moa_parts.get(1).is_some() {
        moa02_claim_hcpcs_payable_amount = moa_parts[1].to_string();
    }
    if moa_parts.get(2).is_some() {
        moa03_claim_payment_remark_code = moa_parts[2].to_string();
    }
    if moa_parts.get(3).is_some() {
        moa04_claim_payment_remark_code = moa_parts[3].to_string();
    }
    if moa_parts.get(4).is_some() {
        moa05_claim_payment_remark_code = moa_parts[4].to_string();
    }
    if moa_parts.get(5).is_some() {
        moa06_claim_payment_remark_code = moa_parts[5].to_string();
    }
    if moa_parts.get(6).is_some() {
        moa07_claim_payment_remark_code = moa_parts[6].to_string();
    }
    if moa_parts.get(7).is_some() {
        moa08_claim_esrd_payment_amount = moa_parts[7].to_string();
    }
    if moa_parts.get(8).is_some() {
        moa09_nonpayable_professional_component_amount = moa_parts[8].to_string();
    }
    MOA {
        moa01_reimbursement_rate,
        moa02_claim_hcpcs_payable_amount,
        moa03_claim_payment_remark_code,
        moa04_claim_payment_remark_code,
        moa05_claim_payment_remark_code,
        moa06_claim_payment_remark_code,
        moa07_claim_payment_remark_code,
        moa08_claim_esrd_payment_amount,
        moa09_nonpayable_professional_component_amount,
    }
}


// unit test


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_moa() {
        let moa_content = "0.00*0.00*0*0*0*0*0.00*0.00*1".to_string();
        let moa = get_moa(moa_content);
        assert_eq!(moa.moa01_reimbursement_rate, "0.00");
        assert_eq!(moa.moa02_claim_hcpcs_payable_amount, "0.00");
        assert_eq!(moa.moa03_claim_payment_remark_code, "0");
        assert_eq!(moa.moa04_claim_payment_remark_code, "0");
        assert_eq!(moa.moa05_claim_payment_remark_code, "0");
        assert_eq!(moa.moa06_claim_payment_remark_code, "0");
        assert_eq!(moa.moa07_claim_payment_remark_code, "0.00");
        assert_eq!(moa.moa08_claim_esrd_payment_amount, "0.00");
        assert_eq!(moa.moa09_nonpayable_professional_component_amount, "1");
    }
}




