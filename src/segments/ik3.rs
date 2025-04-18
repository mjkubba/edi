use serde::{Serialize, Deserialize};
// use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK3 {
    pub ik301_segment_id_code: String,
    pub ik302_segment_position_in_ts: String,
    pub ik303_loop_identifier: String,
    pub ik304_impl_segment_syntax_error_code: String,
}


pub fn get_ik3(ik3_content: String) -> IK3 {
    let ik3_parts: Vec<&str> = ik3_content.split("*").collect();
    let mut ik303_loop_identifier = String::new();
    let mut ik304_impl_segment_syntax_error_code = String::new();

    if ik3_parts.get(2).is_some() {
        ik303_loop_identifier = ik3_parts[2].to_string();
    }
    if ik3_parts.get(3).is_some() {
        ik304_impl_segment_syntax_error_code = ik3_parts[3].to_string();
    }

    IK3 {
        ik301_segment_id_code: ik3_parts[0].to_string(),
        ik302_segment_position_in_ts: ik3_parts[1].to_string(),
        ik303_loop_identifier,
        ik304_impl_segment_syntax_error_code,
    }
}

// pub fn write_ik3(ik3:IK3) -> String {
//     let mut ik3_content = String::new();
//     ik3_content.push_str("IK3*");
//     ik3_content.push_str(&ik3.ik301_segment_id_code);
//     ik3_content.push_str("*");
//     ik3_content.push_str(&ik3.ik302_segment_position_in_ts);
//     ik3_content.push_str(&stiuational_element(ik3.ik303_loop_identifier));
//     ik3_content.push_str(&stiuational_element(ik3.ik304_impl_segment_syntax_error_code));
//     ik3_content.push_str("~");
//     ik3_content
// }


// unit test

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_ak2() {
//         let ak2_content = "A*1*2~".to_string();
//         let ak2 = get_ak2(ak2_content);
//         assert_eq!(ak1.ak01_functional_id_group, "A");
//         assert_eq!(ak1.ak02_group_control_numbner, "1");
//     }
// }