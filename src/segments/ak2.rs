use serde::{Serialize, Deserialize};
// use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AK2 {
    pub ak201_ts_id_code: String,
    pub ak202_ts_control_numbner: String,
    pub ak203_imple_conven_ref: String,
}

pub fn get_ak2(ak2_content: String) -> AK2 {
    let ak2_parts: Vec<&str> = ak2_content.split("*").collect();
    let mut ak203_imple_conven_ref = String::new();

    if ak2_parts.get(2).is_some() {
        ak203_imple_conven_ref = ak2_parts[2].to_string();
    }

    AK2 {
        ak201_ts_id_code: ak2_parts[0].to_string(),
        ak202_ts_control_numbner: ak2_parts[1].to_string(),
        ak203_imple_conven_ref,
    }
}
 


// pub fn write_ak2(ak2:AK2) -> String {
//     let mut ak2_content = String::new();
//     ak2_content.push_str("AK2*");
//     ak2_content.push_str(&ak2.ak201_ts_id_code);
//     ak2_content.push_str("*");
//     ak2_content.push_str(&ak2.ak202_ts_control_numbner);
//     ak2_content.push_str(&stiuational_element(ak2.ak203_imple_conven_ref));
//     ak2_content.push_str("~");
//     ak2_content
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