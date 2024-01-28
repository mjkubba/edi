use serde::{Serialize, Deserialize};

// EDI 835 BPR segment
#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct BPR {
    pub bpr01_transaction_handling_code: String,
    pub bpr02_monetary_amount: String,
    pub bpr03_credit_debit_flag: String,
    pub bpr04_payment_method_code: String,
    pub bpr05_payment_format_code: String,
    pub bpr06_id_number_qualifier: String,
    pub bpr07_id_number: String,
    pub bpr08_account_number_qualifier: String,
    pub bpr09_account_number: String,
    pub bpr10_originating_company_identifier: String,
    pub bpr11_originating_company_supplemental_code: String,
    pub bpr12_dfi_identification_number_qualifier: String,
    pub bpr13_dfi_identification_number: String,
    pub bpr14_account_number_qualifier: String,
    pub bpr15_account_number: String,
    pub bpr16_date: String,
}

// if BPR04 is ACH then BPR05 is required
// if BPR04 is ACH, BOP or FWT then BPR06-BPR10,BPR12-BPR15 are required
// BPR11 is require when BPR10 is present

pub fn get_bpr(bpr_content: String) -> BPR {
    let bpr_parts: Vec<&str> = bpr_content.split("*").collect();
    let mut bpr05_payment_format_code: String ="".to_string();
    let mut bpr06_id_number_qualifier: String ="".to_string();
    let mut bpr07_id_number: String ="".to_string();
    let mut bpr08_account_number_qualifier: String ="".to_string();
    let mut bpr09_account_number: String ="".to_string();
    let mut bpr10_originating_company_identifier: String ="".to_string();
    let mut bpr11_originating_company_supplemental_code: String ="".to_string();
    let mut bpr12_dfi_identification_number_qualifier: String ="".to_string();
    let mut bpr13_dfi_identification_number: String ="".to_string();
    let mut bpr14_account_number_qualifier: String ="".to_string();
    let mut bpr15_account_number: String ="".to_string();

    if bpr_parts.get(4).is_some() {
        bpr05_payment_format_code = bpr_parts[4].to_string();
    }
    if bpr_parts.get(5).is_some() {
        bpr06_id_number_qualifier = bpr_parts[5].to_string();
    }
    if bpr_parts.get(6).is_some() {
        bpr07_id_number = bpr_parts[6].to_string();
    }
    if bpr_parts.get(7).is_some() {
        bpr08_account_number_qualifier = bpr_parts[7].to_string();
    }
    if bpr_parts.get(8).is_some() {
        bpr09_account_number = bpr_parts[8].to_string();
    }
    if bpr_parts.get(9).is_some() {
        bpr10_originating_company_identifier = bpr_parts[9].to_string();
    }
    if bpr_parts.get(10).is_some() {
        bpr11_originating_company_supplemental_code = bpr_parts[10].to_string();
    }
    if bpr_parts.get(11).is_some() {
        bpr12_dfi_identification_number_qualifier = bpr_parts[11].to_string();
    }
    if bpr_parts.get(12).is_some() {
        bpr13_dfi_identification_number = bpr_parts[12].to_string();
    }
    if bpr_parts.get(13).is_some() {
        bpr14_account_number_qualifier = bpr_parts[13].to_string();
    }
    if bpr_parts.get(14).is_some() {
        bpr15_account_number = bpr_parts[14].to_string();
    }

    BPR {
        bpr01_transaction_handling_code: bpr_parts[0].to_string(),
        bpr02_monetary_amount: bpr_parts[1].to_string(),
        bpr03_credit_debit_flag: bpr_parts[2].to_string(),
        bpr04_payment_method_code: bpr_parts[3].to_string(),
        bpr05_payment_format_code,
        bpr06_id_number_qualifier,
        bpr07_id_number,
        bpr08_account_number_qualifier,
        bpr09_account_number,
        bpr10_originating_company_identifier,
        bpr11_originating_company_supplemental_code,
        bpr12_dfi_identification_number_qualifier,
        bpr13_dfi_identification_number,
        bpr14_account_number_qualifier,
        bpr15_account_number,
        bpr16_date: bpr_parts[15].to_string(),
    }
}

// unit test


#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_bpr() {
        let bpr_content = "I*100.00*C*CHK************20190331".to_string();
        let bpr = get_bpr(bpr_content);
        assert_eq!(bpr.bpr01_transaction_handling_code, "I");
        assert_eq!(bpr.bpr02_monetary_amount, "100.00");
        assert_eq!(bpr.bpr03_credit_debit_flag, "C");
    }
}