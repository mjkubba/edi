use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK4 {
    pub ik401_position_in_segment: String,
    pub ik402_data_element_reference_number: String,
    pub ik403_implementation_data_element_syntax_error_code: String,
    pub ik404_copy_of_bad_data_element: String,
}

pub fn get_ik4(ik4_content: String) -> IK4 {
    let ik4_parts: Vec<&str> = ik4_content.split("*").collect();
    
    let mut ik4 = IK4::default();
    
    // IK401 - Position in Segment (Required)
    if !ik4_parts.is_empty() && !ik4_parts[0].is_empty() {
        ik4.ik401_position_in_segment = ik4_parts[0].to_string();
    } else {
        // Default value for required field
        ik4.ik401_position_in_segment = "1".to_string();
    }
    
    // IK402 - Data Element Reference Number (Required)
    if ik4_parts.len() > 1 && !ik4_parts[1].is_empty() {
        ik4.ik402_data_element_reference_number = ik4_parts[1].to_string();
    } else {
        // Default value for required field
        ik4.ik402_data_element_reference_number = "0".to_string();
    }
    
    // IK403 - Implementation Data Element Syntax Error Code (Required)
    if ik4_parts.len() > 2 && !ik4_parts[2].is_empty() {
        ik4.ik403_implementation_data_element_syntax_error_code = ik4_parts[2].to_string();
    } else {
        // Default value for required field
        ik4.ik403_implementation_data_element_syntax_error_code = "1".to_string();
    }
    
    // IK404 - Copy of Bad Data Element (Situational)
    if ik4_parts.len() > 3 && !ik4_parts[3].is_empty() {
        ik4.ik404_copy_of_bad_data_element = ik4_parts[3].to_string();
    }
    
    info!("Parsed IK4 segment: {:?}", ik4_parts);
    ik4
}

pub fn write_ik4(ik4: IK4) -> String {
    let mut ik4_content = String::new();
    
    ik4_content.push_str("IK4*");
    
    // Ensure required fields have values
    let position = if ik4.ik401_position_in_segment.is_empty() { "1" } else { &ik4.ik401_position_in_segment };
    let reference = if ik4.ik402_data_element_reference_number.is_empty() { "0" } else { &ik4.ik402_data_element_reference_number };
    let error_code = if ik4.ik403_implementation_data_element_syntax_error_code.is_empty() { "1" } else { &ik4.ik403_implementation_data_element_syntax_error_code };
    
    ik4_content.push_str(position);
    ik4_content.push_str("*");
    ik4_content.push_str(reference);
    ik4_content.push_str("*");
    ik4_content.push_str(error_code);
    
    // Only include IK404 if not empty
    if !ik4.ik404_copy_of_bad_data_element.is_empty() {
        ik4_content.push_str("*");
        ik4_content.push_str(&ik4.ik404_copy_of_bad_data_element);
    }
    
    ik4_content.push_str("~");
    ik4_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ik4() {
        let ik4_content = "1*66*1*123".to_string();
        let ik4 = get_ik4(ik4_content);
        
        assert_eq!(ik4.ik401_position_in_segment, "1");
        assert_eq!(ik4.ik402_data_element_reference_number, "66");
        assert_eq!(ik4.ik403_implementation_data_element_syntax_error_code, "1");
        assert_eq!(ik4.ik404_copy_of_bad_data_element, "123");
    }
    
    #[test]
    fn test_get_ik4_minimal() {
        let ik4_content = "1*66*1".to_string();
        let ik4 = get_ik4(ik4_content);
        
        assert_eq!(ik4.ik401_position_in_segment, "1");
        assert_eq!(ik4.ik402_data_element_reference_number, "66");
        assert_eq!(ik4.ik403_implementation_data_element_syntax_error_code, "1");
        assert_eq!(ik4.ik404_copy_of_bad_data_element, "");
    }
    
    #[test]
    fn test_get_ik4_empty() {
        let ik4_content = "**".to_string();
        let ik4 = get_ik4(ik4_content);
        
        assert_eq!(ik4.ik401_position_in_segment, "1"); // Default value
        assert_eq!(ik4.ik402_data_element_reference_number, "0"); // Default value
        assert_eq!(ik4.ik403_implementation_data_element_syntax_error_code, "1"); // Default value
        assert_eq!(ik4.ik404_copy_of_bad_data_element, "");
    }
    
    #[test]
    fn test_write_ik4() {
        let ik4 = IK4 {
            ik401_position_in_segment: "1".to_string(),
            ik402_data_element_reference_number: "66".to_string(),
            ik403_implementation_data_element_syntax_error_code: "1".to_string(),
            ik404_copy_of_bad_data_element: "123".to_string(),
        };
        
        let ik4_content = write_ik4(ik4);
        assert_eq!(ik4_content, "IK4*1*66*1*123~");
    }
    
    #[test]
    fn test_write_ik4_minimal() {
        let ik4 = IK4 {
            ik401_position_in_segment: "1".to_string(),
            ik402_data_element_reference_number: "66".to_string(),
            ik403_implementation_data_element_syntax_error_code: "1".to_string(),
            ik404_copy_of_bad_data_element: "".to_string(),
        };
        
        let ik4_content = write_ik4(ik4);
        assert_eq!(ik4_content, "IK4*1*66*1~");
    }
    
    #[test]
    fn test_write_ik4_empty() {
        let ik4 = IK4::default();
        
        let ik4_content = write_ik4(ik4);
        assert_eq!(ik4_content, "IK4*1*0*1~");
    }
}
