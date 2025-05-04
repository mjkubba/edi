use serde::{Serialize, Deserialize};

/// STC - Claim Status Information
/// 
/// This segment is used to convey information about the status of a claim or service line.
/// It is primarily used in the 277 Health Care Claim Status Response transaction.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct STC {
    pub segment_id: String,
    pub stc01_health_care_claim_status: String,
    pub stc01_1_claim_status_category_code: String,
    pub stc01_2_claim_status_code: String,
    pub stc01_3_entity_identifier_code: Option<String>,
    pub stc01_4_code_list_qualifier_code: Option<String>,
    pub stc02_status_information_effective_date: String,
    pub stc03_action_code: Option<String>,
    pub stc04_monetary_amount: Option<String>,
    pub stc05_monetary_amount: Option<String>,
    pub stc06_date: Option<String>,
    pub stc07_payment_method_code: Option<String>,
    pub stc08_date: Option<String>,
    pub stc09_check_number: Option<String>,
    pub stc10_health_care_claim_status: Option<String>,
    pub stc10_1_claim_status_category_code: Option<String>,
    pub stc10_2_claim_status_code: Option<String>,
    pub stc10_3_entity_identifier_code: Option<String>,
    pub stc10_4_code_list_qualifier_code: Option<String>,
    pub stc11_health_care_claim_status: Option<String>,
    pub stc11_1_claim_status_category_code: Option<String>,
    pub stc11_2_claim_status_code: Option<String>,
    pub stc11_3_entity_identifier_code: Option<String>,
    pub stc11_4_code_list_qualifier_code: Option<String>,
    pub stc12_free_form_message_text: Option<String>,
}

/// Parse an STC segment string into an STC struct
#[allow(dead_code)]
pub fn get_stc(segment: &str) -> STC {
    let elements: Vec<&str> = segment.split('*').collect();
    
    let mut stc = STC {
        segment_id: elements[0].to_string(),
        ..Default::default()
    };
    
    // Process STC01 - Health Care Claim Status
    if elements.len() > 1 && !elements[1].is_empty() {
        let stc01_parts: Vec<&str> = elements[1].split(':').collect();
        if stc01_parts.len() > 0 {
            stc.stc01_1_claim_status_category_code = stc01_parts[0].to_string();
        }
        if stc01_parts.len() > 1 {
            stc.stc01_2_claim_status_code = stc01_parts[1].to_string();
        }
        if stc01_parts.len() > 2 {
            stc.stc01_3_entity_identifier_code = Some(stc01_parts[2].to_string());
        }
        if stc01_parts.len() > 3 {
            stc.stc01_4_code_list_qualifier_code = Some(stc01_parts[3].to_string());
        }
        
        stc.stc01_health_care_claim_status = elements[1].to_string();
    }
    
    // Process STC02 - Status Information Effective Date
    if elements.len() > 2 && !elements[2].is_empty() {
        stc.stc02_status_information_effective_date = elements[2].to_string();
    }
    
    // Process STC03 - Action Code
    if elements.len() > 3 && !elements[3].is_empty() {
        stc.stc03_action_code = Some(elements[3].to_string());
    }
    
    // Process STC04 - Monetary Amount
    if elements.len() > 4 && !elements[4].is_empty() {
        stc.stc04_monetary_amount = Some(elements[4].to_string());
    }
    
    // Process STC05 - Monetary Amount
    if elements.len() > 5 && !elements[5].is_empty() {
        stc.stc05_monetary_amount = Some(elements[5].to_string());
    }
    
    // Process STC06 - Date
    if elements.len() > 6 && !elements[6].is_empty() {
        stc.stc06_date = Some(elements[6].to_string());
    }
    
    // Process STC07 - Payment Method Code
    if elements.len() > 7 && !elements[7].is_empty() {
        stc.stc07_payment_method_code = Some(elements[7].to_string());
    }
    
    // Process STC08 - Date
    if elements.len() > 8 && !elements[8].is_empty() {
        stc.stc08_date = Some(elements[8].to_string());
    }
    
    // Process STC09 - Check Number
    if elements.len() > 9 && !elements[9].is_empty() {
        stc.stc09_check_number = Some(elements[9].to_string());
    }
    
    // Process STC10 - Health Care Claim Status
    if elements.len() > 10 && !elements[10].is_empty() {
        let stc10_parts: Vec<&str> = elements[10].split(':').collect();
        if stc10_parts.len() > 0 {
            stc.stc10_1_claim_status_category_code = Some(stc10_parts[0].to_string());
        }
        if stc10_parts.len() > 1 {
            stc.stc10_2_claim_status_code = Some(stc10_parts[1].to_string());
        }
        if stc10_parts.len() > 2 {
            stc.stc10_3_entity_identifier_code = Some(stc10_parts[2].to_string());
        }
        if stc10_parts.len() > 3 {
            stc.stc10_4_code_list_qualifier_code = Some(stc10_parts[3].to_string());
        }
        
        stc.stc10_health_care_claim_status = Some(elements[10].to_string());
    }
    
    // Process STC11 - Health Care Claim Status
    if elements.len() > 11 && !elements[11].is_empty() {
        let stc11_parts: Vec<&str> = elements[11].split(':').collect();
        if stc11_parts.len() > 0 {
            stc.stc11_1_claim_status_category_code = Some(stc11_parts[0].to_string());
        }
        if stc11_parts.len() > 1 {
            stc.stc11_2_claim_status_code = Some(stc11_parts[1].to_string());
        }
        if stc11_parts.len() > 2 {
            stc.stc11_3_entity_identifier_code = Some(stc11_parts[2].to_string());
        }
        if stc11_parts.len() > 3 {
            stc.stc11_4_code_list_qualifier_code = Some(stc11_parts[3].to_string());
        }
        
        stc.stc11_health_care_claim_status = Some(elements[11].to_string());
    }
    
    // Process STC12 - Free Form Message Text
    if elements.len() > 12 && !elements[12].is_empty() {
        stc.stc12_free_form_message_text = Some(elements[12].to_string());
    }
    
    stc
}

