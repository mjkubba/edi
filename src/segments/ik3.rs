use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK3 {
    pub ik301_segment_id_code: String,
    pub ik302_segment_position_in_transaction_set: String,
    pub ik303_loop_identifier_code: String,
    pub ik304_implementation_segment_syntax_error_code: String,
}

pub fn get_ik3(ik3_content: String) -> IK3 {
    let ik3_parts: Vec<&str> = ik3_content.split("*").collect();
    
    let mut ik3 = IK3::default();
    
    // IK301 - Segment ID Code (Required)
    if !ik3_parts.is_empty() && !ik3_parts[0].is_empty() {
        ik3.ik301_segment_id_code = ik3_parts[0].to_string();
    }
    
    // IK302 - Segment Position in Transaction Set (Required)
    if ik3_parts.len() > 1 && !ik3_parts[1].is_empty() {
        ik3.ik302_segment_position_in_transaction_set = ik3_parts[1].to_string();
    }
    
    // IK303 - Loop Identifier Code (Situational)
    if ik3_parts.len() > 2 && !ik3_parts[2].is_empty() {
        ik3.ik303_loop_identifier_code = ik3_parts[2].to_string();
    }
    
    // IK304 - Implementation Segment Syntax Error Code (Situational)
    if ik3_parts.len() > 3 && !ik3_parts[3].is_empty() {
        ik3.ik304_implementation_segment_syntax_error_code = ik3_parts[3].to_string();
    }
    
    info!("Parsed IK3 segment: {:?}", ik3);
    ik3
}

pub fn write_ik3(ik3: IK3) -> String {
    let mut ik3_content = String::new();
    
    ik3_content.push_str("IK3*");
    ik3_content.push_str(&ik3.ik301_segment_id_code);
    ik3_content.push_str("*");
    ik3_content.push_str(&ik3.ik302_segment_position_in_transaction_set);
    
    // Include IK303 (can be empty)
    ik3_content.push_str("*");
    ik3_content.push_str(&ik3.ik303_loop_identifier_code);
    
    // Include IK304 if not empty
    if !ik3.ik304_implementation_segment_syntax_error_code.is_empty() {
        ik3_content.push_str("*");
        ik3_content.push_str(&ik3.ik304_implementation_segment_syntax_error_code);
    }
    
    ik3_content.push_str("~");
    ik3_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ik3() {
        let ik3_content = "NM1*1*8*7".to_string();
        let ik3 = get_ik3(ik3_content);
        
        assert_eq!(ik3.ik301_segment_id_code, "NM1");
        assert_eq!(ik3.ik302_segment_position_in_transaction_set, "1");
        assert_eq!(ik3.ik303_loop_identifier_code, "8");
        assert_eq!(ik3.ik304_implementation_segment_syntax_error_code, "7");
    }
    
    #[test]
    fn test_get_ik3_minimal() {
        let ik3_content = "NM1*1".to_string();
        let ik3 = get_ik3(ik3_content);
        
        assert_eq!(ik3.ik301_segment_id_code, "NM1");
        assert_eq!(ik3.ik302_segment_position_in_transaction_set, "1");
        assert_eq!(ik3.ik303_loop_identifier_code, "");
        assert_eq!(ik3.ik304_implementation_segment_syntax_error_code, "");
    }
    
    #[test]
    fn test_write_ik3() {
        let ik3 = IK3 {
            ik301_segment_id_code: "NM1".to_string(),
            ik302_segment_position_in_transaction_set: "1".to_string(),
            ik303_loop_identifier_code: "8".to_string(),
            ik304_implementation_segment_syntax_error_code: "7".to_string(),
        };
        
        let ik3_content = write_ik3(ik3);
        assert_eq!(ik3_content, "IK3*NM1*1*8*7~");
    }
    
    #[test]
    fn test_write_ik3_minimal() {
        let ik3 = IK3 {
            ik301_segment_id_code: "NM1".to_string(),
            ik302_segment_position_in_transaction_set: "1".to_string(),
            ik303_loop_identifier_code: "".to_string(),
            ik304_implementation_segment_syntax_error_code: "".to_string(),
        };
        
        let ik3_content = write_ik3(ik3);
        assert_eq!(ik3_content, "IK3*NM1*1*~");
    }
}
