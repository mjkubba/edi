use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TRN {
    pub trace_type_code: String,
    pub reference_id: String,
    pub originating_company_id: String,
    pub trn04_reference_id: String,
}

pub fn get_trn(trn_content: String) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    
    let mut trn = TRN::default();
    
    // TRN01 - Trace Type Code
    if !trn_parts.is_empty() && !trn_parts[0].is_empty() {
        trn.trace_type_code = trn_parts[0].to_string();
    }
    
    // TRN02 - Reference Identification
    if trn_parts.len() > 1 && !trn_parts[1].is_empty() {
        trn.reference_id = trn_parts[1].to_string();
    }
    
    // TRN03 - Originating Company Identifier
    if trn_parts.len() > 2 && !trn_parts[2].is_empty() {
        trn.originating_company_id = trn_parts[2].to_string();
    }
    
    // TRN04 - Reference Identification
    if trn_parts.len() > 3 && !trn_parts[3].is_empty() {
        trn.trn04_reference_id = trn_parts[3].to_string();
    }
    
    info!("Parsed TRN segment: {:?}", trn);
    trn
}

pub fn write_trn(trn: TRN) -> String {
    let mut trn_content = String::new();
    
    trn_content.push_str("TRN*");
    trn_content.push_str(&trn.trace_type_code);
    trn_content.push_str("*");
    trn_content.push_str(&trn.reference_id);
    
    // Only include non-empty fields
    if !trn.originating_company_id.is_empty() {
        trn_content.push_str("*");
        trn_content.push_str(&trn.originating_company_id);
        
        if !trn.trn04_reference_id.is_empty() {
            trn_content.push_str("*");
            trn_content.push_str(&trn.trn04_reference_id);
        }
    }
    
    trn_content.push_str("~");
    trn_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_trn() {
        let trn_content = "1*12345*1512345678*REF123".to_string();
        let trn = get_trn(trn_content);
        
        assert_eq!(trn.trace_type_code, "1");
        assert_eq!(trn.reference_id, "12345");
        assert_eq!(trn.originating_company_id, "1512345678");
        assert_eq!(trn.trn04_reference_id, "REF123");
    }
    
    #[test]
    fn test_get_trn_minimal() {
        let trn_content = "1*12345".to_string();
        let trn = get_trn(trn_content);
        
        assert_eq!(trn.trace_type_code, "1");
        assert_eq!(trn.reference_id, "12345");
        assert_eq!(trn.originating_company_id, "");
        assert_eq!(trn.trn04_reference_id, "");
    }
    
    #[test]
    fn test_write_trn() {
        let trn = TRN {
            trace_type_code: "1".to_string(),
            reference_id: "12345".to_string(),
            originating_company_id: "1512345678".to_string(),
            trn04_reference_id: "REF123".to_string(),
        };
        
        let trn_content = write_trn(trn);
        assert_eq!(trn_content, "TRN*1*12345*1512345678*REF123~");
    }
    
    #[test]
    fn test_write_trn_minimal() {
        let trn = TRN {
            trace_type_code: "1".to_string(),
            reference_id: "12345".to_string(),
            originating_company_id: "".to_string(),
            trn04_reference_id: "".to_string(),
        };
        
        let trn_content = write_trn(trn);
        assert_eq!(trn_content, "TRN*1*12345~");
    }
}
