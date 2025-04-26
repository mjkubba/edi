use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ik4::*;
use crate::segments::ctx::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2110 {
    pub ik4_segments: IK4,
    pub ctx_segments: Vec<CTX>,
}

pub fn get_loop_2110(mut contents: String) -> (Loop2110, String) {
    let mut loop2110 = Loop2110::default();
    let mut ctx_segments = Vec::new();

    // Process IK4 segment (required)
    if contents.contains("IK4") {
        info!("IK4 segment found");
        loop2110.ik4_segments = get_ik4(get_segment_contents("IK4", &contents));
        info!("IK4 segment parsed");
        contents = content_trim("IK4", contents);
    } else {
        info!("Warning: Required IK4 segment not found in Loop 2110");
    }

    // Process CTX segments (situational, can be multiple)
    while contents.starts_with("CTX") {
        info!("CTX segment found");
        let ctx = get_ctx(get_segment_contents("CTX", &contents));
        info!("CTX segment parsed: {:?}", ctx);
        ctx_segments.push(ctx);
        contents = content_trim("CTX", contents);
    }

    loop2110.ctx_segments = ctx_segments;
    info!("Loop 2110 parsed");
    
    (loop2110, contents)
}

pub fn get_loop_2110s(mut contents: String) -> (Vec<Loop2110>, String) {
    let ik4_count = contents.matches("IK4").count();
    let mut loop_2110_array = vec![];
    info!("Number of loops in loop 2110: {:?}", ik4_count);

    for _ in 0..ik4_count {
        let (loop2110, new_contents) = get_loop_2110(contents.clone());
        loop_2110_array.push(loop2110);
        contents = new_contents;
    }

    (loop_2110_array, contents)
}

pub fn write_loop2110(loop2110s: Vec<Loop2110>) -> String {
    let mut contents = String::new();
    
    for loop2110 in loop2110s {
        // Write IK4 segment
        contents.push_str(&write_ik4(loop2110.ik4_segments));
        
        // Write all CTX segments
        for ctx in loop2110.ctx_segments {
            contents.push_str(&write_ctx(ctx));
        }
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2110() {
        let contents = "IK4*1*66*1*123~CTX*SITUATIONAL TRIGGER*IK3*2*2000*1*1~".to_string();
        let (loop2110, remaining) = get_loop_2110(contents);
        
        assert_eq!(loop2110.ctx_segments.len(), 1);
        assert_eq!(loop2110.ctx_segments[0].ctx01_context_name, "SITUATIONAL TRIGGER");
        assert_eq!(loop2110.ctx_segments[0].ctx02_segment_id_code, "IK3");
        assert_eq!(remaining, "");
    }
    
    #[test]
    fn test_write_loop2110() {
        let mut loop2110 = Loop2110::default();
        
        // Set up IK4
        loop2110.ik4_segments = IK4 {
            ik401_position_in_segment: "1".to_string(),
            ik402_data_element_reference_number: "66".to_string(),
            ik403_implementation_data_element_syntax_error_code: "1".to_string(),
            ik404_copy_of_bad_data_element: "123".to_string(),
        };
        
        // Add CTX
        let ctx = CTX {
            ctx01_context_name: "SITUATIONAL TRIGGER".to_string(),
            ctx02_segment_id_code: "IK3".to_string(),
            ctx03_segment_position_in_transaction: "2".to_string(),
            ctx04_loop_id_code: "2000".to_string(),
            ctx05_position_in_segment: "1".to_string(),
            ctx06_reference_in_segment: "1".to_string(),
        };
        
        loop2110.ctx_segments.push(ctx);
        
        let result = write_loop2110(vec![loop2110]);
        assert!(result.contains("IK4*1*66*1*123~"));
        assert!(result.contains("CTX*SITUATIONAL TRIGGER*IK3*2*2000*1*1~"));
    }
}
