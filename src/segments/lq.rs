use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct LQ {
    pub lq01_code_list_qualifier: String,
    pub lq02_remark_code: String,
}

pub fn get_lq(lq_content: String) -> LQ {
    let lq_parts: Vec<&str> = lq_content.split("*").collect();
    LQ {
        lq01_code_list_qualifier: get_element(&lq_parts, 0),
        lq02_remark_code: get_element(&lq_parts, 1),
    }
}

pub fn write_lq(lq: LQ) -> String {
    let mut lq_content: String = String::new();
    lq_content.push_str("LQ*");
    lq_content.push_str(&lq.lq01_code_list_qualifier);
    lq_content.push_str("*");
    lq_content.push_str(&lq.lq02_remark_code);
    lq_content.push_str("~");
    lq_content
}

// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_lq() {
        let lq_content = "HE*A".to_string();
        let lq = get_lq(lq_content);
        assert_eq!(lq.lq01_code_list_qualifier, "HE");
        assert_eq!(lq.lq02_remark_code, "A");
    }
}
