use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::um::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000F {
    pub hl_segments: HL,
    pub um_segments: Option<UM>,
}

pub fn get_loop2000f(mut contents: String) -> (Loop2000F, String) {
    let mut hl_segments = HL::default();
    let mut um_segments = None;
    
    if contents.contains("HL") {
        // Check if this is a Service Provider Level HL segment (HL03=PT or HL03=SS)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && (hl_parts[2] == "PT" || hl_parts[2] == "SS") {
            info!("HL segment found for Service Provider Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
            
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
    
    info!("Loop 2000F parsed\n");
    
    let loop2000f = Loop2000F {
        hl_segments,
        um_segments,
    };
    
    return (loop2000f, contents)
}

pub fn write_loop2000f(loop2000f: Loop2000F) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000f.hl_segments));
    
    if let Some(um) = loop2000f.um_segments {
        contents.push_str(&write_um(um));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000f_with_hl_pt() {
        let contents = String::from("HL*6*5*PT*0~");
        let (loop2000f, contents) = get_loop2000f(contents);
        assert_eq!(loop2000f.hl_segments.hl01_hierarchical_id_number, "6");
        assert_eq!(loop2000f.hl_segments.hl02_hierarchical_parent_id_number, "5");
        assert_eq!(loop2000f.hl_segments.hl03_hierarchical_level_code, "PT");
        assert_eq!(loop2000f.hl_segments.hl04_hierarchical_child_code, "0");
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_get_loop2000f_with_hl_ss() {
        let contents = String::from("HL*5*4*SS*0~UM*HS*I*2~");
        let (loop2000f, contents) = get_loop2000f(contents);
        assert_eq!(loop2000f.hl_segments.hl01_hierarchical_id_number, "5");
        assert_eq!(loop2000f.hl_segments.hl02_hierarchical_parent_id_number, "4");
        assert_eq!(loop2000f.hl_segments.hl03_hierarchical_level_code, "SS");
        assert_eq!(loop2000f.hl_segments.hl04_hierarchical_child_code, "0");
        
        assert!(loop2000f.um_segments.is_some());
        let um = loop2000f.um_segments.unwrap();
        assert_eq!(um.um00_request_category_code_prefix, "HS");
        assert_eq!(um.um01_request_category_code, "");
        assert_eq!(um.um02_certification_type_code, "I");
        assert_eq!(um.um03_service_type_code, "2");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2000f_with_hl_pt() {
        let loop2000f = Loop2000F {
            hl_segments: HL {
                hl01_hierarchical_id_number: "6".to_string(),
                hl02_hierarchical_parent_id_number: "5".to_string(),
                hl03_hierarchical_level_code: "PT".to_string(),
                hl04_hierarchical_child_code: "0".to_string(),
            },
            um_segments: None,
        };
        
        let contents = write_loop2000f(loop2000f);
        assert_eq!(contents, "HL*6*5*PT*0~");
    }
    
    #[test]
    fn test_write_loop2000f_with_hl_ss_and_um() {
        let loop2000f = Loop2000F {
            hl_segments: HL {
                hl01_hierarchical_id_number: "5".to_string(),
                hl02_hierarchical_parent_id_number: "4".to_string(),
                hl03_hierarchical_level_code: "SS".to_string(),
                hl04_hierarchical_child_code: "0".to_string(),
            },
            um_segments: Some(UM {
                um00_request_category_code_prefix: "HS".to_string(),
                um01_request_category_code: "".to_string(),
                um02_certification_type_code: "I".to_string(),
                um03_service_type_code: "2".to_string(),
                um04_health_care_service_location_information: "".to_string(),
                um05_related_causes_information: "".to_string(),
                um06_level_of_service_code: "".to_string(),
                um07_current_health_condition_code: "".to_string(),
                um08_prognosis_code: "".to_string(),
                um09_release_of_information_code: "".to_string(),
                um10_delay_reason_code: "".to_string(),
            }),
        };
        
        let contents = write_loop2000f(loop2000f);
        assert_eq!(contents, "HL*5*4*SS*0~UM*HS*I*2~");
    }
}
