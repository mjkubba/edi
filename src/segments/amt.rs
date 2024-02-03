use serde::{Serialize, Deserialize};

// EDI 835 segment with Amount Qualifier Code and Service Line Allowed Amount 
#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AMT {
    pub amt01_amount_qualifier_code: String,
    pub amt02_service_line_allowed_amount: String,
}

pub fn get_amt(amt_content: String) -> AMT {
    let amt_parts: Vec<&str> = amt_content.split("*").collect();
    AMT {
        amt01_amount_qualifier_code: amt_parts[0].to_string(),
        amt02_service_line_allowed_amount: amt_parts[1].to_string(),
    }
}

pub fn write_amt(amt:AMT) -> String {
    let mut amt_content = String::new();
    amt_content.push_str("AMT*");
    amt_content.push_str(&amt.amt01_amount_qualifier_code);
    amt_content.push_str("*");
    amt_content.push_str(&amt.amt02_service_line_allowed_amount);
    amt_content.push_str("~");
    amt_content
}


// unit test

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amt() {
        let amt_content = "A*100".to_string();
        let amt = get_amt(amt_content);
        assert_eq!(amt.amt01_amount_qualifier_code, "A");
        assert_eq!(amt.amt02_service_line_allowed_amount, "100");
    }
}