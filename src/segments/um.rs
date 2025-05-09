use log::info;
use serde::{Serialize, Deserialize};

/// UM - Health Care Services Review Information
/// This segment provides information about health care services review.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UM {
    /// UM01 - Request Category Code
    /// Code specifying the type of request
    pub um01_request_category_code: String,
    
    /// UM02 - Certification Type Code
    /// Code specifying the type of certification/authorization
    pub um02_certification_type_code: String,
    
    /// UM03 - Service Type Code
    /// Code identifying the classification of service
    pub um03_service_type_code: String,
    
    /// UM04 - Health Care Service Location Information
    /// Information about the location of health care service
    pub um04_health_care_service_location_information: String,
    
    /// UM05 - Related Causes Information
    /// Information about related causes
    pub um05_related_causes_information: String,
    
    /// UM06 - Level of Service Code
    /// Code specifying the level of service rendered
    pub um06_level_of_service_code: String,
    
    /// UM07 - Current Health Condition Code
    /// Code specifying the current health condition
    pub um07_current_health_condition_code: String,
    
    /// UM08 - Prognosis Code
    /// Code specifying the prognosis
    pub um08_prognosis_code: String,
    
    /// UM09 - Release of Information Code
    /// Code indicating whether the provider has on file a signed statement by the patient authorizing the release of medical data to other organizations
    pub um09_release_of_information_code: String,
    
    /// UM10 - Delay Reason Code
    /// Code indicating the reason for delay in the claim submission
    pub um10_delay_reason_code: String,
}

impl UM {
    /// Create a new UM segment from an EDI segment string
    pub fn new(segment: &str) -> Self {
        let parts: Vec<&str> = segment.split('*').collect();
        let mut um = UM::default();
        
        // UM01 - Request Category Code
        if parts.len() > 1 && !parts[1].is_empty() {
            um.um01_request_category_code = parts[1].to_string();
        }
        
        // UM02 - Certification Type Code
        if parts.len() > 2 && !parts[2].is_empty() {
            um.um02_certification_type_code = parts[2].to_string();
        }
        
        // UM03 - Service Type Code
        if parts.len() > 3 && !parts[3].is_empty() {
            um.um03_service_type_code = parts[3].to_string();
        }
        
        // UM04 - Health Care Service Location Information
        if parts.len() > 4 && !parts[4].is_empty() {
            um.um04_health_care_service_location_information = parts[4].to_string();
        }
        
        // UM05 - Related Causes Information
        if parts.len() > 5 && !parts[5].is_empty() {
            um.um05_related_causes_information = parts[5].to_string();
        }
        
        // UM06 - Level of Service Code
        if parts.len() > 6 && !parts[6].is_empty() {
            um.um06_level_of_service_code = parts[6].to_string();
        }
        
        // UM07 - Current Health Condition Code
        if parts.len() > 7 && !parts[7].is_empty() {
            um.um07_current_health_condition_code = parts[7].to_string();
        }
        
        // UM08 - Prognosis Code
        if parts.len() > 8 && !parts[8].is_empty() {
            um.um08_prognosis_code = parts[8].to_string();
        }
        
        // UM09 - Release of Information Code
        if parts.len() > 9 && !parts[9].is_empty() {
            um.um09_release_of_information_code = parts[9].to_string();
        }
        
        // UM10 - Delay Reason Code
        if parts.len() > 10 && !parts[10].is_empty() {
            um.um10_delay_reason_code = parts[10].to_string();
        }
        
        info!("Parsed UM segment: {:?}", um);
        um
    }
    
