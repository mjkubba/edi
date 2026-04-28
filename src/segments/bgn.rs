use crate::helper::edihelper::get_element;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BGN {
    pub bgn01_transaction_set_purpose_code: String,
    pub bgn02_reference_identification: String,
    pub bgn03_date: String,
    pub bgn04_time: String,
    pub bgn05_time_code: String,
    pub bgn06_reference_identification_2: String,
    pub bgn07_transaction_type_code: String,
    pub bgn08_action_code: String,
}

pub fn get_bgn(bgn_content: String) -> BGN {
    let bgn_parts: Vec<&str> = bgn_content.split("*").collect();

    let mut bgn = BGN::default();

    // BGN01 - Transaction Set Purpose Code
    if !bgn_parts.is_empty() && !bgn_parts[0].is_empty() {
        bgn.bgn01_transaction_set_purpose_code = get_element(&bgn_parts, 0);
    }

    // BGN02 - Reference Identification
    if bgn_parts.len() > 1 && !bgn_parts[1].is_empty() {
        bgn.bgn02_reference_identification = get_element(&bgn_parts, 1);
    }

    // BGN03 - Date
    if bgn_parts.len() > 2 && !bgn_parts[2].is_empty() {
        bgn.bgn03_date = get_element(&bgn_parts, 2);
    }

    // BGN04 - Time
    if bgn_parts.len() > 3 && !bgn_parts[3].is_empty() {
        bgn.bgn04_time = get_element(&bgn_parts, 3);
    }

    // BGN05 - Time Code
    if bgn_parts.len() > 4 && !bgn_parts[4].is_empty() {
        bgn.bgn05_time_code = get_element(&bgn_parts, 4);
    }

    // BGN06 - Reference Identification 2
    if bgn_parts.len() > 5 && !bgn_parts[5].is_empty() {
        bgn.bgn06_reference_identification_2 = get_element(&bgn_parts, 5);
    }

    // BGN07 - Transaction Type Code
    if bgn_parts.len() > 6 && !bgn_parts[6].is_empty() {
        bgn.bgn07_transaction_type_code = get_element(&bgn_parts, 6);
    }

    // BGN08 - Action Code
    if bgn_parts.len() > 7 && !bgn_parts[7].is_empty() {
        bgn.bgn08_action_code = get_element(&bgn_parts, 7);
    }

    info!("Parsed BGN segment: {:?}", bgn);
    bgn
}

pub fn write_bgn(bgn: BGN) -> String {
    let mut bgn_content = String::new();

    bgn_content.push_str("BGN*");
    bgn_content.push_str(&bgn.bgn01_transaction_set_purpose_code);
    bgn_content.push_str("*");
    bgn_content.push_str(&bgn.bgn02_reference_identification);

    // Include BGN03 if not empty
    if !bgn.bgn03_date.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn03_date);
    } else {
        bgn_content.push_str("*");
    }

    // Include BGN04 if not empty
    if !bgn.bgn04_time.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn04_time);
    } else {
        bgn_content.push_str("*");
    }

    // Include BGN05 if not empty
    if !bgn.bgn05_time_code.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn05_time_code);
    } else {
        bgn_content.push_str("*");
    }

    // Include BGN06 if not empty
    if !bgn.bgn06_reference_identification_2.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn06_reference_identification_2);
    } else {
        bgn_content.push_str("*");
    }

    // Include BGN07 if not empty
    if !bgn.bgn07_transaction_type_code.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn07_transaction_type_code);
    } else {
        bgn_content.push_str("*");
    }

    // Include BGN08 if not empty
    if !bgn.bgn08_action_code.is_empty() {
        bgn_content.push_str("*");
        bgn_content.push_str(&bgn.bgn08_action_code);
    }

    // Remove trailing asterisks
    while bgn_content.ends_with("*") {
        bgn_content.pop();
    }

    bgn_content.push_str("~");
    bgn_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bgn() {
        let bgn_content = "00*12345*20220101*1200*ET*67890*2*4".to_string();
        let bgn = get_bgn(bgn_content);

        assert_eq!(bgn.bgn01_transaction_set_purpose_code, "00");
        assert_eq!(bgn.bgn02_reference_identification, "12345");
        assert_eq!(bgn.bgn03_date, "20220101");
        assert_eq!(bgn.bgn04_time, "1200");
        assert_eq!(bgn.bgn05_time_code, "ET");
        assert_eq!(bgn.bgn06_reference_identification_2, "67890");
        assert_eq!(bgn.bgn07_transaction_type_code, "2");
        assert_eq!(bgn.bgn08_action_code, "4");
    }

    #[test]
    fn test_write_bgn() {
        let bgn = BGN {
            bgn01_transaction_set_purpose_code: "00".to_string(),
            bgn02_reference_identification: "12345".to_string(),
            bgn03_date: "20220101".to_string(),
            bgn04_time: "1200".to_string(),
            bgn05_time_code: "ET".to_string(),
            bgn06_reference_identification_2: "67890".to_string(),
            bgn07_transaction_type_code: "2".to_string(),
            bgn08_action_code: "4".to_string(),
        };

        let bgn_content = write_bgn(bgn);
        assert_eq!(bgn_content, "BGN*00*12345*20220101*1200*ET*67890*2*4~");
    }
}
