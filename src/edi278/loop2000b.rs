use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl_segments: HL,
}

pub fn get_loop2000b(mut contents: String) -> (Loop2000B, String) {
    let mut hl_segments = HL::default();
    
    if contents.contains("HL") {
        // Check if this is a Requester Level HL segment (HL03=21)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && hl_parts[2] == "21" {
            info!("HL segment found for Requester Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
        }
    }
    
    info!("Loop 2000B parsed\n");
    
    let loop2000b = Loop2000B {
        hl_segments,
    };
    
    return (loop2000b, contents)
}

pub fn write_loop2000b(loop2000b: Loop2000B) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000b.hl_segments));
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000b() {
        let contents = String::from("HL*2*1*21*1~");
        let (loop2000b, contents) = get_loop2000b(contents);
        assert_eq!(loop2000b.hl_segments.hl01_hierarchical_id_number, "2");
        assert_eq!(loop2000b.hl_segments.hl02_hierarchical_parent_id_number, "1");
        assert_eq!(loop2000b.hl_segments.hl03_hierarchical_level_code, "21");
        assert_eq!(loop2000b.hl_segments.hl04_hierarchical_child_code, "1");
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2000b() {
        let loop2000b = Loop2000B {
            hl_segments: HL {
                hl01_hierarchical_id_number: "2".to_string(),
                hl02_hierarchical_parent_id_number: "1".to_string(),
                hl03_hierarchical_level_code: "21".to_string(),
                hl04_hierarchical_child_code: "1".to_string(),
            },
        };
        
        let contents = write_loop2000b(loop2000b);
        assert_eq!(contents, "HL*2*1*21*1~");
    }
}
