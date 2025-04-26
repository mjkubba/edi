use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::per::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl_segments: HL,
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
}

pub fn get_loop_2000a(mut contents: String) -> EdiResult<(Loop2000A, String)> {
    let mut loop2000a = Loop2000A::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents).map_err(|_| EdiError::MissingSegment("HL".to_string()))?;
        loop2000a.hl_segments = get_hl(hl_content)?;
        
        // Verify this is an Information Source level HL segment (level code = 20)
        if loop2000a.hl_segments.hl03_hierarchical_level_code != "20" {
            return Err(EdiError::ValidationError(format!(
                "Expected HL03 code '20' for Information Source level, got '{}'",
                loop2000a.hl_segments.hl03_hierarchical_level_code
            )));
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents)?;
    } else {
        return Err(EdiError::MissingSegment("HL".to_string()));
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents).map_err(|_| EdiError::MissingSegment("NM1".to_string()))?;
        loop2000a.nm1_segments = get_nm1(nm1_content)?;
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents)?;
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found");
        let per_content = get_segment_contents("PER", &contents).map_err(|_| EdiError::MissingSegment("PER".to_string()))?;
        let per = get_per(per_content)?;
        info!("PER segment parsed");
        loop2000a.per_segments.push(per);
        contents = content_trim("PER", contents)?;
    }
    
    info!("Loop 2000A parsed");
    Ok((loop2000a, contents))
}

pub fn write_loop_2000a(loop2000a: &Loop2000A) -> EdiResult<String> {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(&loop2000a.hl_segments));
    
    // Write NM1 segment
    contents.push_str(&write_nm1(&loop2000a.nm1_segments));
    
    // Write all PER segments
    for per in &loop2000a.per_segments {
        contents.push_str(&write_per(per));
    }
    
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_2000a() -> EdiResult<()> {
        let contents = "HL*1**20*1~NM1*PR*2*ABC INSURANCE*****PI*12345~PER*IC*JANE DOE*TE*1234567890~".to_string();
        let (loop2000a, remaining) = get_loop_2000a(contents)?;
        
        assert_eq!(loop2000a.hl_segments.hl01_hierarchical_id_number, "1");
        assert_eq!(loop2000a.hl_segments.hl02_hierarchical_parent_id_number, "");
        assert_eq!(loop2000a.hl_segments.hl03_hierarchical_level_code, "20");
        assert_eq!(loop2000a.hl_segments.hl04_hierarchical_child_code, "1");
        
        assert_eq!(loop2000a.nm1_segments.entity_id, "PR");
        assert_eq!(loop2000a.nm1_segments.entity_type, "2");
        assert_eq!(loop2000a.nm1_segments.lastname, "ABC INSURANCE");
        
        assert_eq!(loop2000a.per_segments.len(), 1);
        assert_eq!(loop2000a.per_segments[0].per01_contact_function_code, "IC");
        assert_eq!(loop2000a.per_segments[0].per02_contact_name, "JANE DOE");
        
        assert_eq!(remaining, "");
        
        Ok(())
    }
    
    #[test]
    fn test_write_loop_2000a() -> EdiResult<()> {
        let loop2000a = Loop2000A {
            hl_segments: HL {
                hl01_hierarchical_id_number: "1".to_string(),
                hl02_hierarchical_parent_id_number: "".to_string(),
                hl03_hierarchical_level_code: "20".to_string(),
                hl04_hierarchical_child_code: "1".to_string(),
            },
            nm1_segments: NM1 {
                entity_id: "PR".to_string(),
                entity_type: "2".to_string(),
                lastname: "ABC INSURANCE".to_string(),
                firstname: "".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "PI".to_string(),
                member_number: "12345".to_string(),
            },
            per_segments: vec![
                PER {
                    per01_contact_function_code: "IC".to_string(),
                    per02_contact_name: "JANE DOE".to_string(),
                    per03_contact_number_qualifier: "TE".to_string(),
                    per04_contact_number: "1234567890".to_string(),
                    per05_contact_number_qualifier: "".to_string(),
                    per06_contact_number: "".to_string(),
                    per07_contact_number_qualifier: "".to_string(),
                    per08_contact_number: "".to_string(),
                }
            ],
        };
        
        let contents = write_loop_2000a(&loop2000a)?;
        assert!(contents.contains("HL*1**20*1~"));
        assert!(contents.contains("NM1*PR*2*ABC INSURANCE*****PI*12345~"));
        assert!(contents.contains("PER*IC*JANE DOE*TE*1234567890~"));
        
        Ok(())
    }
}
