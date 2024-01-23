// EDI 835 segment with Amount Qualifier Code and Service Line Allowed Amount 
#[derive(Debug, Default,PartialEq)]
#[allow(dead_code)]
pub struct AMT {
    pub amount_qualifier_code: String,
    pub service_line_allowed_amount: String,
}

pub fn get_amt(amt_content: String) -> AMT {
    let amt_parts: Vec<&str> = amt_content.split("*").collect();
    AMT {
        amount_qualifier_code: amt_parts[0].to_string(),
        service_line_allowed_amount: amt_parts[1].to_string(),
    }
}