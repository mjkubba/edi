use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AAA {
    pub aaa01_valid_request_indicator: String,
    pub aaa02_agency_qualifier_code: String,
    pub aaa03_reject_reason_code: String,
    pub aaa04_follow_up_action_code: String,
}

pub fn get_aaa(aaa_content: String) -> AAA {
    let aaa_parts: Vec<&str> = aaa_content.split("*").collect();
    
    let mut aaa = AAA::default();
    
    // AAA01 - Valid Request Indicator
    if !aaa_parts.is_empty() && !aaa_parts[0].is_empty() {
        aaa.aaa01_valid_request_indicator = aaa_parts[0].to_string();
    }
    
    // AAA02 - Agency Qualifier Code
    if aaa_parts.len() > 1 && !aaa_parts[1].is_empty() {
        aaa.aaa02_agency_qualifier_code = aaa_parts[1].to_string();
    }
    
    // AAA03 - Reject Reason Code
    if aaa_parts.len() > 2 && !aaa_parts[2].is_empty() {
        aaa.aaa03_reject_reason_code = aaa_parts[2].to_string();
    }
    
    // AAA04 - Follow-up Action Code
    if aaa_parts.len() > 3 && !aaa_parts[3].is_empty() {
        aaa.aaa04_follow_up_action_code = aaa_parts[3].to_string();
    }
    
    info!("Parsed AAA segment: {:?}", aaa);
    aaa
}

pub fn write_aaa(aaa: AAA) -> String {
    let mut aaa_content = String::new();
    
    aaa_content.push_str("AAA*");
    aaa_content.push_str(&aaa.aaa01_valid_request_indicator);
    aaa_content.push_str("*");
    aaa_content.push_str(&aaa.aaa02_agency_qualifier_code);
    aaa_content.push_str("*");
    aaa_content.push_str(&aaa.aaa03_reject_reason_code);
    
    // Include AAA04 if not empty
    if !aaa.aaa04_follow_up_action_code.is_empty() {
        aaa_content.push_str("*");
        aaa_content.push_str(&aaa.aaa04_follow_up_action_code);
    }
    
    aaa_content.push_str("~");
    aaa_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_aaa() {
        let aaa_content = "N*Y*72*C".to_string();
        let aaa = get_aaa(aaa_content);
        
        assert_eq!(aaa.aaa01_valid_request_indicator, "N");
        assert_eq!(aaa.aaa02_agency_qualifier_code, "Y");
        assert_eq!(aaa.aaa03_reject_reason_code, "72");
        assert_eq!(aaa.aaa04_follow_up_action_code, "C");
    }
    
    #[test]
    fn test_get_aaa_minimal() {
        let aaa_content = "N*Y*72".to_string();
        let aaa = get_aaa(aaa_content);
        
        assert_eq!(aaa.aaa01_valid_request_indicator, "N");
        assert_eq!(aaa.aaa02_agency_qualifier_code, "Y");
        assert_eq!(aaa.aaa03_reject_reason_code, "72");
        assert_eq!(aaa.aaa04_follow_up_action_code, "");
    }
    
    #[test]
    fn test_write_aaa() {
        let aaa = AAA {
            aaa01_valid_request_indicator: "N".to_string(),
            aaa02_agency_qualifier_code: "Y".to_string(),
            aaa03_reject_reason_code: "72".to_string(),
            aaa04_follow_up_action_code: "C".to_string(),
        };
        
        let aaa_content = write_aaa(aaa);
        assert_eq!(aaa_content, "AAA*N*Y*72*C~");
    }
    
    #[test]
    fn test_write_aaa_minimal() {
        let aaa = AAA {
            aaa01_valid_request_indicator: "N".to_string(),
            aaa02_agency_qualifier_code: "Y".to_string(),
            aaa03_reject_reason_code: "72".to_string(),
            aaa04_follow_up_action_code: "".to_string(),
        };
        
        let aaa_content = write_aaa(aaa);
        assert_eq!(aaa_content, "AAA*N*Y*72~");
    }
}
