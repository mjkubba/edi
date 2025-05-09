use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl_segments: HL,
}

pub fn get_loop2000a(mut contents: String) -> (Loop2000A, String) {
    let mut hl_segments = HL::default();
    
    if contents.contains("HL") {
        // Check if this is a UMO Level HL segment (HL03=20)
        let hl_content = get_segment_contents("HL", &contents);
        let hl_parts: Vec<&str> = hl_content.split('*').collect();
        
        if hl_parts.len() > 3 && hl_parts[2] == "20" {
            info!("HL segment found for UMO Level, ");
            hl_segments = get_hl(hl_content);
            info!("HL segment parsed");
            
            contents = content_trim("HL", contents);
        }
    }
    
    info!("Loop 2000A parsed\n");
    
    let loop2000a = Loop2000A {
        hl_segments,
    };
    
    return (loop2000a, contents)
}

pub fn write_loop2000a(loop2000a: Loop2000A) -> String {
    let mut contents = String::new();
    contents.push_str(&write_hl(loop2000a.hl_segments));
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2000a() {
        let contents = String::from("HL*1**20*1~");
        let (loop2000a, contents) = get_loop2000a(contents);
        assert_eq!(loop2000a.hl_segments.hl01_hierarchical_id_number, "1");
        assert_eq!(loop2000a.hl_segments.hl02_hierarchical_parent_id_number, "");
        assert_eq!(loop2000a.hl_segments.hl03_hierarchical_level_code, "20");
        assert_eq!(loop2000a.hl_segments.hl04_hierarchical_child_code, "1");
        assert_eq!(contents, "");
    }
}
