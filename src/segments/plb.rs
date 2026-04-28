use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct PLB {
    pub plb01_provider_identifier: String,
    pub plb02_fiscal_period_date: String,
    pub plb03_provider_adjustment_identifier: String,
    pub plb04_provider_adjustment_amount: String,
    pub plb05_provider_adjustment_identifier: String,
    pub plb06_provider_adjustment_amount: String,
    pub plb07_provider_adjustment_identifier: String,
    pub plb08_provider_adjustment_amount: String,
    pub plb09_provider_adjustment_identifier: String,
    pub plb10_provider_adjustment_amount: String,
    pub plb11_provider_adjustment_identifier: String,
    pub plb12_provider_adjustment_amount: String,
    pub plb13_provider_adjustment_identifier: String,
    pub plb14_provider_adjustment_amount: String,
}

pub fn get_plb(plb_content: String) -> PLB {
    let plb_parts: Vec<&str> = plb_content.split("*").collect();

    let mut plb05_provider_adjustment_identifier: String = "".to_string();
    let mut plb06_provider_adjustment_amount: String = "".to_string();
    let mut plb07_provider_adjustment_identifier: String = "".to_string();
    let mut plb08_provider_adjustment_amount: String = "".to_string();
    let mut plb09_provider_adjustment_identifier: String = "".to_string();
    let mut plb10_provider_adjustment_amount: String = "".to_string();
    let mut plb11_provider_adjustment_identifier: String = "".to_string();
    let mut plb12_provider_adjustment_amount: String = "".to_string();
    let mut plb13_provider_adjustment_identifier: String = "".to_string();
    let mut plb14_provider_adjustment_amount: String = "".to_string();

    if plb_parts.get(4).is_some() {
        plb05_provider_adjustment_identifier = get_element(&plb_parts, 4);
    }
    if plb_parts.get(5).is_some() {
        plb06_provider_adjustment_amount = get_element(&plb_parts, 5);
    }
    if plb_parts.get(6).is_some() {
        plb07_provider_adjustment_identifier = get_element(&plb_parts, 6);
    }
    if plb_parts.get(7).is_some() {
        plb08_provider_adjustment_amount = get_element(&plb_parts, 7);
    }
    if plb_parts.get(8).is_some() {
        plb09_provider_adjustment_identifier = get_element(&plb_parts, 8);
    }
    if plb_parts.get(9).is_some() {
        plb10_provider_adjustment_amount = get_element(&plb_parts, 9);
    }
    if plb_parts.get(10).is_some() {
        plb11_provider_adjustment_identifier = get_element(&plb_parts, 10);
    }
    if plb_parts.get(11).is_some() {
        plb12_provider_adjustment_amount = get_element(&plb_parts, 11);
    }
    if plb_parts.get(12).is_some() {
        plb13_provider_adjustment_identifier = get_element(&plb_parts, 12);
    }
    if plb_parts.get(13).is_some() {
        plb14_provider_adjustment_amount = get_element(&plb_parts, 13);
    }

    PLB {
        plb01_provider_identifier: get_element(&plb_parts, 0),
        plb02_fiscal_period_date: get_element(&plb_parts, 1),
        plb03_provider_adjustment_identifier: get_element(&plb_parts, 2),
        plb04_provider_adjustment_amount: get_element(&plb_parts, 3),
        plb05_provider_adjustment_identifier,
        plb06_provider_adjustment_amount,
        plb07_provider_adjustment_identifier,
        plb08_provider_adjustment_amount,
        plb09_provider_adjustment_identifier,
        plb10_provider_adjustment_amount,
        plb11_provider_adjustment_identifier,
        plb12_provider_adjustment_amount,
        plb13_provider_adjustment_identifier,
        plb14_provider_adjustment_amount,
    }
}

pub fn write_plb(plb: PLB) -> String {
    build_segment(&[
        "PLB",
        &plb.plb01_provider_identifier,
        &plb.plb02_fiscal_period_date,
        &plb.plb03_provider_adjustment_identifier,
        &plb.plb04_provider_adjustment_amount,
        &plb.plb05_provider_adjustment_identifier,
        &plb.plb06_provider_adjustment_amount,
        &plb.plb07_provider_adjustment_identifier,
        &plb.plb08_provider_adjustment_amount,
        &plb.plb09_provider_adjustment_identifier,
        &plb.plb10_provider_adjustment_amount,
        &plb.plb11_provider_adjustment_identifier,
        &plb.plb12_provider_adjustment_amount,
        &plb.plb13_provider_adjustment_identifier,
        &plb.plb14_provider_adjustment_amount,
    ])
}

// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_plb() {
        let plb_content =
            "PLB*202208*PLB*0.00*PLB*0.00*PLB*0.00*PLB*0.00*PLB*0.00*PLB*0.00*PLB*0.00*"
                .to_string();
        let plb = get_plb(plb_content);
        assert_eq!(plb.plb01_provider_identifier, "PLB".to_string());
        assert_eq!(plb.plb02_fiscal_period_date, "202208".to_string());
        assert_eq!(plb.plb03_provider_adjustment_identifier, "PLB".to_string());
        assert_eq!(plb.plb04_provider_adjustment_amount, "0.00".to_string());
        assert_eq!(plb.plb05_provider_adjustment_identifier, "PLB".to_string());
        assert_eq!(plb.plb06_provider_adjustment_amount, "0.00".to_string());
    }
}
