use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DMG {
    pub dmg01_date_time_qualifier: String,
    pub dmg02_date_time_period: String,
    pub dmg03_gender_code: String,
    pub dmg04_marital_status_code: String,
    pub dmg05_race_or_ethnicity_code: String,
    pub dmg06_citizenship_status_code: String,
    pub dmg07_country_code: String,
    pub dmg08_basis_of_verification_code: String,
    pub dmg09_quantity: String,
}

pub fn get_dmg(dmg_content: String) -> DMG {
    let dmg_parts: Vec<&str> = dmg_content.split("*").collect();
    
    let mut dmg = DMG::default();
    
    // DMG01 - Date Time Qualifier
    if !dmg_parts.is_empty() && !dmg_parts[0].is_empty() {
        dmg.dmg01_date_time_qualifier = dmg_parts[0].to_string();
    }
    
    // DMG02 - Date Time Period
    if dmg_parts.len() > 1 && !dmg_parts[1].is_empty() {
        dmg.dmg02_date_time_period = dmg_parts[1].to_string();
    }
    
    // DMG03 - Gender Code
    if dmg_parts.len() > 2 && !dmg_parts[2].is_empty() {
        dmg.dmg03_gender_code = dmg_parts[2].to_string();
    }
    
    // DMG04 - Marital Status Code
    if dmg_parts.len() > 3 && !dmg_parts[3].is_empty() {
        dmg.dmg04_marital_status_code = dmg_parts[3].to_string();
    }
    
    // DMG05 - Race or Ethnicity Code
    if dmg_parts.len() > 4 && !dmg_parts[4].is_empty() {
        dmg.dmg05_race_or_ethnicity_code = dmg_parts[4].to_string();
    }
    
    // DMG06 - Citizenship Status Code
    if dmg_parts.len() > 5 && !dmg_parts[5].is_empty() {
        dmg.dmg06_citizenship_status_code = dmg_parts[5].to_string();
    }
    
    // DMG07 - Country Code
    if dmg_parts.len() > 6 && !dmg_parts[6].is_empty() {
        dmg.dmg07_country_code = dmg_parts[6].to_string();
    }
    
    // DMG08 - Basis of Verification Code
    if dmg_parts.len() > 7 && !dmg_parts[7].is_empty() {
        dmg.dmg08_basis_of_verification_code = dmg_parts[7].to_string();
    }
    
    // DMG09 - Quantity
    if dmg_parts.len() > 8 && !dmg_parts[8].is_empty() {
        dmg.dmg09_quantity = dmg_parts[8].to_string();
    }
    
    info!("Parsed DMG segment: {:?}", dmg);
    dmg
}

pub fn write_dmg(dmg: DMG) -> String {
    let mut dmg_content = String::new();
    
    dmg_content.push_str("DMG*");
    dmg_content.push_str(&dmg.dmg01_date_time_qualifier);
    dmg_content.push_str("*");
    dmg_content.push_str(&dmg.dmg02_date_time_period);
    
    // Only include non-empty fields
    if !dmg.dmg03_gender_code.is_empty() {
        dmg_content.push_str("*");
        dmg_content.push_str(&dmg.dmg03_gender_code);
        
        if !dmg.dmg04_marital_status_code.is_empty() {
            dmg_content.push_str("*");
            dmg_content.push_str(&dmg.dmg04_marital_status_code);
            
            if !dmg.dmg05_race_or_ethnicity_code.is_empty() {
                dmg_content.push_str("*");
                dmg_content.push_str(&dmg.dmg05_race_or_ethnicity_code);
                
                if !dmg.dmg06_citizenship_status_code.is_empty() {
                    dmg_content.push_str("*");
                    dmg_content.push_str(&dmg.dmg06_citizenship_status_code);
                    
                    if !dmg.dmg07_country_code.is_empty() {
                        dmg_content.push_str("*");
                        dmg_content.push_str(&dmg.dmg07_country_code);
                        
                        if !dmg.dmg08_basis_of_verification_code.is_empty() {
                            dmg_content.push_str("*");
                            dmg_content.push_str(&dmg.dmg08_basis_of_verification_code);
                            
                            if !dmg.dmg09_quantity.is_empty() {
                                dmg_content.push_str("*");
                                dmg_content.push_str(&dmg.dmg09_quantity);
                            }
                        }
                    }
                }
            }
        }
    }
    
    dmg_content.push_str("~");
    dmg_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_dmg() {
        let dmg_content = "D8*19800101*F*M*1*A*US*Y*2".to_string();
        let dmg = get_dmg(dmg_content);
        
        assert_eq!(dmg.dmg01_date_time_qualifier, "D8");
        assert_eq!(dmg.dmg02_date_time_period, "19800101");
        assert_eq!(dmg.dmg03_gender_code, "F");
        assert_eq!(dmg.dmg04_marital_status_code, "M");
        assert_eq!(dmg.dmg05_race_or_ethnicity_code, "1");
        assert_eq!(dmg.dmg06_citizenship_status_code, "A");
        assert_eq!(dmg.dmg07_country_code, "US");
        assert_eq!(dmg.dmg08_basis_of_verification_code, "Y");
        assert_eq!(dmg.dmg09_quantity, "2");
    }
    
    #[test]
    fn test_get_dmg_minimal() {
        let dmg_content = "D8*19800101".to_string();
        let dmg = get_dmg(dmg_content);
        
        assert_eq!(dmg.dmg01_date_time_qualifier, "D8");
        assert_eq!(dmg.dmg02_date_time_period, "19800101");
        assert_eq!(dmg.dmg03_gender_code, "");
    }
    
    #[test]
    fn test_write_dmg() {
        let dmg = DMG {
            dmg01_date_time_qualifier: "D8".to_string(),
            dmg02_date_time_period: "19800101".to_string(),
            dmg03_gender_code: "F".to_string(),
            dmg04_marital_status_code: "M".to_string(),
            dmg05_race_or_ethnicity_code: "1".to_string(),
            dmg06_citizenship_status_code: "A".to_string(),
            dmg07_country_code: "US".to_string(),
            dmg08_basis_of_verification_code: "Y".to_string(),
            dmg09_quantity: "2".to_string(),
        };
        
        let dmg_content = write_dmg(dmg);
        assert_eq!(dmg_content, "DMG*D8*19800101*F*M*1*A*US*Y*2~");
    }
    
    #[test]
    fn test_write_dmg_minimal() {
        let dmg = DMG {
            dmg01_date_time_qualifier: "D8".to_string(),
            dmg02_date_time_period: "19800101".to_string(),
            dmg03_gender_code: "".to_string(),
            dmg04_marital_status_code: "".to_string(),
            dmg05_race_or_ethnicity_code: "".to_string(),
            dmg06_citizenship_status_code: "".to_string(),
            dmg07_country_code: "".to_string(),
            dmg08_basis_of_verification_code: "".to_string(),
            dmg09_quantity: "".to_string(),
        };
        
        let dmg_content = write_dmg(dmg);
        assert_eq!(dmg_content, "DMG*D8*19800101~");
    }
}
