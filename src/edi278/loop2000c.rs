use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::trn::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl_segments: HL,
    pub trn_segments: Vec<TRN>,
}

pub fn get_loop2000c(mut contents: String) -> (Loop2000C, String) {
    let mut hl_segments = HL::default();
    let mut trn_segments = Vec::new();
    
    if contents.contains("HL") {
        // Check if this is a Subscriber Level HL segment (HL03=22)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && hl_parts[2] == "22" {
            info!("HL segment found for Subscriber Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
            
            // Parse TRN segments
            while contents.contains("TRN") && check_if_segement_in_loop("TRN", "NM1", contents.clone()) {
                info!("TRN segment found, ");
                let trn_segment = get_trn(get_segment_contents("TRN", &contents));
                info!("TRN segment parsed");
                
                trn_segments.push(trn_segment);
                contents = content_trim("TRN", contents);
            }
        }
    }
    
    info!("Loop 2000C parsed\n");
    
    let loop2000c = Loop2000C {
        hl_segments,
        trn_segments,
    };
    
    return (loop2000c, contents)
}

pub fn write_loop2000c(loop2000c: Loop2000C) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000c.hl_segments));
    
    for trn in &loop2000c.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000c() {
        let contents = String::from("HL*3*2*22*1~TRN*1*12345*1512345678~");
        let (loop2000c, contents) = get_loop2000c(contents);
        assert_eq!(loop2000c.hl_segments.hl01_hierarchical_id_number, "3");
        assert_eq!(loop2000c.hl_segments.hl02_hierarchical_parent_id_number, "2");
        assert_eq!(loop2000c.hl_segments.hl03_hierarchical_level_code, "22");
        assert_eq!(loop2000c.hl_segments.hl04_hierarchical_child_code, "1");
        
        assert_eq!(loop2000c.trn_segments.len(), 1);
        assert_eq!(loop2000c.trn_segments[0].trace_type_code, "1");
        assert_eq!(loop2000c.trn_segments[0].reference_id, "12345");
        assert_eq!(loop2000c.trn_segments[0].originating_company_id, "1512345678");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2000c() {
        let loop2000c = Loop2000C {
            hl_segments: HL {
                hl01_hierarchical_id_number: "3".to_string(),
                hl02_hierarchical_parent_id_number: "2".to_string(),
                hl03_hierarchical_level_code: "22".to_string(),
                hl04_hierarchical_child_code: "1".to_string(),
            },
            trn_segments: vec![
                TRN {
                    trace_type_code: "1".to_string(),
                    reference_id: "12345".to_string(),
                    originating_company_id: "1512345678".to_string(),
                    trn04_reference_id: "".to_string(),
                }
            ],
        };
        
        let contents = write_loop2000c(loop2000c);
        assert_eq!(contents, "HL*3*2*22*1~TRN*1*12345*1512345678~");
    }
}
