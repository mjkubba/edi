use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010B {
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    pub ref_segments: Vec<REF>,
}

pub fn get_loop2010b(mut contents: String) -> (Loop2010B, String) {
    let mut nm1_segments = NM1::default();
    let mut per_segments = Vec::new();
    let mut ref_segments = Vec::new();
    
    if contents.contains("NM1") {
        // Check if this is a Requester Name NM1 segment (NM101=1P)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && (nm1_parts[0] == "1P" || nm1_parts[0] == "FA") {
            info!("NM1 segment found for Requester Name, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse PER segments
            while contents.contains("PER") && check_if_segement_in_loop("PER", "HL", contents.clone()) {
                info!("PER segment found, ");
                let per_segment = get_per(get_segment_contents("PER", &contents));
                info!("PER segment parsed");
                
                per_segments.push(per_segment);
                contents = content_trim("PER", contents);
            }
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "HL", contents.clone()) {
                info!("REF segment found, ");
                let ref_segment = get_ref(get_segment_contents("REF", &contents));
                info!("REF segment parsed");
                
                ref_segments.push(ref_segment);
                contents = content_trim("REF", contents);
            }
        }
    }
    
    info!("Loop 2010B parsed\n");
    
    let loop2010b = Loop2010B {
        nm1_segments,
        per_segments,
        ref_segments,
    };
    
    return (loop2010b, contents)
}

pub fn write_loop2010b(loop2010b: Loop2010B) -> String {
    let mut contents = String::new();
    contents.push_str(&write_nm1(loop2010b.nm1_segments));
    
    for per in loop2010b.per_segments {
        contents.push_str(&write_per(per));
    }
    
    for ref_segment in loop2010b.ref_segments {
        contents.push_str(&write_ref(ref_segment));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2010b() {
        let contents = String::from("NM1*1P*2*BONE AND JOINT CLINIC*****SV*2000035~REF*XZ*7654321~");
        let (loop2010b, contents) = get_loop2010b(contents);
        assert_eq!(loop2010b.nm1_segments.entity_id, "1P");
        assert_eq!(loop2010b.nm1_segments.entity_type, "2");
        assert_eq!(loop2010b.nm1_segments.lastname, "BONE AND JOINT CLINIC");
        assert_eq!(loop2010b.nm1_segments.id_code, "SV");
        assert_eq!(loop2010b.nm1_segments.id_code, "2000035");
        
        assert_eq!(loop2010b.ref_segments.len(), 1);
        assert_eq!(loop2010b.ref_segments[0].reference_id_number_qualifier, "XZ");
        assert_eq!(loop2010b.ref_segments[0].reference_id_number, "7654321");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2010b() {
        let loop2010b = Loop2010B {
            nm1_segments: NM1 {
                entity_id: "1P".to_string(),
                entity_type: "2".to_string(),
                lastname: "BONE AND JOINT CLINIC".to_string(),
                firstname: "".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "2000035".to_string(),
                member_number: "".to_string(),
            },
            per_segments: Vec::new(),
            ref_segments: vec![
                REF {
                    reference_id_number_qualifier: "XZ".to_string(),
                    reference_id_number: "7654321".to_string(),
                }
            ],
        };
        
        let contents = write_loop2010b(loop2010b);
        assert!(contents.contains("NM1*1P*2*BONE AND JOINT CLINIC"));
        assert!(contents.contains("REF*XZ*7654321"));
    }
}
