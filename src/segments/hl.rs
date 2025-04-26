use log::info;
use serde::{Serialize, Deserialize};
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HL {
    pub hl01_hierarchical_id_number: String,
    pub hl02_hierarchical_parent_id_number: String,
    pub hl03_hierarchical_level_code: String,
    pub hl04_hierarchical_child_code: String,
}

pub fn get_hl(hl_content: String) -> EdiResult<HL> {
    let hl_parts: Vec<&str> = hl_content.split("*").collect();
    
    if hl_parts.len() < 3 {
        return Err(EdiError::MalformedSegment("HL".to_string()));
    }
    
    let mut hl = HL::default();
    
    // HL01 - Hierarchical ID Number
    if !hl_parts[0].is_empty() {
        hl.hl01_hierarchical_id_number = hl_parts[0].to_string();
    }
    
    // HL02 - Hierarchical Parent ID Number (Situational)
    if hl_parts.len() > 1 && !hl_parts[1].is_empty() {
        hl.hl02_hierarchical_parent_id_number = hl_parts[1].to_string();
    }
    
    // HL03 - Hierarchical Level Code
    if hl_parts.len() > 2 && !hl_parts[2].is_empty() {
        hl.hl03_hierarchical_level_code = hl_parts[2].to_string();
    }
    
    // HL04 - Hierarchical Child Code (Situational)
    if hl_parts.len() > 3 && !hl_parts[3].is_empty() {
        hl.hl04_hierarchical_child_code = hl_parts[3].to_string();
    }
    
    info!("Parsed HL segment: {:?}", hl);
    Ok(hl)
}

pub fn write_hl(hl: &HL) -> String {
    let mut hl_content = String::new();
    
    hl_content.push_str("HL*");
    hl_content.push_str(&hl.hl01_hierarchical_id_number);
    
    // Always include HL02, even if empty
    hl_content.push_str("*");
    hl_content.push_str(&hl.hl02_hierarchical_parent_id_number);
    
    // Always include HL03
    hl_content.push_str("*");
    hl_content.push_str(&hl.hl03_hierarchical_level_code);
    
    // Only include HL04 if not empty
    if !hl.hl04_hierarchical_child_code.is_empty() {
        hl_content.push_str("*");
        hl_content.push_str(&hl.hl04_hierarchical_child_code);
    }
    
    hl_content.push_str("~");
    hl_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_hl() -> EdiResult<()> {
        let hl_content = "1**20*1~".to_string();
        let hl = get_hl(hl_content)?;
        
        assert_eq!(hl.hl01_hierarchical_id_number, "1");
        assert_eq!(hl.hl02_hierarchical_parent_id_number, "");
        assert_eq!(hl.hl03_hierarchical_level_code, "20");
        assert_eq!(hl.hl04_hierarchical_child_code, "1");
        
        Ok(())
    }
    
    #[test]
    fn test_write_hl() {
        let hl = HL {
            hl01_hierarchical_id_number: "1".to_string(),
            hl02_hierarchical_parent_id_number: "".to_string(),
            hl03_hierarchical_level_code: "20".to_string(),
            hl04_hierarchical_child_code: "1".to_string(),
        };
        
        let hl_content = write_hl(&hl);
        assert_eq!(hl_content, "HL*1**20*1~");
    }
    
    #[test]
    fn test_write_hl_minimal() {
        let hl = HL {
            hl01_hierarchical_id_number: "1".to_string(),
            hl02_hierarchical_parent_id_number: "".to_string(),
            hl03_hierarchical_level_code: "20".to_string(),
            hl04_hierarchical_child_code: "".to_string(),
        };
        
        let hl_content = write_hl(&hl);
        assert_eq!(hl_content, "HL*1**20~");
    }
}
