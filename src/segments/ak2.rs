use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AK2 {
    pub ak201_transaction_set_identifier_code: String,
    pub ak202_transaction_set_control_number: String,
    pub ak203_implementation_convention_reference: String,
}

pub fn get_ak2(ak2_content: String) -> AK2 {
    let ak2_parts: Vec<&str> = ak2_content.split("*").collect();
    
    let mut ak2 = AK2::default();
    
    // AK201 - Transaction Set Identifier Code (Required)
    if !ak2_parts.is_empty() && !ak2_parts[0].is_empty() {
        ak2.ak201_transaction_set_identifier_code = ak2_parts[0].to_string();
    }
    
    // AK202 - Transaction Set Control Number (Required)
    if ak2_parts.len() > 1 && !ak2_parts[1].is_empty() {
        ak2.ak202_transaction_set_control_number = ak2_parts[1].to_string();
    }
    
    // AK203 - Implementation Convention Reference (Situational)
    if ak2_parts.len() > 2 && !ak2_parts[2].is_empty() {
        ak2.ak203_implementation_convention_reference = ak2_parts[2].to_string();
    }
    
    info!("Parsed AK2 segment: {:?}", ak2);
    ak2
}

pub fn write_ak2(ak2: AK2) -> String {
    let mut ak2_content = String::new();
    
    ak2_content.push_str("AK2*");
    ak2_content.push_str(&ak2.ak201_transaction_set_identifier_code);
    ak2_content.push_str("*");
    ak2_content.push_str(&ak2.ak202_transaction_set_control_number);
    
    // Only include AK203 if not empty
    if !ak2.ak203_implementation_convention_reference.is_empty() {
        ak2_content.push_str("*");
        ak2_content.push_str(&ak2.ak203_implementation_convention_reference);
    }
    
    ak2_content.push_str("~");
    ak2_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ak2() {
        let ak2_content = "837*000000001*005010X222A1".to_string();
        let ak2 = get_ak2(ak2_content);
        
        assert_eq!(ak2.ak201_transaction_set_identifier_code, "837");
        assert_eq!(ak2.ak202_transaction_set_control_number, "000000001");
        assert_eq!(ak2.ak203_implementation_convention_reference, "005010X222A1");
    }
    
    #[test]
    fn test_get_ak2_minimal() {
        let ak2_content = "837*000000001".to_string();
        let ak2 = get_ak2(ak2_content);
        
        assert_eq!(ak2.ak201_transaction_set_identifier_code, "837");
        assert_eq!(ak2.ak202_transaction_set_control_number, "000000001");
        assert_eq!(ak2.ak203_implementation_convention_reference, "");
    }
    
    #[test]
    fn test_write_ak2() {
        let ak2 = AK2 {
            ak201_transaction_set_identifier_code: "837".to_string(),
            ak202_transaction_set_control_number: "000000001".to_string(),
            ak203_implementation_convention_reference: "005010X222A1".to_string(),
        };
        
        let ak2_content = write_ak2(ak2);
        assert_eq!(ak2_content, "AK2*837*000000001*005010X222A1~");
    }
    
    #[test]
    fn test_write_ak2_minimal() {
        let ak2 = AK2 {
            ak201_transaction_set_identifier_code: "837".to_string(),
            ak202_transaction_set_control_number: "000000001".to_string(),
            ak203_implementation_convention_reference: "".to_string(),
        };
        
        let ak2_content = write_ak2(ak2);
        assert_eq!(ak2_content, "AK2*837*000000001~");
    }
}