    /// Convert the UM segment to an EDI string
    pub fn to_edi(&self) -> String {
        let mut result = String::from("UM*");
        
        // UM01 - Request Category Code
        result.push_str(&self.um01_request_category_code);
        result.push('*');
        
        // UM02 - Certification Type Code
        result.push_str(&self.um02_certification_type_code);
        result.push('*');
        
        // UM03 - Service Type Code
        result.push_str(&self.um03_service_type_code);
        
        // UM04 - Health Care Service Location Information
        if !self.um04_health_care_service_location_information.is_empty() {
            result.push('*');
            result.push_str(&self.um04_health_care_service_location_information);
            
            // UM05 - Related Causes Information
            if !self.um05_related_causes_information.is_empty() {
                result.push('*');
                result.push_str(&self.um05_related_causes_information);
                
                // UM06 - Level of Service Code
                if !self.um06_level_of_service_code.is_empty() {
                    result.push('*');
                    result.push_str(&self.um06_level_of_service_code);
                    
                    // UM07 - Current Health Condition Code
                    if !self.um07_current_health_condition_code.is_empty() {
                        result.push('*');
                        result.push_str(&self.um07_current_health_condition_code);
                        
                        // UM08 - Prognosis Code
                        if !self.um08_prognosis_code.is_empty() {
                            result.push('*');
                            result.push_str(&self.um08_prognosis_code);
                            
                            // UM09 - Release of Information Code
                            if !self.um09_release_of_information_code.is_empty() {
                                result.push('*');
                                result.push_str(&self.um09_release_of_information_code);
                                
                                // UM10 - Delay Reason Code
                                if !self.um10_delay_reason_code.is_empty() {
                                    result.push('*');
                                    result.push_str(&self.um10_delay_reason_code);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        result.push('~');
        result
    }
}

/// Get a UM segment from an EDI segment string
pub fn get_um(um_content: String) -> UM {
    UM::new(&um_content)
}

/// Convert a UM segment to an EDI string
pub fn write_um(um: UM) -> String {
    um.to_edi()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_um_full() {
        let um_content = "HS*I*1*2*3*4*5*6*7*Y*8".to_string();
        let um = get_um(um_content);
        
        assert_eq!(um.um01_request_category_code, "HS");
        assert_eq!(um.um02_certification_type_code, "I");
        assert_eq!(um.um03_service_type_code, "1");
        assert_eq!(um.um04_health_care_service_location_information, "2");
        assert_eq!(um.um05_related_causes_information, "3");
        assert_eq!(um.um06_level_of_service_code, "4");
        assert_eq!(um.um07_current_health_condition_code, "5");
        assert_eq!(um.um08_prognosis_code, "6");
        assert_eq!(um.um09_release_of_information_code, "7");
        assert_eq!(um.um10_delay_reason_code, "Y");
    }
    
    #[test]
    fn test_get_um_minimal() {
        let um_content = "HS*I*1".to_string();
        let um = get_um(um_content);
        
        assert_eq!(um.um01_request_category_code, "HS");
        assert_eq!(um.um02_certification_type_code, "I");
        assert_eq!(um.um03_service_type_code, "1");
        assert_eq!(um.um04_health_care_service_location_information, "");
        assert_eq!(um.um05_related_causes_information, "");
        assert_eq!(um.um06_level_of_service_code, "");
        assert_eq!(um.um07_current_health_condition_code, "");
        assert_eq!(um.um08_prognosis_code, "");
        assert_eq!(um.um09_release_of_information_code, "");
        assert_eq!(um.um10_delay_reason_code, "");
    }
    
    #[test]
    fn test_write_um_full() {
        let um = UM {
            um01_request_category_code: "HS".to_string(),
            um02_certification_type_code: "I".to_string(),
            um03_service_type_code: "1".to_string(),
            um04_health_care_service_location_information: "2".to_string(),
            um05_related_causes_information: "3".to_string(),
            um06_level_of_service_code: "4".to_string(),
            um07_current_health_condition_code: "5".to_string(),
            um08_prognosis_code: "6".to_string(),
            um09_release_of_information_code: "7".to_string(),
            um10_delay_reason_code: "Y".to_string(),
        };
        
        let um_content = write_um(um);
        assert_eq!(um_content, "UM*HS*I*1*2*3*4*5*6*7*Y~");
    }
    
    #[test]
    fn test_write_um_minimal() {
        let um = UM {
            um01_request_category_code: "HS".to_string(),
            um02_certification_type_code: "I".to_string(),
            um03_service_type_code: "1".to_string(),
            um04_health_care_service_location_information: "".to_string(),
            um05_related_causes_information: "".to_string(),
            um06_level_of_service_code: "".to_string(),
            um07_current_health_condition_code: "".to_string(),
            um08_prognosis_code: "".to_string(),
            um09_release_of_information_code: "".to_string(),
            um10_delay_reason_code: "".to_string(),
        };
        
        let um_content = write_um(um);
        assert_eq!(um_content, "UM*HS*I*1~");
    }
}
