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
    
    let mut dtp = DTP::default();
    
    // DTP01 - Date Time Qualifier
    if !dtp_parts.is_empty() && !dtp_parts[0].is_empty() {
        dtp.dtp01_date_time_qualifier = dtp_parts[0].to_string();
    }
    
    // DTP02 - Date Time Format Qualifier
    if dtp_parts.len() > 1 && !dtp_parts[1].is_empty() {
        dtp.dtp02_date_time_format_qualifier = dtp_parts[1].to_string();
    }
    
    // DTP03 - Date Time Value
    if dtp_parts.len() > 2 && !dtp_parts[2].is_empty() {
        dtp.dtp03_date_time_value = dtp_parts[2].to_string();
    }
    
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
        let dtp_content = "291*D8*20220101".to_string();
        let dtp = get_dtp(dtp_content);
        
        assert_eq!(dtp.dtp01_date_time_qualifier, "291");
        assert_eq!(dtp.dtp02_date_time_format_qualifier, "D8");
        assert_eq!(dtp.dtp03_date_time_value, "20220101");
    }
    
    #[test]
    fn test_get_dtp_range() {
        let dtp_content = "348*RD8*20220101-20221231".to_string();
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
