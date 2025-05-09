use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::nm1::*;
use crate::segments::per::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010A {
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
}

pub fn get_loop2010a(mut contents: String) -> (Loop2010A, String) {
    let mut nm1_segments = NM1::default();
    let mut per_segments = Vec::new();
    
    if contents.contains("NM1") {
        // Check if this is a UMO Name NM1 segment (NM101=X3)
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        if nm1_parts.len() > 1 && nm1_parts[0] == "X3" {
            info!("NM1 segment found for UMO Name, ");
            nm1_segments = get_nm1(nm1_content);
            info!("NM1 segment parsed");
            
            contents = content_trim("NM1", contents);
            
            // Parse PER segments
            while contents.contains("PER") {
                info!("PER segment found, ");
                let per_segment = get_per(get_segment_contents("PER", &contents));
                info!("PER segment parsed");
                
                per_segments.push(per_segment);
                contents = content_trim("PER", contents);
                
                // Check if the next segment is not a PER segment
                if !contents.starts_with("PER*") {
                    break;
                }
            }
        }
    }
    
    info!("Loop 2010A parsed\n");
    
    let loop2010a = Loop2010A {
        nm1_segments,
        per_segments,
    };
    
    return (loop2010a, contents)
}

pub fn write_loop2010a(loop2010a: Loop2010A) -> String {
    let mut contents = String::new();
    contents.push_str(&write_nm1(loop2010a.nm1_segments));
    
    for per in loop2010a.per_segments {
        contents.push_str(&write_per(per));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2010a() {
        let contents = String::from("NM1*X3*2*UMO NAME*****PI*12345~PER*IC*CONTACT NAME*TE*5551234567~");
        let (loop2010a, contents) = get_loop2010a(contents);
        assert_eq!(loop2010a.nm1_segments.entity_id, "X3");
        assert_eq!(loop2010a.nm1_segments.entity_type, "2");
        assert_eq!(loop2010a.nm1_segments.lastname, "UMO NAME");
        assert_eq!(loop2010a.nm1_segments.id_code, "PI");
        assert_eq!(loop2010a.nm1_segments.member_number, "12345");
        
        assert_eq!(loop2010a.per_segments.len(), 1);
        assert_eq!(loop2010a.per_segments[0].per01_contact_function_code, "IC");
        assert_eq!(loop2010a.per_segments[0].per02_contact_name, "CONTACT NAME");
        assert_eq!(loop2010a.per_segments[0].per03_contact_number_qualifier, "TE");
        assert_eq!(loop2010a.per_segments[0].per04_contact_number, "5551234567");
        
        assert_eq!(contents, "");
    }
}
