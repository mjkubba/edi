use serde::{Serialize, Deserialize};

/// PRV - Provider Information
/// 
/// This segment is used to specify provider information.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PRV {
    pub segment_id: String,
    pub prv01_provider_code: String,
    pub prv02_reference_identification_qualifier: String,
    pub prv03_reference_identification: String,
    pub prv04_state_or_province_code: Option<String>,
    pub prv05_provider_specialty_information: Option<String>,
    pub prv06_provider_organization_code: Option<String>,
}

/// Parse a PRV segment string into a PRV struct
#[allow(dead_code)]
pub fn get_prv(segment: &str) -> PRV {
    let elements: Vec<&str> = segment.split('*').collect();
    
    let mut prv = PRV {
        segment_id: elements[0].to_string(),
        ..Default::default()
    };
    
    // Process PRV01 - Provider Code
    if elements.len() > 1 && !elements[1].is_empty() {
        prv.prv01_provider_code = elements[1].to_string();
    }
    
    // Process PRV02 - Reference Identification Qualifier
    if elements.len() > 2 && !elements[2].is_empty() {
        prv.prv02_reference_identification_qualifier = elements[2].to_string();
    }
    
    // Process PRV03 - Reference Identification
    if elements.len() > 3 && !elements[3].is_empty() {
        prv.prv03_reference_identification = elements[3].to_string();
    }
    
    // Process PRV04 - State or Province Code
    if elements.len() > 4 && !elements[4].is_empty() {
        prv.prv04_state_or_province_code = Some(elements[4].to_string());
    }
    
    // Process PRV05 - Provider Specialty Information
    if elements.len() > 5 && !elements[5].is_empty() {
        prv.prv05_provider_specialty_information = Some(elements[5].to_string());
    }
    
    // Process PRV06 - Provider Organization Code
    if elements.len() > 6 && !elements[6].is_empty() {
        prv.prv06_provider_organization_code = Some(elements[6].to_string());
    }
    
    prv
}

/// Convert a PRV struct to an EDI segment string
#[allow(dead_code)]
pub fn write_prv(prv: &PRV) -> String {
    let mut result = String::new();
    
    // Add segment ID
    result.push_str(&prv.segment_id);
    result.push('*');
    
    // Add PRV01 - Provider Code
    result.push_str(&prv.prv01_provider_code);
    result.push('*');
    
    // Add PRV02 - Reference Identification Qualifier
    result.push_str(&prv.prv02_reference_identification_qualifier);
    result.push('*');
    
    // Add PRV03 - Reference Identification
    result.push_str(&prv.prv03_reference_identification);
    
    // Add PRV04 - State or Province Code
    if let Some(state_or_province_code) = &prv.prv04_state_or_province_code {
        result.push('*');
        result.push_str(state_or_province_code);
    } else if prv.prv05_provider_specialty_information.is_some() || 
              prv.prv06_provider_organization_code.is_some() {
        result.push('*');
    }
    
    // Add PRV05 - Provider Specialty Information
    if let Some(provider_specialty_information) = &prv.prv05_provider_specialty_information {
        result.push('*');
        result.push_str(provider_specialty_information);
    } else if prv.prv06_provider_organization_code.is_some() {
        result.push('*');
    }
    
    // Add PRV06 - Provider Organization Code
    if let Some(provider_organization_code) = &prv.prv06_provider_organization_code {
        result.push('*');
        result.push_str(provider_organization_code);
    }
    
    // Remove trailing delimiters
    while result.ends_with('*') {
        result.pop();
    }
    
    // Add segment terminator
    result.push('~');
    
    result
}
