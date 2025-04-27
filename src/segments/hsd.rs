use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HSD {
    pub hsd01_quantity_qualifier: String,
    pub hsd02_quantity: String,
    pub hsd03_unit_of_measure_code: String,
    pub hsd04_sample_selection_modulus: String,
    pub hsd05_time_period_qualifier: String,
    pub hsd06_period_count: String,
    pub hsd07_delivery_frequency_code: String,
    pub hsd08_delivery_pattern_time_code: String,
}

pub fn get_hsd(hsd_content: String) -> HSD {
    let hsd_parts: Vec<&str> = hsd_content.split("*").collect();
    
    let mut hsd = HSD::default();
    
    // HSD01 - Quantity Qualifier
    if !hsd_parts.is_empty() && !hsd_parts[0].is_empty() {
        hsd.hsd01_quantity_qualifier = hsd_parts[0].to_string();
    }
    
    // HSD02 - Quantity
    if hsd_parts.len() > 1 && !hsd_parts[1].is_empty() {
        hsd.hsd02_quantity = hsd_parts[1].to_string();
    }
    
    // HSD03 - Unit of Measure Code
    if hsd_parts.len() > 2 && !hsd_parts[2].is_empty() {
        hsd.hsd03_unit_of_measure_code = hsd_parts[2].to_string();
    }
    
    // HSD04 - Sample Selection Modulus
    if hsd_parts.len() > 3 && !hsd_parts[3].is_empty() {
        hsd.hsd04_sample_selection_modulus = hsd_parts[3].to_string();
    }
    
    // HSD05 - Time Period Qualifier
    if hsd_parts.len() > 4 && !hsd_parts[4].is_empty() {
        hsd.hsd05_time_period_qualifier = hsd_parts[4].to_string();
    }
    
    // HSD06 - Period Count
    if hsd_parts.len() > 5 && !hsd_parts[5].is_empty() {
        hsd.hsd06_period_count = hsd_parts[5].to_string();
    }
    
    // HSD07 - Delivery Frequency Code
    if hsd_parts.len() > 6 && !hsd_parts[6].is_empty() {
        hsd.hsd07_delivery_frequency_code = hsd_parts[6].to_string();
    }
    
    // HSD08 - Delivery Pattern Time Code
    if hsd_parts.len() > 7 && !hsd_parts[7].is_empty() {
        hsd.hsd08_delivery_pattern_time_code = hsd_parts[7].to_string();
    }
    
    info!("Parsed HSD segment: {:?}", hsd);
    hsd
}

pub fn write_hsd(hsd: HSD) -> String {
    let mut hsd_content = String::new();
    
    hsd_content.push_str("HSD*");
    hsd_content.push_str(&hsd.hsd01_quantity_qualifier);
    
    // Include HSD02 if not empty
    if !hsd.hsd02_quantity.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd02_quantity);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD03 if not empty
    if !hsd.hsd03_unit_of_measure_code.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd03_unit_of_measure_code);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD04 if not empty
    if !hsd.hsd04_sample_selection_modulus.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd04_sample_selection_modulus);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD05 if not empty
    if !hsd.hsd05_time_period_qualifier.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd05_time_period_qualifier);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD06 if not empty
    if !hsd.hsd06_period_count.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd06_period_count);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD07 if not empty
    if !hsd.hsd07_delivery_frequency_code.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd07_delivery_frequency_code);
    } else {
        hsd_content.push_str("*");
    }
    
    // Include HSD08 if not empty
    if !hsd.hsd08_delivery_pattern_time_code.is_empty() {
        hsd_content.push_str("*");
        hsd_content.push_str(&hsd.hsd08_delivery_pattern_time_code);
    }
    
    // Remove trailing asterisks
    while hsd_content.ends_with("*") {
        hsd_content.pop();
    }
    
    hsd_content.push_str("~");
    hsd_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_hsd() {
        let hsd_content = "VS*12*DA*1*7*4*1*D".to_string();
        let hsd = get_hsd(hsd_content);
        
        assert_eq!(hsd.hsd01_quantity_qualifier, "VS");
        assert_eq!(hsd.hsd02_quantity, "12");
        assert_eq!(hsd.hsd03_unit_of_measure_code, "DA");
        assert_eq!(hsd.hsd04_sample_selection_modulus, "1");
        assert_eq!(hsd.hsd05_time_period_qualifier, "7");
        assert_eq!(hsd.hsd06_period_count, "4");
        assert_eq!(hsd.hsd07_delivery_frequency_code, "1");
        assert_eq!(hsd.hsd08_delivery_pattern_time_code, "D");
    }
    
    #[test]
    fn test_get_hsd_minimal() {
        let hsd_content = "VS*12".to_string();
        let hsd = get_hsd(hsd_content);
        
        assert_eq!(hsd.hsd01_quantity_qualifier, "VS");
        assert_eq!(hsd.hsd02_quantity, "12");
        assert_eq!(hsd.hsd03_unit_of_measure_code, "");
        assert_eq!(hsd.hsd04_sample_selection_modulus, "");
    }
    
    #[test]
    fn test_write_hsd() {
        let hsd = HSD {
            hsd01_quantity_qualifier: "VS".to_string(),
            hsd02_quantity: "12".to_string(),
            hsd03_unit_of_measure_code: "DA".to_string(),
            hsd04_sample_selection_modulus: "1".to_string(),
            hsd05_time_period_qualifier: "7".to_string(),
            hsd06_period_count: "4".to_string(),
            hsd07_delivery_frequency_code: "1".to_string(),
            hsd08_delivery_pattern_time_code: "D".to_string(),
        };
        
        let hsd_content = write_hsd(hsd);
        assert_eq!(hsd_content, "HSD*VS*12*DA*1*7*4*1*D~");
    }
    
    #[test]
    fn test_write_hsd_minimal() {
        let hsd = HSD {
            hsd01_quantity_qualifier: "VS".to_string(),
            hsd02_quantity: "12".to_string(),
            hsd03_unit_of_measure_code: "".to_string(),
            hsd04_sample_selection_modulus: "".to_string(),
            hsd05_time_period_qualifier: "".to_string(),
            hsd06_period_count: "".to_string(),
            hsd07_delivery_frequency_code: "".to_string(),
            hsd08_delivery_pattern_time_code: "".to_string(),
        };
        
        let hsd_content = write_hsd(hsd);
        assert_eq!(hsd_content, "HSD*VS*12~");
    }
}
