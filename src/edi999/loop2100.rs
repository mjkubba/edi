use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ik3::*;
use crate::segments::ctx::*;
use crate::edi999::loop2110::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2100 {
    pub ik3_segments: IK3,
    pub ctx_segments: Vec<CTX>,
    pub loop2110s: Vec<Loop2110>,
}

pub fn get_loop_2100(mut contents: String) -> (Loop2100, String) {
    let mut loop2100 = Loop2100::default();
    let mut ctx_segments = Vec::new();

    // Process IK3 segment (required)
    if contents.contains("IK3") {
        info!("IK3 segment found");
        let ik3_content = get_segment_contents("IK3", &contents);
        loop2100.ik3_segments = get_ik3(ik3_content);
        info!("IK3 segment parsed");
        contents = content_trim("IK3", contents);
    } else {
        info!("Warning: Required IK3 segment not found in Loop 2100");
    }

    // Process CTX segments (situational, can be multiple)
    while contents.starts_with("CTX") {
        info!("CTX segment found");
        let ctx_content = get_segment_contents("CTX", &contents);
        let ctx = get_ctx(ctx_content);
        info!("CTX segment parsed: {:?}", ctx);
        ctx_segments.push(ctx);
        contents = content_trim("CTX", contents);
    }

    loop2100.ctx_segments = ctx_segments;

    // Process Loop 2110 segments
    let (loop2110s, new_contents) = get_loop_2110s(contents);
    loop2100.loop2110s = loop2110s;
    contents = new_contents;

    info!("Loop 2100 parsed");
    
    (loop2100, contents)
}

pub fn get_loop_2100s(mut contents: String) -> (Vec<Loop2100>, String) {
    let ik3_count = contents.matches("IK3").count();
    let mut loop_2100_array = vec![];
    info!("Number of loops in loop 2100: {:?}", ik3_count);

    for _ in 0..ik3_count {
        let (loop2100, new_contents) = get_loop_2100(contents.clone());
        loop_2100_array.push(loop2100);
        contents = new_contents;
    }

    (loop_2100_array, contents)
}

pub fn write_loop2100(loop2100s: Vec<Loop2100>) -> String {
    let mut contents = String::new();
    
    for loop2100 in loop2100s {
        // Write IK3 segment
        contents.push_str(&write_ik3(loop2100.ik3_segments));
        
        // Write all CTX segments
        for ctx in loop2100.ctx_segments {
            contents.push_str(&write_ctx(ctx));
        }
        
        // Write all Loop 2110 segments
        contents.push_str(&write_loop2110(loop2100.loop2110s));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2100() {
        let contents = "IK3*NM1*1*8~CTX*SITUATIONAL TRIGGER*IK3*1*2100*1~IK4*1*66*1*123~CTX*ELEMENT*IK4*1*2110*2~".to_string();
        let (loop2100, remaining) = get_loop_2100(contents);
        
        assert_eq!(loop2100.ik3_segments.ik301_segment_id_code, "NM1");
        assert_eq!(loop2100.ctx_segments.len(), 1);
        assert_eq!(loop2100.ctx_segments[0].ctx01_context_name, "SITUATIONAL TRIGGER");
        assert_eq!(loop2100.loop2110s.len(), 1);
        assert_eq!(loop2100.loop2110s[0].ctx_segments.len(), 1);
        assert_eq!(remaining, "");
    }
    
    #[test]
    fn test_write_loop2100() {
        let mut loop2100 = Loop2100::default();
        
        // Set up IK3
        loop2100.ik3_segments = IK3 {
            ik301_segment_id_code: "NM1".to_string(),
            ik302_segment_position_in_transaction_set: "1".to_string(),
            ik303_loop_identifier_code: "8".to_string(),
            ik304_implementation_segment_syntax_error_code: "".to_string(),
        };
        
        // Add CTX
        let ctx = CTX {
            ctx01_context_name: "SITUATIONAL TRIGGER".to_string(),
            ctx02_segment_id_code: "IK3".to_string(),
            ctx03_segment_position_in_transaction: "1".to_string(),
            ctx04_loop_id_code: "2100".to_string(),
            ctx05_position_in_segment: "1".to_string(),
            ctx06_reference_in_segment: "".to_string(),
        };
        
        loop2100.ctx_segments.push(ctx);
        
        // Add Loop 2110
        let mut loop2110 = Loop2110::default();
        loop2110.ik4_segments = IK4 {
            ik401_position_in_segment: "1".to_string(),
            ik402_data_element_reference_number: "66".to_string(),
            ik403_implementation_data_element_syntax_error_code: "1".to_string(),
            ik404_copy_of_bad_data_element: "123".to_string(),
        };
        
        let ctx2110 = CTX {
            ctx01_context_name: "ELEMENT".to_string(),
            ctx02_segment_id_code: "IK4".to_string(),
            ctx03_segment_position_in_transaction: "1".to_string(),
            ctx04_loop_id_code: "2110".to_string(),
            ctx05_position_in_segment: "2".to_string(),
            ctx06_reference_in_segment: "".to_string(),
        };
        
        loop2110.ctx_segments.push(ctx2110);
        loop2100.loop2110s.push(loop2110);
        
        let result = write_loop2100(vec![loop2100]);
        assert!(result.contains("IK3*NM1*1*8~"));
        assert!(result.contains("CTX*SITUATIONAL TRIGGER*IK3*1*2100*1~"));
        assert!(result.contains("IK4*1*66*1*123~"));
        assert!(result.contains("CTX*ELEMENT*IK4*1*2110*2~"));
    }
}
