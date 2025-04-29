use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MSG {
    pub msg01_free_form_message_text: String,
    pub msg02_printer_carriage_control_code: String,
    pub msg03_number: String,
}

pub fn get_msg(msg_content: String) -> MSG {
    let msg_parts: Vec<&str> = msg_content.split("*").collect();
    
    // Ensure we have at least one part (the segment ID)
    if msg_parts.is_empty() {
        return MSG::default();
    }
    
    // Extract fields with bounds checking, skipping the segment ID
    let msg01_free_form_message_text = if msg_parts.len() > 1 { msg_parts[1].to_string() } else { String::new() };
    let msg02_printer_carriage_control_code = if msg_parts.len() > 2 { msg_parts[2].to_string() } else { String::new() };
    let msg03_number = if msg_parts.len() > 3 { msg_parts[3].to_string() } else { String::new() };
    
    let msg = MSG {
        msg01_free_form_message_text,
        msg02_printer_carriage_control_code,
        msg03_number,
    };
    
    info!("Parsed MSG segment: {:?}", msg);
    msg
}

pub fn write_msg(msg: MSG) -> String {
    let mut msg_content = String::new();
    
    msg_content.push_str("MSG*");
    msg_content.push_str(&msg.msg01_free_form_message_text);
    
    // Include MSG02 if not empty
    if !msg.msg02_printer_carriage_control_code.is_empty() {
        msg_content.push_str("*");
        msg_content.push_str(&msg.msg02_printer_carriage_control_code);
        
        // Include MSG03 if not empty
        if !msg.msg03_number.is_empty() {
            msg_content.push_str("*");
            msg_content.push_str(&msg.msg03_number);
        }
    }
    
    msg_content.push_str("~");
    msg_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_msg() {
        let msg_content = "MSG*PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION".to_string();
        let msg = get_msg(msg_content);
        
        assert_eq!(msg.msg01_free_form_message_text, "PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION");
        assert_eq!(msg.msg02_printer_carriage_control_code, "");
        assert_eq!(msg.msg03_number, "");
    }
    
    #[test]
    fn test_get_msg_with_control_code() {
        let msg_content = "MSG*PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION*NL*1".to_string();
        let msg = get_msg(msg_content);
        
        assert_eq!(msg.msg01_free_form_message_text, "PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION");
        assert_eq!(msg.msg02_printer_carriage_control_code, "NL");
        assert_eq!(msg.msg03_number, "1");
    }
    
    #[test]
    fn test_write_msg() {
        let msg = MSG {
            msg01_free_form_message_text: "PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION".to_string(),
            msg02_printer_carriage_control_code: "NL".to_string(),
            msg03_number: "1".to_string(),
        };
        
        let msg_content = write_msg(msg);
        assert_eq!(msg_content, "MSG*PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION*NL*1~");
    }
    
    #[test]
    fn test_write_msg_minimal() {
        let msg = MSG {
            msg01_free_form_message_text: "PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION".to_string(),
            msg02_printer_carriage_control_code: "".to_string(),
            msg03_number: "".to_string(),
        };
        
        let msg_content = write_msg(msg);
        assert_eq!(msg_content, "MSG*PLEASE CONTACT CUSTOMER SERVICE FOR ADDITIONAL INFORMATION~");
    }
}
