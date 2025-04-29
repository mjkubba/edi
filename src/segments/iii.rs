use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct III {
    pub code_list_qualifier_code: String,
    pub industry_code: String,
    pub code_category: String,
    pub free_form_message_text: String,
}

pub fn get_iii(iii_content: String) -> III {
    let iii_parts: Vec<&str> = iii_content.split("*").collect();
    
    let mut iii = III::default();
    
    // Code List Qualifier Code
    if !iii_parts.is_empty() && !iii_parts[0].is_empty() {
        iii.code_list_qualifier_code = iii_parts[0].to_string();
    }
    
    // Industry Code
    if iii_parts.len() > 1 && !iii_parts[1].is_empty() {
        iii.industry_code = iii_parts[1].to_string();
    }
    
    // Code Category
    if iii_parts.len() > 2 && !iii_parts[2].is_empty() {
        iii.code_category = iii_parts[2].to_string();
    }
    
    // Free Form Message Text
    if iii_parts.len() > 3 && !iii_parts[3].is_empty() {
        iii.free_form_message_text = iii_parts[3].to_string();
    }
    
    info!("Parsed III segment: {:?}", iii);
    iii
}

pub fn write_iii(iii: III) -> String {
    let mut iii_content = String::new();
    
    iii_content.push_str("III*");
    iii_content.push_str(&iii.code_list_qualifier_code);
    iii_content.push_str("*");
    iii_content.push_str(&iii.industry_code);
    iii_content.push_str("*");
    iii_content.push_str(&iii.code_category);
    iii_content.push_str("*");
    iii_content.push_str(&iii.free_form_message_text);
    iii_content.push_str("~");
    
    iii_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_iii() {
        let iii_content = "ZZ*ELIGIBILITY*Y*ADDITIONAL INFORMATION".to_string();
        let iii = get_iii(iii_content);
        
        assert_eq!(iii.code_list_qualifier_code, "ZZ");
        assert_eq!(iii.industry_code, "ELIGIBILITY");
        assert_eq!(iii.code_category, "Y");
        assert_eq!(iii.free_form_message_text, "ADDITIONAL INFORMATION");
    }
    
    #[test]
    fn test_write_iii() {
        let iii = III {
            code_list_qualifier_code: "ZZ".to_string(),
            industry_code: "ELIGIBILITY".to_string(),
            code_category: "Y".to_string(),
            free_form_message_text: "ADDITIONAL INFORMATION".to_string(),
        };
        
        let iii_content = write_iii(iii);
        assert_eq!(iii_content, "III*ZZ*ELIGIBILITY*Y*ADDITIONAL INFORMATION~");
    }
}
