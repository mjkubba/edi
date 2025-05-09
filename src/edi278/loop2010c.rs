use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::dmg::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010C {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub dmg_segments: Option<DMG>,
}

pub fn get_loop2010c(mut contents: String) -> (Loop2010C, String) {
    let mut nm1_segments = NM1::default();
    let mut ref_segments = Vec::new();
    let mut dmg_segments = None;
    
    if contents.contains("NM1") {
        // Check if this is a Subscriber Name NM1 segment (NM101=IL)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && nm1_parts[0] == "IL" {
            info!("NM1 segment found for Subscriber Name, ");
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
    
    info!("Loop 2010C parsed\n");
    
    let loop2010c = Loop2010C {
        nm1_segments,
        ref_segments,
        dmg_segments,
    };
    
    return (loop2010c, contents)
}

pub fn write_loop2010c(loop2010c: Loop2010C) -> String {
    let mut contents = String::new();
    contents.push_str(&write_nm1(loop2010c.nm1_segments));
    
    for ref_segment in loop2010c.ref_segments {
        contents.push_str(&write_ref(ref_segment));
    }
    
    if let Some(dmg) = loop2010c.dmg_segments {
        contents.push_str(&write_dmg(dmg));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2010c() {
        let contents = String::from("NM1*IL*1*DOE*JOHN****MI*123456789A~REF*SY*123456789~DMG*D8*19800519*M~");
        let (loop2010c, contents) = get_loop2010c(contents);
        assert_eq!(loop2010c.nm1_segments.entity_id, "IL");
        assert_eq!(loop2010c.nm1_segments.entity_type, "1");
        assert_eq!(loop2010c.nm1_segments.lastname, "DOE");
        assert_eq!(loop2010c.nm1_segments.firstname, "JOHN");
        assert_eq!(loop2010c.nm1_segments.id_code_qualifier, "MI");
        assert_eq!(loop2010c.nm1_segments.id_code, "123456789A");
        
        assert_eq!(loop2010c.ref_segments.len(), 1);
        assert_eq!(loop2010c.ref_segments[0].reference_id_number_qualifier, "SY");
        assert_eq!(loop2010c.ref_segments[0].reference_id_number, "123456789");
        
        assert!(loop2010c.dmg_segments.is_some());
        let dmg = loop2010c.dmg_segments.unwrap();
        assert_eq!(dmg.date_time_period_format_qualifier, "D8");
        assert_eq!(dmg.date_time_period, "19800519");
        assert_eq!(dmg.gender_code, "M");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2010c() {
        let loop2010c = Loop2010C {
            nm1_segments: NM1 {
                entity_id: "IL".to_string(),
                entity_type: "1".to_string(),
                lastname: "DOE".to_string(),
                firstname: "JOHN".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "123456789A".to_string(),
                member_number: "".to_string(),
            },
            ref_segments: vec![
                REF {
                    reference_id_number_qualifier: "SY".to_string(),
                    reference_id_number: "123456789".to_string(),
                    description: "".to_string(),
                    reference_identifier: "".to_string(),
                }
            ],
            dmg_segments: Some(DMG {
                date_time_period_format_qualifier: "D8".to_string(),
                date_time_period: "19800519".to_string(),
                gender_code: "M".to_string(),
            }),
        };
        
        let contents = write_loop2010c(loop2010c);
        assert!(contents.contains("NM1*IL*1*DOE*JOHN"));
        assert!(contents.contains("REF*SY*123456789"));
        assert!(contents.contains("DMG*D8*19800519*M"));
    }
}
