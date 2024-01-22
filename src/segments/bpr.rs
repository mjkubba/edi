// EDI 835 BPR segment
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct BPR {
    transaction_handling_code: String,
    monetary_amount: String,
    credit_debit_flag: String,
    payment_method_code: String,
    payment_format_code: String,
    id_number_qualifier: String,
    id_number: String,
    account_number_qualifier: String,
    account_number: String,
    originating_company_identifier: String,
    originating_company_supplemental_code: String,
    dfi_identification_number_qualifier: String,
    dfi_identification_number: String,
    bpr14_account_number_qualifier: String,
    bpr15_account_number: String,
    date: String,
}

// if BPR04 is ACH then BPR05 is required
// if BPR04 is ACH, BOP or FWT then BPR06-BPR10,BPR12-BPR15 are required
// BPR11 is require when BPR10 is present

pub fn get_bpr(bpr_content: String) -> BPR {
    let bpr_parts: Vec<&str> = bpr_content.split("*").collect();
    let mut payment_format_code: String ="".to_string();
    let mut id_number_qualifier: String ="".to_string();
    let mut id_number: String ="".to_string();
    let mut account_number_qualifier: String ="".to_string();
    let mut account_number: String ="".to_string();
    let mut originating_company_identifier: String ="".to_string();
    let mut originating_company_supplemental_code: String ="".to_string();
    let mut dfi_identification_number_qualifier: String ="".to_string();
    let mut dfi_identification_number: String ="".to_string();
    let mut bpr14_account_number_qualifier: String ="".to_string();
    let mut bpr15_account_number: String ="".to_string();

    if bpr_parts.get(4).is_some() {
        payment_format_code = bpr_parts[4].to_string();
    }
    if bpr_parts.get(5).is_some() {
        id_number_qualifier = bpr_parts[5].to_string();
    }
    if bpr_parts.get(6).is_some() {
        id_number = bpr_parts[6].to_string();
    }
    if bpr_parts.get(7).is_some() {
        account_number_qualifier = bpr_parts[7].to_string();
    }
    if bpr_parts.get(8).is_some() {
        account_number = bpr_parts[8].to_string();
    }
    if bpr_parts.get(9).is_some() {
        originating_company_identifier = bpr_parts[9].to_string();
    }
    if bpr_parts.get(10).is_some() {
        originating_company_supplemental_code = bpr_parts[10].to_string();
    }
    if bpr_parts.get(11).is_some() {
        dfi_identification_number_qualifier = bpr_parts[11].to_string();
    }
    if bpr_parts.get(12).is_some() {
        dfi_identification_number = bpr_parts[12].to_string();
    }
    if bpr_parts.get(13).is_some() {
        bpr14_account_number_qualifier = bpr_parts[13].to_string();
    }
    if bpr_parts.get(14).is_some() {
        bpr15_account_number = bpr_parts[14].to_string();
    }

    BPR {
        transaction_handling_code: bpr_parts[0].to_string(),
        monetary_amount: bpr_parts[1].to_string(),
        credit_debit_flag: bpr_parts[2].to_string(),
        payment_method_code: bpr_parts[3].to_string(),
        payment_format_code,
        id_number_qualifier,
        id_number,
        account_number_qualifier,
        account_number,
        originating_company_identifier,
        originating_company_supplemental_code,
        dfi_identification_number_qualifier,
        dfi_identification_number,
        bpr14_account_number_qualifier,
        bpr15_account_number,
        date: bpr_parts[15].to_string(),
    }
}