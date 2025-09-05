use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::r#ref::*;
use crate::segments::prv::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2110E {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub prv_segments: Option<PRV>,
}

pub fn get_loop2110e(mut contents: String) -> (Loop2110E, String) {
    let mut nm1_segments = NM1::default();
    let mut ref_segments = Vec::new();
    let mut n3_segments = None;
    let mut n4_segments = None;
    let mut prv_segments = None;
    
    if contents.contains("NM1") {
        // Check if this is a Service Provider NM1 segment (NM101=71 or NM101=72 or NM101=77 or NM101=AAJ or NM101=FA)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && (nm1_parts[0] == "71" || nm1_parts[0] == "72" || 
                                  nm1_parts[0] == "77" || nm1_parts[0] == "AAJ" || 
                                  nm1_parts[0] == "FA" || nm1_parts[0] == "SJ") {
            info!("NM1 segment found for Service Provider, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "N3", contents.clone()) && 
                  check_if_segement_in_loop("REF", "N4", contents.clone()) && 
                  check_if_segement_in_loop("REF", "PRV", contents.clone()) && 
                  check_if_segement_in_loop("REF", "NM1", contents.clone()) {
                info!("REF segment found, ");
                let ref_segment = get_ref(get_segment_contents("REF", &contents));
                info!("REF segment parsed");
                
                ref_segments.push(ref_segment);
                contents = content_trim("REF", contents);
            }
            
            // Parse N3 segment (address)
            if contents.contains("N3") && check_if_segement_in_loop("N3", "N4", contents.clone()) && 
               check_if_segement_in_loop("N3", "PRV", contents.clone()) && 
               check_if_segement_in_loop("N3", "NM1", contents.clone()) {
                info!("N3 segment found, ");
                let n3_content = get_segment_contents("N3", &contents);
                n3_segments = Some(get_n3(n3_content));
                info!("N3 segment parsed");
                
                contents = content_trim("N3", contents);
            }
            
            // Parse N4 segment (city, state, zip)
            if contents.contains("N4") && check_if_segement_in_loop("N4", "PRV", contents.clone()) && 
               check_if_segement_in_loop("N4", "NM1", contents.clone()) {
                info!("N4 segment found, ");
                let n4_content = get_segment_contents("N4", &contents);
                n4_segments = Some(get_n4(n4_content));
                info!("N4 segment parsed");
                
                contents = content_trim("N4", contents);
            }
            
            // Parse PRV segment
            if contents.contains("PRV") && check_if_segement_in_loop("PRV", "NM1", contents.clone()) {
                info!("PRV segment found, ");
                let prv_content = get_segment_contents("PRV", &contents);
                prv_segments = Some(get_prv(&prv_content));
                info!("PRV segment parsed");
                
                contents = content_trim("PRV", contents);
            }
        }
    }
    
    info!("Loop 2110E parsed\n");
    
    let loop2110e = Loop2110E {
        nm1_segments,
        ref_segments,
        n3_segments,
        n4_segments,
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
    
    if let Some(n3) = loop2110e.n3_segments {
        contents.push_str(&write_n3(n3));
    }
    
    if let Some(n4) = loop2110e.n4_segments {
        contents.push_str(&write_n4(n4));
    }
    
    if let Some(prv) = loop2110e.prv_segments {
        contents.push_str(&write_prv(&prv));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2110e_provider() {
        let contents = String::from("NM1*71*1*SMITH*JOHN*A***XX*1234567890~REF*1J*12345~PRV*PE*ZZ*207Q00000X~");
        let (loop2110e, contents) = get_loop2110e(contents);
        assert_eq!(loop2110e.nm1_segments.entity_id, "71");
        assert_eq!(loop2110e.nm1_segments.entity_type, "1");
        assert_eq!(loop2110e.nm1_segments.lastname, "SMITH");
        assert_eq!(loop2110e.nm1_segments.firstname, "JOHN");
        assert_eq!(loop2110e.nm1_segments.id_code, "XX");
        assert_eq!(loop2110e.nm1_segments.member_number, "1234567890");
        
        assert_eq!(loop2110e.ref_segments.len(), 1);
        assert_eq!(loop2110e.ref_segments[0].reference_id_number_qualifier, "1J");
        assert_eq!(loop2110e.ref_segments[0].reference_id_number, "12345");
        
        assert!(loop2110e.prv_segments.is_some());
        let prv = loop2110e.prv_segments.unwrap();
        assert_eq!(prv.prv01_provider_code, "PE");
        assert_eq!(prv.prv02_reference_identification_qualifier, "ZZ");
        assert_eq!(prv.prv03_reference_identification, "207Q00000X");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_get_loop2110e_facility() {
        let contents = String::from("NM1*FA*2*MONTGOMERY HOSPITAL*****24*000012121~N3*475 MAIN STREET~N4*ANYTOWN*PA*19087~");
        let (loop2110e, contents) = get_loop2110e(contents);
        assert_eq!(loop2110e.nm1_segments.entity_id, "FA");
        assert_eq!(loop2110e.nm1_segments.entity_type, "2");
        assert_eq!(loop2110e.nm1_segments.lastname, "MONTGOMERY HOSPITAL");
        assert_eq!(loop2110e.nm1_segments.id_code, "24");
        assert_eq!(loop2110e.nm1_segments.member_number, "000012121");
        
        assert!(loop2110e.n3_segments.is_some());
        let n3 = loop2110e.n3_segments.unwrap();
        assert_eq!(n3.payee_address, "475 MAIN STREET");
        
        assert!(loop2110e.n4_segments.is_some());
        let n4 = loop2110e.n4_segments.unwrap();
        assert_eq!(n4.payee_city, "ANYTOWN");
        assert_eq!(n4.payee_state, "PA");
        assert_eq!(n4.payee_zip, "19087");
        
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
                id_code: "XX".to_string(),
                member_number: "1234567890".to_string(),
            },
            ref_segments: vec![
                REF {
                    reference_id_number_qualifier: "1J".to_string(),
                    reference_id_number: "12345".to_string(),
                }
            ],
            n3_segments: None,
            n4_segments: None,
            prv_segments: Some(PRV {
                segment_id: "PRV".to_string(),
                prv01_provider_code: "PE".to_string(),
                prv02_reference_identification_qualifier: "ZZ".to_string(),
                prv03_reference_identification: "207Q00000X".to_string(),
                prv04_state_or_province_code: None,
                prv05_provider_specialty_information: None,
                prv06_provider_organization_code: None,
            }),
        };
        
        let contents = write_loop2110e(loop2110e);
        assert!(contents.contains("NM1*71*1*SMITH*JOHN*A"));
        assert!(contents.contains("REF*1J*12345"));
        assert!(contents.contains("PRV*PE*ZZ*207Q00000X"));
    }
    
    #[test]
    fn test_write_loop2110e_with_address() {
        let loop2110e = Loop2110E {
            nm1_segments: NM1 {
                entity_id: "FA".to_string(),
                entity_type: "2".to_string(),
                lastname: "MONTGOMERY HOSPITAL".to_string(),
                firstname: "".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "24".to_string(),
                member_number: "000012121".to_string(),
            },
            ref_segments: vec![],
            n3_segments: Some(N3 {
                payee_address: "475 MAIN STREET".to_string(),
                payee_address2: "".to_string(),
            }),
            n4_segments: Some(N4 {
                payee_city: "ANYTOWN".to_string(),
                payee_state: "PA".to_string(),
                payee_zip: "19087".to_string(),
                payee_country_code: "".to_string(),
                payee_country_sub_code: "".to_string(),
            }),
            prv_segments: None,
        };
        
        let contents = write_loop2110e(loop2110e);
        assert!(contents.contains("NM1*FA*2*MONTGOMERY HOSPITAL"));
        assert!(contents.contains("N3*475 MAIN STREET"));
        assert!(contents.contains("N4*ANYTOWN*PA*19087"));
    }
}
