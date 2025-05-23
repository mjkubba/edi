use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct REF{
    pub reference_id_number_qualifier: String,
    pub reference_id_number: String,
}

pub fn get_ref(ref_content: String) -> REF {
    let ref_parts: Vec<&str> = ref_content.split("*").collect();
    
    // Ensure we have at least one part (the segment ID)
    if ref_parts.is_empty() {
        return REF::default();
    }
    
    // Check if the first part is the segment ID "REF"
    let start_index = if ref_parts[0] == "REF" { 1 } else { 0 };
    
    // Extract the qualifier and reference number, skipping the segment ID if present
    let reference_id_number_qualifier = if ref_parts.len() > start_index { 
        ref_parts[start_index].to_string() 
    } else { 
        String::new() 
    };
    
    let reference_id_number = if ref_parts.len() > start_index + 1 { 
        ref_parts[start_index + 1].to_string() 
    } else { 
        String::new() 
    };
    
    REF {
        reference_id_number_qualifier,
        reference_id_number,
    }
}

pub fn write_ref(rref:REF) -> String {
    if rref.reference_id_number_qualifier.is_empty() {
        return String::new();
    }
    let mut ref_content = String::new();
    ref_content.push_str("REF*");
    ref_content.push_str(&rref.reference_id_number_qualifier);
    
    // Add reference identification if present
    if !rref.reference_id_number.is_empty() {
        ref_content.push_str("*");
        ref_content.push_str(&rref.reference_id_number);
    }
    
    ref_content.push_str("~");
    ref_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ref() {
        let ref_content = "SY*123456789".to_string();
        let ref_segment = get_ref(ref_content);
        
        assert_eq!(ref_segment.reference_id_number_qualifier, "SY");
        assert_eq!(ref_segment.reference_id_number, "123456789");
    }
    
    #[test]
    fn test_write_ref() {
        let ref_segment = REF {
            reference_id_number_qualifier: "SY".to_string(),
            reference_id_number: "123456789".to_string(),
        };
        
        let ref_content = write_ref(ref_segment);
        assert_eq!(ref_content, "REF*SY*123456789~");
    }
}
