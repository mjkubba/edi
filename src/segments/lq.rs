#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]
pub struct LQ {
    pub lq01_code_list_qualifier: String,
    pub lq02_remark_code: String,
}

pub fn get_lq(lq_content: String) -> LQ {
    let lq_parts: Vec<&str> = lq_content.split("*").collect();
    LQ {
        lq01_code_list_qualifier: lq_parts[0].to_string(),
        lq02_remark_code: lq_parts[1].to_string(),
    }
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