/// Convert an STC struct to an EDI segment string
#[allow(dead_code)]
pub fn write_stc(stc: &STC) -> String {
    let mut result = String::new();
    
    // Add segment ID
    result.push_str(&stc.segment_id);
    result.push('*');
    
    // Add STC01 - Health Care Claim Status
    if !stc.stc01_health_care_claim_status.is_empty() {
        result.push_str(&stc.stc01_health_care_claim_status);
    }
    result.push('*');
    
    // Add STC02 - Status Information Effective Date
    result.push_str(&stc.stc02_status_information_effective_date);
    result.push('*');
    
    // Add STC03 - Action Code
    if let Some(action_code) = &stc.stc03_action_code {
        result.push_str(action_code);
    }
    result.push('*');
    
    // Add STC04 - Monetary Amount
    if let Some(monetary_amount) = &stc.stc04_monetary_amount {
        result.push_str(monetary_amount);
    }
    result.push('*');
    
    // Add STC05 - Monetary Amount
    if let Some(monetary_amount) = &stc.stc05_monetary_amount {
        result.push_str(monetary_amount);
    }
    result.push('*');
    
    // Add STC06 - Date
    if let Some(date) = &stc.stc06_date {
        result.push_str(date);
    }
    result.push('*');
    
    // Add STC07 - Payment Method Code
    if let Some(payment_method_code) = &stc.stc07_payment_method_code {
        result.push_str(payment_method_code);
    }
    result.push('*');
    
    // Add STC08 - Date
    if let Some(date) = &stc.stc08_date {
        result.push_str(date);
    }
    result.push('*');
    
    // Add STC09 - Check Number
    if let Some(check_number) = &stc.stc09_check_number {
        result.push_str(check_number);
    }
    result.push('*');
    
    // Add STC10 - Health Care Claim Status
    if let Some(health_care_claim_status) = &stc.stc10_health_care_claim_status {
        result.push_str(health_care_claim_status);
    }
    result.push('*');
    
    // Add STC11 - Health Care Claim Status
    if let Some(health_care_claim_status) = &stc.stc11_health_care_claim_status {
        result.push_str(health_care_claim_status);
    }
    result.push('*');
    
    // Add STC12 - Free Form Message Text
    if let Some(free_form_message_text) = &stc.stc12_free_form_message_text {
        result.push_str(free_form_message_text);
    }
    
    // Remove trailing delimiters
    while result.ends_with('*') {
        result.pop();
    }
    
    // Add segment terminator
    result.push('~');
    
    result
}
