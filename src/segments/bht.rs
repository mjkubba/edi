use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BHT {
    pub bht01_hierarchical_structure_code: String,
    pub bht02_transaction_set_purpose_code: String,
    pub bht03_reference_identification: String,
    pub bht04_date: String,
    pub bht05_time: String,
    pub bht06_transaction_type_code: String,
}

pub fn get_bht(bht_content: String) -> BHT {
    let bht_parts: Vec<&str> = bht_content.split("*").collect();
    
    let mut bht = BHT::default();
    
    // BHT01 - Hierarchical Structure Code
    if !bht_parts.is_empty() && !bht_parts[0].is_empty() {
        bht.bht01_hierarchical_structure_code = bht_parts[0].to_string();
    }
    
    // BHT02 - Transaction Set Purpose Code
    if bht_parts.len() > 1 && !bht_parts[1].is_empty() {
        bht.bht02_transaction_set_purpose_code = bht_parts[1].to_string();
    }
    
    // BHT03 - Reference Identification
    if bht_parts.len() > 2 && !bht_parts[2].is_empty() {
        bht.bht03_reference_identification = bht_parts[2].to_string();
    }
    
    // BHT04 - Date
    if bht_parts.len() > 3 && !bht_parts[3].is_empty() {
        bht.bht04_date = bht_parts[3].to_string();
    }
    
    // BHT05 - Time
    if bht_parts.len() > 4 && !bht_parts[4].is_empty() {
        bht.bht05_time = bht_parts[4].to_string();
    }
    
    // BHT06 - Transaction Type Code
    if bht_parts.len() > 5 && !bht_parts[5].is_empty() {
        bht.bht06_transaction_type_code = bht_parts[5].to_string();
    }
    
    info!("Parsed BHT segment: {:?}", bht);
    bht
}

pub fn write_bht(bht: BHT) -> String {
    let mut bht_content = String::new();
    
    bht_content.push_str("BHT*");
    bht_content.push_str(&bht.bht01_hierarchical_structure_code);
    bht_content.push_str("*");
    bht_content.push_str(&bht.bht02_transaction_set_purpose_code);
    bht_content.push_str("*");
    bht_content.push_str(&bht.bht03_reference_identification);
    
    // Include BHT04 if not empty
    if !bht.bht04_date.is_empty() {
        bht_content.push_str("*");
        bht_content.push_str(&bht.bht04_date);
        
        // Include BHT05 if not empty
        if !bht.bht05_time.is_empty() {
            bht_content.push_str("*");
            bht_content.push_str(&bht.bht05_time);
            
            // Include BHT06 if not empty
            if !bht.bht06_transaction_type_code.is_empty() {
                bht_content.push_str("*");
                bht_content.push_str(&bht.bht06_transaction_type_code);
            }
        }
    }
    
    bht_content.push_str("~");
    bht_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_bht() {
        let bht_content = "0022*13*10001234*20060501*1319*00".to_string();
        let bht = get_bht(bht_content);
        
        assert_eq!(bht.bht01_hierarchical_structure_code, "0022");
        assert_eq!(bht.bht02_transaction_set_purpose_code, "13");
        assert_eq!(bht.bht03_reference_identification, "10001234");
        assert_eq!(bht.bht04_date, "20060501");
        assert_eq!(bht.bht05_time, "1319");
        assert_eq!(bht.bht06_transaction_type_code, "00");
    }
    
    #[test]
    fn test_get_bht_minimal() {
        let bht_content = "0022*13*10001234".to_string();
        let bht = get_bht(bht_content);
        
        assert_eq!(bht.bht01_hierarchical_structure_code, "0022");
        assert_eq!(bht.bht02_transaction_set_purpose_code, "13");
        assert_eq!(bht.bht03_reference_identification, "10001234");
        assert_eq!(bht.bht04_date, "");
        assert_eq!(bht.bht05_time, "");
        assert_eq!(bht.bht06_transaction_type_code, "");
    }
    
    #[test]
    fn test_write_bht() {
        let bht = BHT {
            bht01_hierarchical_structure_code: "0022".to_string(),
            bht02_transaction_set_purpose_code: "13".to_string(),
            bht03_reference_identification: "10001234".to_string(),
            bht04_date: "20060501".to_string(),
            bht05_time: "1319".to_string(),
            bht06_transaction_type_code: "00".to_string(),
        };
        
        let bht_content = write_bht(bht);
        assert_eq!(bht_content, "BHT*0022*13*10001234*20060501*1319*00~");
    }
    
    #[test]
    fn test_write_bht_minimal() {
        let bht = BHT {
            bht01_hierarchical_structure_code: "0022".to_string(),
            bht02_transaction_set_purpose_code: "13".to_string(),
            bht03_reference_identification: "10001234".to_string(),
            bht04_date: "".to_string(),
            bht05_time: "".to_string(),
            bht06_transaction_type_code: "".to_string(),
        };
        
        let bht_content = write_bht(bht);
        assert_eq!(bht_content, "BHT*0022*13*10001234~");
    }
}
