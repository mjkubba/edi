use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::trn::*;
use crate::segments::um::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000E {
    pub hl_segments: HL,
    pub trn_segments: Vec<TRN>,
    pub um_segments: Option<UM>,
}

pub fn get_loop2000e(mut contents: String) -> (Loop2000E, String) {
    let mut hl_segments = HL::default();
    let mut trn_segments = Vec::new();
    let mut um_segments = None;
    
    if contents.contains("HL") {
        // Check if this is a Service Level HL segment (HL03=EV or HL03=SS)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && (hl_parts[2] == "EV" || hl_parts[2] == "SS") {
            info!("HL segment found for Service Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
            
            // Parse TRN segments
            while contents.contains("TRN") && check_if_segement_in_loop("TRN", "UM", contents.clone()) {
                info!("TRN segment found, ");
                let trn_segment = get_trn(get_segment_contents("TRN", &contents));
                info!("TRN segment parsed");
                
                trn_segments.push(trn_segment);
                contents = content_trim("TRN", contents);
            }
            
            // Parse UM segment
            if contents.contains("UM") {
                info!("UM segment found, ");
                let um_content = get_segment_contents("UM", &contents);
                um_segments = Some(get_um(um_content));
                info!("Parsed UM segment: {:?}", um_segments);
                
                contents = content_trim("UM", contents);
            }
        }
    }
    
    info!("Loop 2000E parsed\n");
    
    let loop2000e = Loop2000E {
        hl_segments,
        trn_segments,
        um_segments,
    };
    
    return (loop2000e, contents)
}

pub fn write_loop2000e(loop2000e: Loop2000E) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000e.hl_segments));
    
    for trn in &loop2000e.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    if let Some(um) = loop2000e.um_segments {
        contents.push_str(&write_um(um));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000e_with_full_um() {
        let contents = String::from("HL*5*4*SS*0~TRN*1*12345*1512345678~UM*AR*I*2*21:B*****Y~");
        let (loop2000e, contents) = get_loop2000e(contents);
        assert_eq!(loop2000e.hl_segments.hl01_hierarchical_id_number, "5");
        assert_eq!(loop2000e.hl_segments.hl02_hierarchical_parent_id_number, "4");
        assert_eq!(loop2000e.hl_segments.hl03_hierarchical_level_code, "SS");
        assert_eq!(loop2000e.hl_segments.hl04_hierarchical_child_code, "0");
        
        assert_eq!(loop2000e.trn_segments.len(), 1);
        assert_eq!(loop2000e.trn_segments[0].trace_type_code, "1");
        assert_eq!(loop2000e.trn_segments[0].reference_id, "12345");
        assert_eq!(loop2000e.trn_segments[0].originating_company_id, "1512345678");
        
        assert!(loop2000e.um_segments.is_some());
        let um = loop2000e.um_segments.unwrap();
        assert_eq!(um.um01_request_category_code, "AR");
        assert_eq!(um.um02_certification_type_code, "I");
        assert_eq!(um.um03_service_type_code, "2");
        assert_eq!(um.um04_health_care_service_location_information, "21:B");
        assert_eq!(um.um08_prognosis_code, "Y");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_get_loop2000e_with_minimal_um() {
        let contents = String::from("HL*5*4*SS*0~TRN*1*12345*1512345678~UM*HS*I*2~");
        let (loop2000e, contents) = get_loop2000e(contents);
        
        assert!(loop2000e.um_segments.is_some());
        let um = loop2000e.um_segments.unwrap();
        assert_eq!(um.um01_request_category_code, "HS");
        assert_eq!(um.um02_certification_type_code, "I");
        assert_eq!(um.um03_service_type_code, "2");
        assert_eq!(um.um04_health_care_service_location_information, "");
        assert_eq!(um.um08_prognosis_code, "");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2000e() {
        let loop2000e = Loop2000E {
            hl_segments: HL {
                hl01_hierarchical_id_number: "5".to_string(),
                hl02_hierarchical_parent_id_number: "4".to_string(),
                hl03_hierarchical_level_code: "SS".to_string(),
                hl04_hierarchical_child_code: "0".to_string(),
            },
            trn_segments: vec![
                TRN {
                    trace_type_code: "1".to_string(),
                    reference_id: "12345".to_string(),
                    originating_company_id: "1512345678".to_string(),
                    trn04_reference_id: "".to_string(),
                }
            ],
            um_segments: Some(UM {
                um00_request_category_code_prefix: "".to_string(),
                um01_request_category_code: "AR".to_string(),
                um02_certification_type_code: "I".to_string(),
                um03_service_type_code: "2".to_string(),
                um04_health_care_service_location_information: "21:B".to_string(),
                um05_related_causes_information: "".to_string(),
                um06_level_of_service_code: "".to_string(),
                um07_current_health_condition_code: "".to_string(),
                um08_prognosis_code: "Y".to_string(),
                um09_release_of_information_code: "".to_string(),
                um10_delay_reason_code: "".to_string(),
            }),
        };
        
        let contents = write_loop2000e(loop2000e);
        assert!(contents.contains("HL*5*4*SS*0"));
        assert!(contents.contains("TRN*1*12345*1512345678"));
        assert!(contents.contains("UM*AR*I*2*21:B****Y"));
    }
}
