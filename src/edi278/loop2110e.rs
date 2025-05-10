use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::prv::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2110E {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub prv_segments: Option<PRV>,
}

pub fn get_loop2110e(mut contents: String) -> (Loop2110E, String) {
    let mut nm1_segments = NM1::default();
    let mut ref_segments = Vec::new();
    let mut prv_segments = None;
    
    if contents.contains("NM1") {
        // Check if this is a Service Provider NM1 segment (NM101=71 or NM101=72 or NM101=77 or NM101=AAJ)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && (nm1_parts[0] == "71" || nm1_parts[0] == "72" || nm1_parts[0] == "77" || nm1_parts[0] == "AAJ") {
            info!("NM1 segment found for Service Provider, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "PRV", contents.clone()) && check_if_segement_in_loop("REF", "NM1", contents.clone()) {
                info!("REF segment found, ");
                let ref_segment = get_ref(get_segment_contents("REF", &contents));
                info!("REF segment parsed");
                
                ref_segments.push(ref_segment);
                contents = content_trim("REF", contents);
            }
            
            // Parse PRV segment
            if contents.contains("PRV") && check_if_segement_in_loop("PRV", "NM1", contents.clone()) {
                info!("PRV segment found, ");
                prv_segments = Some(get_prv(get_segment_contents("PRV", &contents)));
                info!("PRV segment parsed");
                
                contents = content_trim("PRV", contents);
            }
        }
    }
    
    info!("Loop 2110E parsed\n");
    
    let loop2110e = Loop2110E {
        nm1_segments,
        ref_segments,
        prv_segments,
    };
    
    return (loop2110e, contents)
}

pub fn write_loop2110e(loop2110e: Loop2110E) -> String {
    let mut contents = String::new();
    contents.push_str(&write_nm1(loop2110e.nm1_segments));
    
    for ref_segment in loop2110e.ref_segments {
        contents.push_str(&write_ref(ref_segment));
    }
    
    if let Some(prv) = loop2110e.prv_segments {
        contents.push_str(&write_prv(prv));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2110e() {
        let contents = String::from("NM1*71*1*SMITH*JOHN*A***XX*1234567890~REF*1J*12345~PRV*PE*ZZ*207Q00000X~");
        let (loop2110e, contents) = get_loop2110e(contents);
        assert_eq!(loop2110e.nm1_segments.entity_id, "71");
        assert_eq!(loop2110e.nm1_segments.entity_type, "1");
        assert_eq!(loop2110e.nm1_segments.lastname, "SMITH");
        assert_eq!(loop2110e.nm1_segments.firstname, "JOHN");
        assert_eq!(loop2110e.nm1_segments.id_code_qualifier, "XX");
        assert_eq!(loop2110e.nm1_segments.id_code, "1234567890");
        
        assert_eq!(loop2110e.ref_segments.len(), 1);
        assert_eq!(loop2110e.ref_segments[0].reference_id_number_qualifier, "1J");
        assert_eq!(loop2110e.ref_segments[0].reference_id_number, "12345");
        
        assert!(loop2110e.prv_segments.is_some());
        let prv = loop2110e.prv_segments.unwrap();
        assert_eq!(prv.provider_code, "PE");
        assert_eq!(prv.reference_identification_qualifier, "ZZ");
        assert_eq!(prv.reference_identification, "207Q00000X");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2110e() {
        let loop2110e = Loop2110E {
            nm1_segments: NM1 {
                entity_id: "71".to_string(),
                entity_type: "1".to_string(),
                lastname: "SMITH".to_string(),
                firstname: "JOHN".to_string(),
                middle_initial: "A".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code_qualifier: "XX".to_string(),
                id_code: "1234567890".to_string(),
                member_number: "".to_string(),
            },
            ref_segments: vec![
                REF {
                    reference_id_number_qualifier: "1J".to_string(),
                    reference_id_number: "12345".to_string(),
                    description: "".to_string(),
                    reference_identifier: "".to_string(),
                }
            ],
            prv_segments: Some(PRV {
                provider_code: "PE".to_string(),
                reference_identification_qualifier: "ZZ".to_string(),
                reference_identification: "207Q00000X".to_string(),
            }),
        };
        
        let contents = write_loop2110e(loop2110e);
        assert!(contents.contains("NM1*71*1*SMITH*JOHN*A"));
        assert!(contents.contains("REF*1J*12345"));
        assert!(contents.contains("PRV*PE*ZZ*207Q00000X"));
    }
}
