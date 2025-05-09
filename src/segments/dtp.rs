use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTP {
    pub dtp01_date_time_qualifier: String,
    pub dtp02_date_time_format_qualifier: String,
    pub dtp03_date_time_value: String,
}

pub fn get_dtp(dtp_content: String) -> DTP {
    let dtp_parts: Vec<&str> = dtp_content.split("*").collect();
    
    // Ensure we have at least one part (the segment ID)
    if dtp_parts.is_empty() {
        return DTP::default();
    }
    
    // Check if the first part is the segment ID "DTP"
    let start_index = if dtp_parts[0] == "DTP" { 1 } else { 0 };
    
    // Extract fields with bounds checking, skipping the segment ID if present
    let dtp01_date_time_qualifier = if dtp_parts.len() > start_index { dtp_parts[start_index].to_string() } else { String::new() };
    let dtp02_date_time_format_qualifier = if dtp_parts.len() > start_index + 1 { dtp_parts[start_index + 1].to_string() } else { String::new() };
    let dtp03_date_time_value = if dtp_parts.len() > start_index + 2 { dtp_parts[start_index + 2].to_string() } else { String::new() };
    
    let dtp = DTP {
        dtp01_date_time_qualifier,
        dtp02_date_time_format_qualifier,
        dtp03_date_time_value,
    };
    
    info!("Parsed DTP segment: {:?}", dtp);
    dtp
}

pub fn write_dtp(dtp: DTP) -> String {
    let mut dtp_content = String::new();
    
    dtp_content.push_str("DTP*");
    dtp_content.push_str(&dtp.dtp01_date_time_qualifier);
    dtp_content.push_str("*");
    dtp_content.push_str(&dtp.dtp02_date_time_format_qualifier);
    dtp_content.push_str("*");
    dtp_content.push_str(&dtp.dtp03_date_time_value);
    dtp_content.push_str("~");
    
    dtp_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_dtp() {
        let dtp_content = "DTP*291*D8*20220101".to_string();
        let dtp = get_dtp(dtp_content);
        
        assert_eq!(dtp.dtp01_date_time_qualifier, "291");
        assert_eq!(dtp.dtp02_date_time_format_qualifier, "D8");
        assert_eq!(dtp.dtp03_date_time_value, "20220101");
    }
    
    #[test]
    fn test_get_dtp_range() {
        let dtp_content = "DTP*348*RD8*20220101-20221231".to_string();
        let dtp = get_dtp(dtp_content);
        
        assert_eq!(dtp.dtp01_date_time_qualifier, "348");
        assert_eq!(dtp.dtp02_date_time_format_qualifier, "RD8");
        assert_eq!(dtp.dtp03_date_time_value, "20220101-20221231");
    }
    
    #[test]
    fn test_write_dtp() {
        let dtp = DTP {
            dtp01_date_time_qualifier: "291".to_string(),
            dtp02_date_time_format_qualifier: "D8".to_string(),
            dtp03_date_time_value: "20220101".to_string(),
        };
        
        let dtp_content = write_dtp(dtp);
        assert_eq!(dtp_content, "DTP*291*D8*20220101~");
    }
    
    #[test]
    fn test_write_dtp_range() {
        let dtp = DTP {
            dtp01_date_time_qualifier: "348".to_string(),
            dtp02_date_time_format_qualifier: "RD8".to_string(),
            dtp03_date_time_value: "20220101-20221231".to_string(),
        };
        
        let dtp_content = write_dtp(dtp);
        assert_eq!(dtp_content, "DTP*348*RD8*20220101-20221231~");
    }
}
