use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CL1 {
    pub cl101_admission_type_code: String,
    pub cl102_admission_source_code: String,
    pub cl103_patient_status_code: String,
}

pub fn get_cl1(cl1_content: String) -> CL1 {
    let cl1_parts: Vec<&str> = cl1_content.split("*").collect();
    
    let mut cl1 = CL1::default();
    
    if cl1_parts.len() > 0 && !cl1_parts[0].is_empty() {
        cl1.cl101_admission_type_code = cl1_parts[0].to_string();
    }
    
    if cl1_parts.len() > 1 && !cl1_parts[1].is_empty() {
        cl1.cl102_admission_source_code = cl1_parts[1].to_string();
    }
    
    if cl1_parts.len() > 2 && !cl1_parts[2].is_empty() {
        cl1.cl103_patient_status_code = cl1_parts[2].to_string();
    }
    
    cl1
}

pub fn write_cl1(cl1: CL1) -> String {
    if cl1.cl101_admission_type_code.is_empty() {
        return String::new();
    }
    
    let mut cl1_content = String::new();
    cl1_content.push_str("CL1*");
    cl1_content.push_str(&cl1.cl101_admission_type_code);
    
    if !cl1.cl102_admission_source_code.is_empty() || !cl1.cl103_patient_status_code.is_empty() {
        cl1_content.push_str("*");
        cl1_content.push_str(&cl1.cl102_admission_source_code);
        
        if !cl1.cl103_patient_status_code.is_empty() {
            cl1_content.push_str("*");
            cl1_content.push_str(&cl1.cl103_patient_status_code);
        }
    }
    
    cl1_content.push_str("~");
    cl1_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_cl1() {
        let cl1_content = "2".to_string();
        let cl1 = get_cl1(cl1_content);
        assert_eq!(cl1.cl101_admission_type_code, "2");
        assert_eq!(cl1.cl102_admission_source_code, "");
        assert_eq!(cl1.cl103_patient_status_code, "");
    }
    
    #[test]
    fn test_write_cl1() {
        let cl1 = CL1 {
            cl101_admission_type_code: "2".to_string(),
            cl102_admission_source_code: "".to_string(),
            cl103_patient_status_code: "".to_string(),
        };
        
        let cl1_content = write_cl1(cl1);
        assert_eq!(cl1_content, "CL1*2~");
    }
    
    #[test]
    fn test_write_cl1_with_all_fields() {
        let cl1 = CL1 {
            cl101_admission_type_code: "2".to_string(),
            cl102_admission_source_code: "7".to_string(),
            cl103_patient_status_code: "01".to_string(),
        };
        
        let cl1_content = write_cl1(cl1);
        assert_eq!(cl1_content, "CL1*2*7*01~");
    }
}
