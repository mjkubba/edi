#[derive(Debug, Default,PartialEq)]
#[allow(dead_code)]
pub struct QTY {
    pub qty01_quantity_qualifier: String,
    pub qty02_claim_supplement_information_quantity: String,
}

pub fn get_qty(qty_content: String) -> QTY {
    let qty_parts: Vec<&str> = qty_content.split("*").collect();
    QTY {
        qty01_quantity_qualifier: qty_parts[0].to_string(),
        qty02_claim_supplement_information_quantity: qty_parts[1].to_string(),
    }
}