use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EQ {
    pub eq01_service_type_code: String,
    pub eq02_composite_medical_procedure_identifier: String,
    pub eq03_coverage_level_code: String,
    pub eq04_insurance_type_code: String,
}

pub fn get_eq(eq_content: String) -> EQ {
    let eq_parts: Vec<&str> = eq_content.split("*").collect();
    
    let mut eq = EQ::default();
    
    // EQ01 - Service Type Code
    if !eq_parts.is_empty() && !eq_parts[0].is_empty() {
        eq.eq01_service_type_code = eq_parts[0].to_string();
    }
    
    // EQ02 - Composite Medical Procedure Identifier
    if eq_parts.len() > 1 && !eq_parts[1].is_empty() {
        eq.eq02_composite_medical_procedure_identifier = eq_parts[1].to_string();
    }
    
    // EQ03 - Coverage Level Code
    if eq_parts.len() > 2 && !eq_parts[2].is_empty() {
        eq.eq03_coverage_level_code = eq_parts[2].to_string();
    }
    
    // EQ04 - Insurance Type Code
    if eq_parts.len() > 3 && !eq_parts[3].is_empty() {
        eq.eq04_insurance_type_code = eq_parts[3].to_string();
    }
    
    info!("Parsed EQ segment: {:?}", eq);
    eq
}

pub fn write_eq(eq: EQ) -> String {
    let mut eq_content = String::new();
    
    eq_content.push_str("EQ*");
    eq_content.push_str(&eq.eq01_service_type_code);
    
    // Only include subsequent fields if they're not empty
    if !eq.eq02_composite_medical_procedure_identifier.is_empty() || 
       !eq.eq03_coverage_level_code.is_empty() || 
       !eq.eq04_insurance_type_code.is_empty() {
        eq_content.push_str("*");
        eq_content.push_str(&eq.eq02_composite_medical_procedure_identifier);
    }
    
    if !eq.eq03_coverage_level_code.is_empty() || 
       !eq.eq04_insurance_type_code.is_empty() {
        eq_content.push_str("*");
        eq_content.push_str(&eq.eq03_coverage_level_code);
    }
    
    if !eq.eq04_insurance_type_code.is_empty() {
        eq_content.push_str("*");
        eq_content.push_str(&eq.eq04_insurance_type_code);
    }
    
    eq_content.push_str("~");
    
    eq_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_eq_minimal() {
        let eq_content = "30".to_string();
        let eq = get_eq(eq_content);
        
        assert_eq!(eq.eq01_service_type_code, "30");
        assert_eq!(eq.eq02_composite_medical_procedure_identifier, "");
        assert_eq!(eq.eq03_coverage_level_code, "");
        assert_eq!(eq.eq04_insurance_type_code, "");
    }
    
    #[test]
    fn test_get_eq_full() {
        let eq_content = "30*AD:CPT:99211*IND*HM".to_string();
        let eq = get_eq(eq_content);
        
        assert_eq!(eq.eq01_service_type_code, "30");
        assert_eq!(eq.eq02_composite_medical_procedure_identifier, "AD:CPT:99211");
        assert_eq!(eq.eq03_coverage_level_code, "IND");
        assert_eq!(eq.eq04_insurance_type_code, "HM");
    }
    
    #[test]
    fn test_write_eq_minimal() {
        let eq = EQ {
            eq01_service_type_code: "30".to_string(),
            eq02_composite_medical_procedure_identifier: "".to_string(),
            eq03_coverage_level_code: "".to_string(),
            eq04_insurance_type_code: "".to_string(),
        };
        
        let eq_content = write_eq(eq);
        assert_eq!(eq_content, "EQ*30~");
    }
    
    #[test]
    fn test_write_eq_full() {
        let eq = EQ {
            eq01_service_type_code: "30".to_string(),
            eq02_composite_medical_procedure_identifier: "AD:CPT:99211".to_string(),
            eq03_coverage_level_code: "IND".to_string(),
            eq04_insurance_type_code: "HM".to_string(),
        };
        
        let eq_content = write_eq(eq);
        assert_eq!(eq_content, "EQ*30*AD:CPT:99211*IND*HM~");
    }
}
