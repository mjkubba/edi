use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2115C {
    pub nm1_segments: NM1,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub per_segments: Vec<PER>,
}

pub fn get_loop_2115c(mut contents: String) -> EdiResult<(Loop2115C, String)> {
    let mut loop2115c = Loop2115C::default();
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found for Loop 2115C");
        let nm1_content = get_segment_contents("NM1", &contents);
        if nm1_content.is_empty() {
            return Err(EdiError::MissingSegment("NM1".to_string()));
        }
        
        // Check if this is a P3 entity identifier (Provider)
        let nm1 = get_nm1(nm1_content);
        if nm1.entity_id == "P3" {
            loop2115c.nm1_segments = nm1;
            info!("NM1*P3 segment parsed for Loop 2115C");
            contents = content_trim("NM1", contents);
        } else {
            // If not a P3 entity, this is not a 2115C loop
            return Err(EdiError::MissingSegment("NM1*P3".to_string()));
        }
    } else {
        return Err(EdiError::MissingSegment("NM1".to_string()));
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found for Loop 2115C");
        let n3_content = get_segment_contents("N3", &contents);
        if !n3_content.is_empty() {
            loop2115c.n3_segments = Some(get_n3(n3_content));
            info!("N3 segment parsed for Loop 2115C");
            contents = content_trim("N3", contents);
        }
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found for Loop 2115C");
        let n4_content = get_segment_contents("N4", &contents);
        if !n4_content.is_empty() {
            loop2115c.n4_segments = Some(get_n4(n4_content));
            info!("N4 segment parsed for Loop 2115C");
            contents = content_trim("N4", contents);
        }
    }
    
    // Process PER segments (situational, can be multiple)
    while contents.starts_with("PER") {
        info!("PER segment found for Loop 2115C");
        let per_content = get_segment_contents("PER", &contents);
        if per_content.is_empty() {
            break;
        }
        let per = get_per(per_content);
        info!("PER segment parsed for Loop 2115C");
        loop2115c.per_segments.push(per);
        contents = content_trim("PER", contents);
    }
    
    info!("Loop 2115C parsed");
    Ok((loop2115c, contents))
}

pub fn write_loop_2115c(loop2115c: &Loop2115C) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2115c.nm1_segments.clone()));
    
    // Write N3 segment if present
    if let Some(n3) = &loop2115c.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2115c.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write all PER segments
    for per in &loop2115c.per_segments {
        contents.push_str(&write_per(per.clone()));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2115c() {
        let contents = "NM1*P3*1*JONES*MARCUS****SV*0202034~N3*123 MAIN ST~N4*ANYTOWN*NY*12345~PER*IC*OFFICE*TE*5551234567~".to_string();
        let result = get_loop_2115c(contents);
        
        assert!(result.is_ok());
        let (loop2115c, _) = result.unwrap();
        
        assert_eq!(loop2115c.nm1_segments.entity_id, "P3");
        assert_eq!(loop2115c.nm1_segments.entity_type, "1");
        assert_eq!(loop2115c.nm1_segments.lastname, "JONES");
        assert_eq!(loop2115c.nm1_segments.firstname, "MARCUS");
        assert_eq!(loop2115c.nm1_segments.id_code, "SV");
        assert_eq!(loop2115c.nm1_segments.member_number, "0202034");
        
        assert!(loop2115c.n3_segments.is_some());
        assert_eq!(loop2115c.n3_segments.unwrap().payee_address, "123 MAIN ST");
        
        assert!(loop2115c.n4_segments.is_some());
        let n4 = loop2115c.n4_segments.unwrap();
        assert_eq!(n4.payee_city, "ANYTOWN");
        assert_eq!(n4.payee_state, "NY");
        assert_eq!(n4.payee_zip, "12345");
        
        assert_eq!(loop2115c.per_segments.len(), 1);
        let per = &loop2115c.per_segments[0];
        assert_eq!(per.per01_contact_function_code, "IC");
        assert_eq!(per.per02_contact_name, "OFFICE");
        assert_eq!(per.per03_contact_number_qualifier, "TE");
        assert_eq!(per.per04_contact_number, "5551234567");
    }
    
    #[test]
    fn test_write_loop_2115c() {
        let loop2115c = Loop2115C {
            nm1_segments: NM1 {
                entity_id: "P3".to_string(),
                entity_type: "1".to_string(),
                lastname: "JONES".to_string(),
                firstname: "MARCUS".to_string(),
                middle_initial: "".to_string(),
                suffix: "".to_string(),
                title: "".to_string(),
                id_code: "SV".to_string(),
                member_number: "0202034".to_string(),
            },
            n3_segments: Some(N3 {
                payee_address: "123 MAIN ST".to_string(),
                payee_address2: "".to_string(),
            }),
            n4_segments: Some(N4 {
                payee_city: "ANYTOWN".to_string(),
                payee_state: "NY".to_string(),
                payee_zip: "12345".to_string(),
                payee_country_code: "".to_string(),
                payee_country_sub_code: "".to_string(),
            }),
            per_segments: vec![
                PER {
                    per01_contact_function_code: "IC".to_string(),
                    per02_contact_name: "OFFICE".to_string(),
                    per03_contact_number_qualifier: "TE".to_string(),
                    per04_contact_number: "5551234567".to_string(),
                    per05_contact_number_qualifier: "".to_string(),
                    per06_contact_number: "".to_string(),
                    per07_contact_number_qualifier: "".to_string(),
                    per08_contact_number: "".to_string(),
                }
            ],
        };
        
        let contents = write_loop_2115c(&loop2115c);
        assert!(contents.contains("NM1*P3*1*JONES*MARCUS****SV*0202034~"));
        assert!(contents.contains("N3*123 MAIN ST~"));
        assert!(contents.contains("N4*ANYTOWN*NY*12345~"));
        assert!(contents.contains("PER*IC*OFFICE*TE*5551234567~"));
    }
}
