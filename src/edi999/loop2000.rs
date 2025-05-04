use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ak2::*;
use crate::segments::ik5::*;
use crate::edi999::loop2100::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2000 {
    pub ak2_segments: AK2,
    pub loop2100s: Vec<Loop2100>,
    pub ik5_segments: IK5,
}

pub fn get_loop_2000(mut contents: String) -> (Loop2000, String) {
    let mut loop2000 = Loop2000::default();

    // Process AK2 segment (required)
    if contents.contains("AK2") {
        info!("AK2 segment found");
        let ak2_content = get_segment_contents("AK2", &contents);
        loop2000.ak2_segments = get_ak2(ak2_content);
        info!("AK2 segment parsed");
        contents = content_trim("AK2", contents);
    } else {
        info!("Warning: Required AK2 segment not found in Loop 2000");
    }

    // Process Loop 2100 segments
    let (loop2100s, new_contents) = get_loop_2100s(contents);
    loop2000.loop2100s = loop2100s;
    contents = new_contents;

    // Process IK5 segment (required)
    if contents.contains("IK5") {
        info!("IK5 segment found");
        let ik5_content = get_segment_contents("IK5", &contents);
        loop2000.ik5_segments = get_ik5(ik5_content);
        info!("IK5 segment parsed");
        contents = content_trim("IK5", contents);
    } else {
        info!("Warning: Required IK5 segment not found in Loop 2000");
    }

    info!("Loop 2000 parsed");
    
    (loop2000, contents)
}

pub fn get_loop_2000s(mut contents: String) -> (Vec<Loop2000>, String) {
    let ak2_count = contents.matches("AK2").count();
    let mut loop_2000_array = vec![];
    info!("Number of loops in loop 2000: {:?}", ak2_count);

    // Process each AK2 segment
    for _ in 0..ak2_count {
        // Find the next AK2 segment
        if contents.contains("AK2") {
            // Extract the content for this AK2 loop
            let end_pos = if let Some(next_ak2_pos) = contents[3..].find("AK2") {
                // If there's another AK2, extract up to that point
                3 + next_ak2_pos
            } else {
                // Otherwise, use all remaining content
                contents.len()
            };
            
            let loop_content = contents[..end_pos].to_string();
            let (loop2000, _) = get_loop_2000(loop_content);
            
            loop_2000_array.push(loop2000);
            
            // Remove the processed content
            contents = contents[end_pos..].to_string();
        } else {
            break;
        }
    }

    (loop_2000_array, contents)
}

pub fn write_loop2000(loop2000s: Vec<Loop2000>) -> String {
    let mut contents = String::new();
    
    for loop2000 in loop2000s {
        // Write AK2 segment
        contents.push_str(&write_ak2(loop2000.ak2_segments));
        
        // Write all Loop 2100 segments
        contents.push_str(&write_loop2100(loop2000.loop2100s));
        
        // Write IK5 segment
        contents.push_str(&write_ik5(loop2000.ik5_segments));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2000() {
        let contents = "AK2*837*000000001~IK3*NM1*1*8~CTX*SITUATIONAL TRIGGER*IK3*1*2100*1~IK4*1*66*1*123~CTX*ELEMENT*IK4*1*2110*2~IK5*A~".to_string();
        let (loop2000, remaining) = get_loop_2000(contents);
        
        assert_eq!(loop2000.ak2_segments.ak201_transaction_set_identifier_code, "837");
        assert_eq!(loop2000.ak2_segments.ak202_transaction_set_control_number, "000000001");
        assert_eq!(loop2000.loop2100s.len(), 1);
        assert_eq!(loop2000.ik5_segments.ik501_transaction_set_acknowledgment_code, "A");
        assert_eq!(remaining, "");
    }
    
    #[test]
    fn test_write_loop2000() {
        let mut loop2000 = Loop2000::default();
        
        // Set up AK2
        loop2000.ak2_segments = AK2 {
            ak201_transaction_set_identifier_code: "837".to_string(),
            ak202_transaction_set_control_number: "000000001".to_string(),
            ak203_implementation_convention_reference: "".to_string(),
        };
        
        // Set up Loop 2100
        let mut loop2100 = Loop2100::default();
        loop2100.ik3_segments = IK3 {
            ik301_segment_id_code: "NM1".to_string(),
            ik302_segment_position_in_transaction_set: "1".to_string(),
            ik303_loop_identifier_code: "8".to_string(),
            ik304_implementation_segment_syntax_error_code: "".to_string(),
        };
        
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
        loop2000.loop2100s.push(loop2100);
        
        // Set up IK5
        loop2000.ik5_segments = IK5 {
            ik501_transaction_set_acknowledgment_code: "A".to_string(),
            ik502_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik503_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik504_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik505_implementation_transaction_set_syntax_error_code: "".to_string(),
            ik506_implementation_transaction_set_syntax_error_code: "".to_string(),
        };
        
        let result = write_loop2000(vec![loop2000]);
        assert!(result.contains("AK2*837*000000001~"));
        assert!(result.contains("IK3*NM1*1*8~"));
        assert!(result.contains("CTX*SITUATIONAL TRIGGER*IK3*1*2100*1~"));
        assert!(result.contains("IK4*1*66*1*123~"));
        assert!(result.contains("CTX*ELEMENT*IK4*1*2110*2~"));
        assert!(result.contains("IK5*A~"));
    }
}
