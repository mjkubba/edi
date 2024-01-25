// EDI 835 CUR segment
#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]

pub struct CUR {
    pub identity_identifier_code: String,
    pub currency_code: String,
}

pub fn get_cur(cur_content: String) -> CUR {
    let cur_parts: Vec<&str> = cur_content.split("*").collect();
    CUR {
        identity_identifier_code: cur_parts[0].to_string(),
        currency_code: cur_parts[1].to_string(),
    }
}

// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_cur() {
        let cur_content = "PR*USD".to_string();
        let cur = get_cur(cur_content);
        assert_eq!(cur.identity_identifier_code, "PR".to_string());
        assert_eq!(cur.currency_code, "USD".to_string());
    }
}