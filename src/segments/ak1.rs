use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AK1 {
    pub ak01_functional_id_group: String,
    pub ak02_group_control_numbner: String,
    pub ak03_ver_release_id_code: String,
}

pub fn get_ak1(ak1_content: String) -> AK1 {
    let ak1_parts: Vec<&str> = ak1_content.split("*").collect();
    AK1 {
        ak01_functional_id_group: ak1_parts[0].to_string(),
        ak02_group_control_numbner: ak1_parts[1].to_string(),
        ak03_ver_release_id_code: ak1_parts[2].to_string(),
    }
}
 


// pub fn write_ak1(amt:AK1) -> String {
//     let mut ak1_content = String::new();
//     ak1_content.push_str("AK1*");
//     ak1_content.push_str(&amt.ak01_functional_id_group);
//     ak1_content.push_str("*");
//     ak1_content.push_str(&amt.ak02_group_control_numbner);
//     ak1_content.push_str("*");
//     ak1_content.push_str(&amt.ak03_ver_release_id_code);
//     ak1_content.push_str("~");
//     ak1_content
// }

// unit test

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ak1() {
        let ak1_content = "A*1*2~".to_string();
        let ak1 = get_ak1(ak1_content);
        assert_eq!(ak1.ak01_functional_id_group, "A");
        assert_eq!(ak1.ak02_group_control_numbner, "1");
    }
}