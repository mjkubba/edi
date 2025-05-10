use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000F {
    pub hl_segments: HL,
}

pub fn get_loop2000f(mut contents: String) -> (Loop2000F, String) {
    let mut hl_segments = HL::default();
    
    if contents.contains("HL") {
        // Check if this is a Service Provider Level HL segment (HL03=PT)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && hl_parts[2] == "PT" {
            info!("HL segment found for Service Provider Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
        }
    }
    
    info!("Loop 2000F parsed\n");
    
    let loop2000f = Loop2000F {
        hl_segments,
    };
    
    return (loop2000f, contents)
}

pub fn write_loop2000f(loop2000f: Loop2000F) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000f.hl_segments));
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000f() {
        let contents = String::from("HL*6*5*PT*0~");
        let (loop2000f, contents) = get_loop2000f(contents);
        assert_eq!(loop2000f.hl_segments.hl01_hierarchical_id_number, "6");
        assert_eq!(loop2000f.hl_segments.hl02_hierarchical_parent_id_number, "5");
        assert_eq!(loop2000f.hl_segments.hl03_hierarchical_level_code, "PT");
        assert_eq!(loop2000f.hl_segments.hl04_hierarchical_child_code, "0");
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2000f() {
        let loop2000f = Loop2000F {
            hl_segments: HL {
                hl01_hierarchical_id_number: "6".to_string(),
                hl02_hierarchical_parent_id_number: "5".to_string(),
                hl03_hierarchical_level_code: "PT".to_string(),
                hl04_hierarchical_child_code: "0".to_string(),
            },
        };
        
        let contents = write_loop2000f(loop2000f);
        assert_eq!(contents, "HL*6*5*PT*0~");
    }
}
