use serde::{Serialize, Deserialize};
// use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct IK5 {
    pub ik501_ts_ack_code: String,
    pub ik502_imp_ts_syn_er_code: String,
    pub ik503_imp_ts_syn_er_code: String,
    pub ik504_imp_ts_syn_er_code: String,
    pub ik505_imp_ts_syn_er_code: String,
    pub ik506_imp_ts_syn_er_code: String,
}


pub fn get_ik5(ik5_content: String) -> IK5 {
    let ik5_parts: Vec<&str> = ik5_content.split("*").collect();
    let mut ik502_imp_ts_syn_er_code = String::new();
    let mut ik503_imp_ts_syn_er_code = String::new();
    let mut ik504_imp_ts_syn_er_code = String::new();
    let mut ik505_imp_ts_syn_er_code = String::new();
    let mut ik506_imp_ts_syn_er_code = String::new();

    if ik5_parts.get(1).is_some() {
        ik502_imp_ts_syn_er_code = ik5_parts[1].to_string();
    }
    if ik5_parts.get(2).is_some() {
        ik503_imp_ts_syn_er_code = ik5_parts[2].to_string();
    }
    if ik5_parts.get(3).is_some() {
        ik504_imp_ts_syn_er_code = ik5_parts[3].to_string();
    }
    if ik5_parts.get(4).is_some() {
        ik505_imp_ts_syn_er_code = ik5_parts[4].to_string();
    }
    if ik5_parts.get(5).is_some() {
        ik506_imp_ts_syn_er_code = ik5_parts[5].to_string();
    }

    IK5 {
        ik501_ts_ack_code: ik5_parts[0].to_string(),
        ik502_imp_ts_syn_er_code,
        ik503_imp_ts_syn_er_code,
        ik504_imp_ts_syn_er_code,
        ik505_imp_ts_syn_er_code,
        ik506_imp_ts_syn_er_code,
    }
}

// pub fn write_ik5(ik5:IK5) -> String {
//     let mut ik5_content = String::new();
//     ik5_content.push_str("IK5*");
//     ik5_content.push_str(&ik5.ik501_ts_ack_code);
//     ik5_content.push_str(&stiuational_element(ik5.ik502_imp_ts_syn_er_code));
//     ik5_content.push_str(&stiuational_element(ik5.ik503_imp_ts_syn_er_code));
//     ik5_content.push_str(&stiuational_element(ik5.ik504_imp_ts_syn_er_code));
//     ik5_content.push_str(&stiuational_element(ik5.ik505_imp_ts_syn_er_code));
//     ik5_content.push_str(&stiuational_element(ik5.ik506_imp_ts_syn_er_code));
//     ik5_content.push_str("~");
//     ik5_content
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