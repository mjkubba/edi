use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK5 {
    pub ik501_transaction_set_acknowledgment_code: String,
    pub ik502_implementation_transaction_set_syntax_error_code: String,
    pub ik503_implementation_transaction_set_syntax_error_code: String,
    pub ik504_implementation_transaction_set_syntax_error_code: String,
    pub ik505_implementation_transaction_set_syntax_error_code: String,
    pub ik506_implementation_transaction_set_syntax_error_code: String,
}

pub fn get_ik5(ik5_content: String) -> IK5 {
    let ik5_parts: Vec<&str> = ik5_content.split("*").collect();
    
    let mut ik5 = IK5::default();
    
    // IK501 - Transaction Set Acknowledgment Code (Required)
    if !ik5_parts.is_empty() && !ik5_parts[0].is_empty() {
        ik5.ik501_transaction_set_acknowledgment_code = ik5_parts[0].to_string();
    }
    
    // IK502 - Implementation Transaction Set Syntax Error Code (Situational)
    if ik5_parts.len() > 1 && !ik5_parts[1].is_empty() {
        ik5.ik502_implementation_transaction_set_syntax_error_code = ik5_parts[1].to_string();
    }
    
    // IK503 - Implementation Transaction Set Syntax Error Code (Situational)
    if ik5_parts.len() > 2 && !ik5_parts[2].is_empty() {
        ik5.ik503_implementation_transaction_set_syntax_error_code = ik5_parts[2].to_string();
    }
    
    // IK504 - Implementation Transaction Set Syntax Error Code (Situational)
    if ik5_parts.len() > 3 && !ik5_parts[3].is_empty() {
        ik5.ik504_implementation_transaction_set_syntax_error_code = ik5_parts[3].to_string();
    }
    
    // IK505 - Implementation Transaction Set Syntax Error Code (Situational)
    if ik5_parts.len() > 4 && !ik5_parts[4].is_empty() {
        ik5.ik505_implementation_transaction_set_syntax_error_code = ik5_parts[4].to_string();
    }
    
    // IK506 - Implementation Transaction Set Syntax Error Code (Situational)
    if ik5_parts.len() > 5 && !ik5_parts[5].is_empty() {
        ik5.ik506_implementation_transaction_set_syntax_error_code = ik5_parts[5].to_string();
    }
    
    info!("Parsed IK5 segment: {:?}", ik5);
    ik5
}

pub fn write_ik5(ik5: IK5) -> String {
    let mut ik5_content = String::new();
    
    ik5_content.push_str("IK5*");
    ik5_content.push_str(&ik5.ik501_transaction_set_acknowledgment_code);
    
    // Only include non-empty fields
    if !ik5.ik502_implementation_transaction_set_syntax_error_code.is_empty() {
        ik5_content.push_str("*");
        ik5_content.push_str(&ik5.ik502_implementation_transaction_set_syntax_error_code);
        
        if !ik5.ik503_implementation_transaction_set_syntax_error_code.is_empty() {
            ik5_content.push_str("*");
            ik5_content.push_str(&ik5.ik503_implementation_transaction_set_syntax_error_code);
            
            if !ik5.ik504_implementation_transaction_set_syntax_error_code.is_empty() {
                ik5_content.push_str("*");
                ik5_content.push_str(&ik5.ik504_implementation_transaction_set_syntax_error_code);
                
                if !ik5.ik505_implementation_transaction_set_syntax_error_code.is_empty() {
                    ik5_content.push_str("*");
                    ik5_content.push_str(&ik5.ik505_implementation_transaction_set_syntax_error_code);
                    
                    if !ik5.ik506_implementation_transaction_set_syntax_error_code.is_empty() {
                        ik5_content.push_str("*");
                        ik5_content.push_str(&ik5.ik506_implementation_transaction_set_syntax_error_code);
                    }
                }
            }
        }
    }
    
    ik5_content.push_str("~");
    ik5_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ik5() {
        let ik5_content = "A*1*2*3*4*5".to_string();
        let ik5 = get_ik5(ik5_content);
        
        assert_eq!(ik5.ik501_transaction_set_acknowledgment_code, "A");
        assert_eq!(ik5.ik502_implementation_transaction_set_syntax_error_code, "1");
        assert_eq!(ik5.ik503_implementation_transaction_set_syntax_error_code, "2");
        assert_eq!(ik5.ik504_implementation_transaction_set_syntax_error_code, "3");
        assert_eq!(ik5.ik505_implementation_transaction_set_syntax_error_code, "4");
        assert_eq!(ik5.ik506_implementation_transaction_set_syntax_error_code, "5");
    }
    
    #[test]
    fn test_get_ik5_minimal() {
        let ik5_content = "A".to_string();
        let ik5 = get_ik5(ik5_content);
        
        assert_eq!(ik5.ik501_transaction_set_acknowledgment_code, "A");
        assert_eq!(ik5.ik502_implementation_transaction_set_syntax_error_code, "");
    }
    
    #[test]
    fn test_write_ik5() {
        let ik5 = IK5 {
            ik501_transaction_set_acknowledgment_code: "A".to_string(),
            ik502_implementation_transaction_set_syntax_error_code: "1".to_string(),
            ik503_implementation_transaction_set_syntax_error_code: "2".to_string(),
            ik504_implementation_transaction_set_syntax_error_code: "3".to_string(),
            ik505_implementation_transaction_set_syntax_error_code: "4".to_string(),
            ik506_implementation_transaction_set_syntax_error_code: "5".to_string(),
        };
        
        let ik5_content = write_ik5(ik5);
        assert_eq!(ik5_content, "IK5*A*1*2*3*4*5~");
    }
    
    #[test]
    fn test_write_ik5_minimal() {
        let ik5 = IK5 {
            ik501_transaction_set_acknowledgment_code: "A".to_string(),
            ik502_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik503_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik504_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik505_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik506_implementation_transaction_set_syntax_error_code: "".to_string(),
        };
        
        let ik5_content = write_ik5(ik5);
        assert_eq!(ik5_content, "IK5*A~");
    }
}
