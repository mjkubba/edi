use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SE {
    pub number_of_segment: String,
    pub transaction_set_control_number: String,
}

pub fn get_se(se_content: String) -> SE {
    let se_parts: Vec<&str> = se_content.split("*").collect();
    
    let mut se = SE::default();
    
    // SE01 - Number of Included Segments
    if se_parts.len() > 0 && !se_parts[0].is_empty() {
        se.number_of_segment = se_parts[0].to_string();
    }
    
    // SE02 - Transaction Set Control Number
    if se_parts.len() > 1 && !se_parts[1].is_empty() {
        se.transaction_set_control_number = se_parts[1].to_string();
    }
    
    info!("Parsed SE segment: {:?}", se);
    se
}

pub fn write_se(se: SE) -> String {
    let mut se_content = String::new();
    
    se_content.push_str("SE*");
    se_content.push_str(&se.number_of_segment);
    se_content.push_str("*");
    se_content.push_str(&se.transaction_set_control_number);
    se_content.push_str("~");
    
    se_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_se() {
        let se_content = "10*0001".to_string();
        let se = get_se(se_content);
        
        assert_eq!(se.number_of_segment, "10");
        assert_eq!(se.transaction_set_control_number, "0001");
    }
    
    #[test]
    fn test_write_se() {
        let se = SE {
            number_of_segment: "10".to_string(),
            transaction_set_control_number: "0001".to_string(),
        };
        
        let se_content = write_se(se);
        assert_eq!(se_content, "SE*10*0001~");
    }
}
