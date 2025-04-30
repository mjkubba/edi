use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PRV {
    pub prv01_provider_code: String,
    pub prv02_reference_identification_qualifier: String,
    pub prv03_reference_identification: String,
    pub prv04_state_or_province_code: String,
    pub prv05_provider_specialty_information: String,
    pub prv06_provider_organization_code: String,
}

pub fn get_prv(prv_content: &str) -> PRV {
    let mut prv = PRV::default();
    
    let prv_elements: Vec<&str> = prv_content.split('*').collect();
    
    if prv_elements.len() >= 2 {
        prv.prv01_provider_code = prv_elements[1].to_string();
    }
    
    if prv_elements.len() >= 3 {
        prv.prv02_reference_identification_qualifier = prv_elements[2].to_string();
    }
    
    if prv_elements.len() >= 4 {
        prv.prv03_reference_identification = prv_elements[3].to_string();
    }
    
    if prv_elements.len() >= 5 {
        prv.prv04_state_or_province_code = prv_elements[4].to_string();
    }
    
    if prv_elements.len() >= 6 {
        prv.prv05_provider_specialty_information = prv_elements[5].to_string();
    }
    
    if prv_elements.len() >= 7 {
        prv.prv06_provider_organization_code = prv_elements[6].to_string();
    }
    
    prv
}

pub fn write_prv(prv: &PRV) -> String {
    // Filter out empty fields to avoid trailing delimiters
    let mut elements = Vec::new();
    elements.push("PRV".to_string());
    elements.push(prv.prv01_provider_code.clone());
    elements.push(prv.prv02_reference_identification_qualifier.clone());
    elements.push(prv.prv03_reference_identification.clone());
    
    if !prv.prv04_state_or_province_code.is_empty() || !prv.prv05_provider_specialty_information.is_empty() || 
       !prv.prv06_provider_organization_code.is_empty() {
        elements.push(prv.prv04_state_or_province_code.clone());
    }
    
    if !prv.prv05_provider_specialty_information.is_empty() || !prv.prv06_provider_organization_code.is_empty() {
        elements.push(prv.prv05_provider_specialty_information.clone());
    }
    
    if !prv.prv06_provider_organization_code.is_empty() {
        elements.push(prv.prv06_provider_organization_code.clone());
    }
    
    format!("{}~", elements.join("*"))
}
