use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::dmg::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010D {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub dmg_segments: Option<DMG>,
}

pub fn get_loop2010d(mut contents: String) -> (Loop2010D, String) {
    let mut nm1_segments = NM1::default();
    let mut ref_segments = Vec::new();
    let mut dmg_segments = None;
    
    if contents.contains("NM1") {
        // Check if this is a Dependent Name NM1 segment (NM101=QC)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && nm1_parts[0] == "QC" {
            info!("NM1 segment found for Dependent Name, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "DMG", contents.clone()) && check_if_segement_in_loop("REF", "HL", contents.clone()) {
                info!("REF segment found, ");
                let ref_segment = get_ref(get_segment_contents("REF", &contents));
                info!("REF segment parsed");
                
                ref_segments.push(ref_segment);
                contents = content_trim("REF", contents);
            }
            
            // Parse DMG segment
            if contents.contains("DMG") && check_if_segement_in_loop("DMG", "HL", contents.clone()) {
                info!("DMG segment found, ");
                dmg_segments = Some(get_dmg(get_segment_contents("DMG", &contents)));
                info!("DMG segment parsed");
                
                contents = content_trim("DMG", contents);
            }
        }
    }
    
    info!("Loop 2010D parsed\n");
    
    let loop2010d = Loop2010D {
        nm1_segments,
        ref_segments,
        dmg_segments,
    };
    
    return (loop2010d, contents)
}

pub fn write_loop2010d(loop2010d: Loop2010D) -> String {
    let mut contents = String::new();
    contents.push_str(&write_nm1(loop2010d.nm1_segments));
    
    for ref_segment in loop2010d.ref_segments {
        contents.push_str(&write_ref(ref_segment));
    }
    
    if let Some(dmg) = loop2010d.dmg_segments {
        contents.push_str(&write_dmg(dmg));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2010d() {
        let contents = String::from("NM1*QC*1*DOE*JANE****MI*123456789B~REF*SY*987654321~DMG*D8*20100519*F~");
        let (loop2010d, contents) = get_loop2010d(contents);
        assert_eq!(loop2010d.nm1_segments.entity_id, "QC");
        assert_eq!(loop2010d.nm1_segments.entity_type, "1");
        assert_eq!(loop2010d.nm1_segments.lastname, "DOE");
        assert_eq!(loop2010d.nm1_segments.firstname, "JANE");
        assert_eq!(loop2010d.nm1_segments.id_code_qualifier, "MI");
        assert_eq!(loop2010d.nm1_segments.id_code, "123456789B");
        
        assert_eq!(loop2010d.ref_segments.len(), 1);
        assert_eq!(loop2010d.ref_segments[0].reference_id_number_qualifier, "SY");
        assert_eq!(loop2010d.ref_segments[0].reference_id_number, "987654321");
        
        assert!(loop2010d.dmg_segments.is_some());
        let dmg = loop2010d.dmg_segments.unwrap();
        assert_eq!(dmg.date_time_period_format_qualifier, "D8");
        assert_eq!(dmg.date_time_period, "20100519");
        assert_eq!(dmg.gender_code, "F");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2010d() {
        let loop2010d = Loop2010D {
            nm1_segments: NM1 {
                entity_id: "QC".to_string(),
                entity_type: "1".to_string(),
                lastname: "DOE".to_string(),
                firstname: "JANE".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "123456789B".to_string(),
                member_number: "".to_string(),
            },
            ref_segments: vec![
                REF {
                    reference_id_number_qualifier: "SY".to_string(),
                    reference_id_number: "987654321".to_string(),
                    description: "".to_string(),
                    reference_identifier: "".to_string(),
                }
            ],
            dmg_segments: Some(DMG {
                date_time_period_format_qualifier: "D8".to_string(),
                date_time_period: "20100519".to_string(),
                gender_code: "F".to_string(),
            }),
        };
        
        let contents = write_loop2010d(loop2010d);
        assert!(contents.contains("NM1*QC*1*DOE*JANE"));
        assert!(contents.contains("REF*SY*987654321"));
        assert!(contents.contains("DMG*D8*20100519*F"));
    }
}
