use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DSB {
    pub dsb01_disability_type_code: String,
    pub dsb02_quantity: String,
    pub dsb03_occupation_code: String,
    pub dsb04_work_intensity_code: String,
    pub dsb05_product_option_code: String,
    pub dsb06_monetary_amount: String,
    pub dsb07_product_option_code_2: String,
    pub dsb08_monetary_amount_2: String,
}

#[allow(dead_code)]
pub fn get_dsb(dsb_content: String) -> DSB {
    let dsb_parts: Vec<&str> = dsb_content.split("*").collect();

    let mut dsb = DSB::default();

    // DSB01 - Disability Type Code
    if !dsb_parts.is_empty() && !dsb_parts[0].is_empty() {
        dsb.dsb01_disability_type_code = dsb_parts[0].to_string();
    }

    // DSB02 - Quantity
    if dsb_parts.len() > 1 && !dsb_parts[1].is_empty() {
        dsb.dsb02_quantity = dsb_parts[1].to_string();
    }

    // DSB03 - Occupation Code
    if dsb_parts.len() > 2 && !dsb_parts[2].is_empty() {
        dsb.dsb03_occupation_code = dsb_parts[2].to_string();
    }

    // DSB04 - Work Intensity Code
    if dsb_parts.len() > 3 && !dsb_parts[3].is_empty() {
        dsb.dsb04_work_intensity_code = dsb_parts[3].to_string();
    }

    // DSB05 - Product Option Code
    if dsb_parts.len() > 4 && !dsb_parts[4].is_empty() {
        dsb.dsb05_product_option_code = dsb_parts[4].to_string();
    }

    // DSB06 - Monetary Amount
    if dsb_parts.len() > 5 && !dsb_parts[5].is_empty() {
        dsb.dsb06_monetary_amount = dsb_parts[5].to_string();
    }

    // DSB07 - Product Option Code 2
    if dsb_parts.len() > 6 && !dsb_parts[6].is_empty() {
        dsb.dsb07_product_option_code_2 = dsb_parts[6].to_string();
    }

    // DSB08 - Monetary Amount 2
    if dsb_parts.len() > 7 && !dsb_parts[7].is_empty() {
        dsb.dsb08_monetary_amount_2 = dsb_parts[7].to_string();
    }

    info!("Parsed DSB segment: {:?}", dsb);
    dsb
}

#[allow(dead_code)]
pub fn write_dsb(dsb: DSB) -> String {
    let mut dsb_content = String::new();

    dsb_content.push_str("DSB*");
    dsb_content.push_str(&dsb.dsb01_disability_type_code);

    // Include DSB02 if not empty
    if !dsb.dsb02_quantity.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb02_quantity);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB03 if not empty
    if !dsb.dsb03_occupation_code.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb03_occupation_code);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB04 if not empty
    if !dsb.dsb04_work_intensity_code.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb04_work_intensity_code);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB05 if not empty
    if !dsb.dsb05_product_option_code.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb05_product_option_code);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB06 if not empty
    if !dsb.dsb06_monetary_amount.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb06_monetary_amount);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB07 if not empty
    if !dsb.dsb07_product_option_code_2.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb07_product_option_code_2);
    } else {
        dsb_content.push_str("*");
    }

    // Include DSB08 if not empty
    if !dsb.dsb08_monetary_amount_2.is_empty() {
        dsb_content.push_str("*");
        dsb_content.push_str(&dsb.dsb08_monetary_amount_2);
    }

    // Remove trailing asterisks
    while dsb_content.ends_with("*") {
        dsb_content.pop();
    }

    dsb_content.push_str("~");
    dsb_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dsb() {
        let dsb_content = "1*100*12345*FT*A*50000*B*25000".to_string();
        let dsb = get_dsb(dsb_content);

        assert_eq!(dsb.dsb01_disability_type_code, "1");
        assert_eq!(dsb.dsb02_quantity, "100");
        assert_eq!(dsb.dsb03_occupation_code, "12345");
        assert_eq!(dsb.dsb04_work_intensity_code, "FT");
        assert_eq!(dsb.dsb05_product_option_code, "A");
        assert_eq!(dsb.dsb06_monetary_amount, "50000");
        assert_eq!(dsb.dsb07_product_option_code_2, "B");
        assert_eq!(dsb.dsb08_monetary_amount_2, "25000");
    }

    #[test]
    fn test_write_dsb() {
        let dsb = DSB {
            dsb01_disability_type_code: "1".to_string(),
            dsb02_quantity: "100".to_string(),
            dsb03_occupation_code: "12345".to_string(),
            dsb04_work_intensity_code: "FT".to_string(),
            dsb05_product_option_code: "A".to_string(),
            dsb06_monetary_amount: "50000".to_string(),
            dsb07_product_option_code_2: "B".to_string(),
            dsb08_monetary_amount_2: "25000".to_string(),
        };

        let dsb_content = write_dsb(dsb);
        assert_eq!(dsb_content, "DSB*1*100*12345*FT*A*50000*B*25000~");
    }
}
