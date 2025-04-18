use serde::{Serialize, Deserialize};
// use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK4 {
    pub ik401_pos_in_segment: String,
    pub ik402_data_elemnt_ref_num: String,
    pub ik403_impl_data_syntax_error_code: String,
    pub ik404_copy_of_bad_data_elemnt: String,
}


pub fn get_ik4(ik4_content: String) -> IK4 {
    let ik4_parts: Vec<&str> = ik4_content.split("*").collect();
    let mut ik402_data_elemnt_ref_num = String::new();
    let mut ik404_copy_of_bad_data_elemnt = String::new();

    if ik4_parts.get(2).is_some() {
        ik402_data_elemnt_ref_num = ik4_parts[2].to_string();
    }
    if ik4_parts.get(3).is_some() {
        ik404_copy_of_bad_data_elemnt = ik4_parts[3].to_string();
    }

    IK4 {
        ik401_pos_in_segment: ik4_parts[1].to_string(),
        ik402_data_elemnt_ref_num,
        ik403_impl_data_syntax_error_code: ik4_parts[2].to_string(),
        ik404_copy_of_bad_data_elemnt,
    }
}

// pub fn write_ik4(ik4:IK4) -> String {
//     let mut ik4_content = String::new();
//     ik4_content.push_str("IK4*");
//     ik4_content.push_str(&ik4.ik401_pos_in_segment);
//     ik4_content.push_str(&stiuational_element(ik4.ik402_data_elemnt_ref_num));
//     ik4_content.push_str(&stiuational_element(ik4.ik403_impl_data_syntax_error_code));
//     ik4_content.push_str(&stiuational_element(ik4.ik404_copy_of_bad_data_elemnt));
//     ik4_content.push_str("~");
//     ik4_content
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