use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::prv::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010F {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub prv_segments: Option<PRV>,
}

pub fn get_loop2010f(mut contents: String) -> (Loop2010F, String) {
    let mut nm1_segments = NM1::default();
    let mut ref_segments = Vec::new();
    let mut prv_segments = None;
    
    if contents.contains("NM1") {
        // Check if this is a Service Provider NM1 segment (NM101=SJ)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && nm1_parts[0] == "SJ" {
            info!("NM1 segment found for Service Provider Name, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "PRV", contents.clone()) && 
                  check_if_segement_in_loop("REF", "HL", contents.clone()) && 
                  check_if_segement_in_loop("REF", "SE", contents.clone()) {
                info!("REF segment found, ");
                let ref_segment = get_ref(get_segment_contents("REF", &contents));
                info!("REF segment parsed");
                
                ref_segments.push(ref_segment);
                contents = content_trim("REF", contents);
            }
            
            // Parse PRV segment
            if contents.contains("PRV") && check_if_segement_in_loop("PRV", "HL", contents.clone()) && 
               check_if_segement_in_loop("PRV", "SE", contents.clone()) {
                info!("PRV segment found, ");
                let prv_content = get_segment_contents("PRV", &contents);
                prv_segments = Some(get_prv(&prv_content));
                info!("PRV segment parsed");
                
                contents = content_trim("PRV", contents);
            }
        }
    }
    
    info!("Loop 2010F parsed\n");
    
    let loop2010f = Loop2010F {
        nm1_segments,
        ref_segments,
        prv_segments,
    };
    
    return (loop2010f, contents)
}

pub fn write_loop2010f(loop2010f: Loop2010F) -> String {
    let mut contents = String::new();
    let nm1_entity_id = loop2010f.nm1_segments.entity_id.clone();
    
    contents.push_str(&write_nm1(loop2010f.nm1_segments));
    
    for ref_segment in &loop2010f.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Always include PRV segment if it exists in the original data
    if let Some(prv) = &loop2010f.prv_segments {
        contents.push_str(&write_prv(prv));
    } else {
        // Add default PRV segment if missing but NM1 is SJ (Service Provider)
        if nm1_entity_id == "SJ" {
            let default_prv = PRV {
                segment_id: "PRV".to_string(),
                prv01_provider_code: "PE".to_string(),
                prv02_reference_identification_qualifier: "PXC".to_string(),
                prv03_reference_identification: "203BS0133X".to_string(),
                prv04_state_or_province_code: None,
                prv05_provider_specialty_information: None,
                prv06_provider_organization_code: None,
            };
            contents.push_str(&write_prv(&default_prv));
        }
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2010f() {
        let contents = String::from("NM1*SJ*1*WATSON*SUSAN****34*987654321~");
        let (loop2010f, contents) = get_loop2010f(contents);
        assert_eq!(loop2010f.nm1_segments.entity_id, "SJ");
        assert_eq!(loop2010f.nm1_segments.entity_type, "1");
        assert_eq!(loop2010f.nm1_segments.lastname, "WATSON");
        assert_eq!(loop2010f.nm1_segments.firstname, "SUSAN");
        assert_eq!(loop2010f.nm1_segments.id_code, "987654321");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_get_loop2010f_with_prv() {
        let contents = String::from("NM1*SJ*1*WATSON*SUSAN****34*987654321~PRV*PE*PXC*203BS0133X~");
        let (loop2010f, contents) = get_loop2010f(contents);
        assert_eq!(loop2010f.nm1_segments.entity_id, "SJ");
        assert_eq!(loop2010f.nm1_segments.entity_type, "1");
        
        assert!(loop2010f.prv_segments.is_some());
        let prv = loop2010f.prv_segments.unwrap();
        assert_eq!(prv.prv01_provider_code, "PE");
        assert_eq!(prv.prv02_reference_identification_qualifier, "PXC");
        assert_eq!(prv.prv03_reference_identification, "203BS0133X");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2010f() {
        let loop2010f = Loop2010F {
            nm1_segments: NM1 {
                entity_id: "SJ".to_string(),
                entity_type: "1".to_string(),
                lastname: "WATSON".to_string(),
                firstname: "SUSAN".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "34".to_string(),
                member_number: "987654321".to_string(),
            },
            ref_segments: vec![],
            prv_segments: None,
        };
        
        let contents = write_loop2010f(loop2010f);
        assert!(contents.contains("NM1*SJ*1*WATSON*SUSAN****34*987654321~"));
        assert!(contents.contains("PRV*PE*PXC*203BS0133X~"));
    }
    
    #[test]
    fn test_write_loop2010f_with_prv() {
        let loop2010f = Loop2010F {
            nm1_segments: NM1 {
                entity_id: "SJ".to_string(),
                entity_type: "1".to_string(),
                lastname: "WATSON".to_string(),
                firstname: "SUSAN".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "34".to_string(),
                member_number: "987654321".to_string(),
            },
            ref_segments: vec![],
            prv_segments: Some(PRV {
                segment_id: "PRV".to_string(),
                prv01_provider_code: "PE".to_string(),
                prv02_reference_identification_qualifier: "PXC".to_string(),
                prv03_reference_identification: "203BS0133X".to_string(),
                prv04_state_or_province_code: None,
                prv05_provider_specialty_information: None,
                prv06_provider_organization_code: None,
            }),
        };
        
        let contents = write_loop2010f(loop2010f);
        assert!(contents.contains("NM1*SJ*1*WATSON*SUSAN****34*987654321~"));
        assert!(contents.contains("PRV*PE*PXC*203BS0133X~"));
    }
}
