use serde::{Serialize, Deserialize};
use crate::helper::edihelper::stiuational_element;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AK9 {
    pub ak901_functional_ack_code: String,
    pub ak902_num_of_ts_incl: String,
    pub ak903_num_of_recv_ts: String,
    pub ak904_num_of_accepted_ts: String,
    pub ak905_fn_group_err_code: String,
    pub ak906_fn_group_err_code: String,
    pub ak907_fn_group_err_code: String,
    pub ak908_fn_group_err_code: String,
    pub ak909_fn_group_err_code: String,
    
}

pub fn get_ak9(ak9_content: String) -> AK9 {
    let ak9_parts: Vec<&str> = ak9_content.split("*").collect();
    let ak905_fn_group_err_code = String::new();
    let ak906_fn_group_err_code = String::new();
    let ak907_fn_group_err_code = String::new();
    let ak908_fn_group_err_code = String::new();
    let ak909_fn_group_err_code = String::new();

    AK9 {
        ak901_functional_ack_code: ak9_parts[0].to_string(),
        ak902_num_of_ts_incl: ak9_parts[1].to_string(),
        ak903_num_of_recv_ts: ak9_parts[2].to_string(),
        ak904_num_of_accepted_ts: ak9_parts[3].to_string(),
        ak905_fn_group_err_code,
        ak906_fn_group_err_code,
        ak907_fn_group_err_code,
        ak908_fn_group_err_code,
        ak909_fn_group_err_code,
    }
}
 


pub fn write_ak9(ak9:AK9) -> String {
    let mut ak9_content = String::new();
    ak9_content.push_str("AK9*");
    ak9_content.push_str(&ak9.ak901_functional_ack_code);
    ak9_content.push_str("*");
    ak9_content.push_str(&ak9.ak902_num_of_ts_incl);
    ak9_content.push_str("*");
    ak9_content.push_str(&ak9.ak903_num_of_recv_ts);
    ak9_content.push_str("*");
    ak9_content.push_str(&ak9.ak904_num_of_accepted_ts);
    ak9_content.push_str(&stiuational_element(ak9.ak905_fn_group_err_code));
    ak9_content.push_str(&stiuational_element(ak9.ak906_fn_group_err_code));
    ak9_content.push_str(&stiuational_element(ak9.ak907_fn_group_err_code));
    ak9_content.push_str(&stiuational_element(ak9.ak908_fn_group_err_code));
    ak9_content.push_str(&stiuational_element(ak9.ak909_fn_group_err_code));
    ak9_content.push_str("~");
    ak9_content
}

// unit test

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_ak1() {
//         let ak9_content = "A*1*2~".to_string();
//         let ak9 = get_ak9(ak9_content);

//     }
// }