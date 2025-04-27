use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EB {
    pub eb01_eligibility_indicator: String,
    pub eb02_benefit_type: String,
    pub eb03_service_type_code: String,
    pub eb04_insurance_type_code: String,
    pub eb05_plan_coverage_description: String,
    pub eb06_time_period_qualifier: String,
    pub eb07_monetary_amount: String,
    pub eb08_percent: String,
    pub eb09_quantity_qualifier: String,
    pub eb10_quantity: String,
    pub eb11_yes_no_condition_response_code: String,
    pub eb12_yes_no_condition_response_code: String,
}

pub fn get_eb(eb_content: String) -> EB {
    let eb_parts: Vec<&str> = eb_content.split("*").collect();
    
    let mut eb = EB::default();
    
    // EB01 - Eligibility or Benefit Information
    if !eb_parts.is_empty() && !eb_parts[0].is_empty() {
        eb.eb01_eligibility_indicator = eb_parts[0].to_string();
    }
    
    // EB02 - Benefit Type
    if eb_parts.len() > 1 && !eb_parts[1].is_empty() {
        eb.eb02_benefit_type = eb_parts[1].to_string();
    }
    
    // EB03 - Service Type Code
    if eb_parts.len() > 2 && !eb_parts[2].is_empty() {
        eb.eb03_service_type_code = eb_parts[2].to_string();
    }
    
    // EB04 - Insurance Type Code
    if eb_parts.len() > 3 && !eb_parts[3].is_empty() {
        eb.eb04_insurance_type_code = eb_parts[3].to_string();
    }
    
    // EB05 - Plan Coverage Description
    if eb_parts.len() > 4 && !eb_parts[4].is_empty() {
        eb.eb05_plan_coverage_description = eb_parts[4].to_string();
    }
    
    // EB06 - Time Period Qualifier
    if eb_parts.len() > 5 && !eb_parts[5].is_empty() {
        eb.eb06_time_period_qualifier = eb_parts[5].to_string();
    }
    
    // EB07 - Monetary Amount
    if eb_parts.len() > 6 && !eb_parts[6].is_empty() {
        eb.eb07_monetary_amount = eb_parts[6].to_string();
    }
    
    // EB08 - Percent
    if eb_parts.len() > 7 && !eb_parts[7].is_empty() {
        eb.eb08_percent = eb_parts[7].to_string();
    }
    
    // EB09 - Quantity Qualifier
    if eb_parts.len() > 8 && !eb_parts[8].is_empty() {
        eb.eb09_quantity_qualifier = eb_parts[8].to_string();
    }
    
    // EB10 - Quantity
    if eb_parts.len() > 9 && !eb_parts[9].is_empty() {
        eb.eb10_quantity = eb_parts[9].to_string();
    }
    
    // EB11 - Yes/No Condition or Response Code
    if eb_parts.len() > 10 && !eb_parts[10].is_empty() {
        eb.eb11_yes_no_condition_response_code = eb_parts[10].to_string();
    }
    
    // EB12 - Yes/No Condition or Response Code
    if eb_parts.len() > 11 && !eb_parts[11].is_empty() {
        eb.eb12_yes_no_condition_response_code = eb_parts[11].to_string();
    }
    
    info!("Parsed EB segment: {:?}", eb);
    eb
}

pub fn write_eb(eb: EB) -> String {
    let mut eb_content = String::new();
    
    eb_content.push_str("EB*");
    eb_content.push_str(&eb.eb01_eligibility_indicator);
    eb_content.push_str("*");
    eb_content.push_str(&eb.eb02_benefit_type);
    
    // Include EB03 if not empty
    if !eb.eb03_service_type_code.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb03_service_type_code);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB04 if not empty
    if !eb.eb04_insurance_type_code.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb04_insurance_type_code);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB05 if not empty
    if !eb.eb05_plan_coverage_description.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb05_plan_coverage_description);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB06 if not empty
    if !eb.eb06_time_period_qualifier.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb06_time_period_qualifier);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB07 if not empty
    if !eb.eb07_monetary_amount.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb07_monetary_amount);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB08 if not empty
    if !eb.eb08_percent.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb08_percent);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB09 if not empty
    if !eb.eb09_quantity_qualifier.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb09_quantity_qualifier);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB10 if not empty
    if !eb.eb10_quantity.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb10_quantity);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB11 if not empty
    if !eb.eb11_yes_no_condition_response_code.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb11_yes_no_condition_response_code);
    } else {
        eb_content.push_str("*");
    }
    
    // Include EB12 if not empty
    if !eb.eb12_yes_no_condition_response_code.is_empty() {
        eb_content.push_str("*");
        eb_content.push_str(&eb.eb12_yes_no_condition_response_code);
    }
    
    // Remove trailing asterisks
    while eb_content.ends_with("*") {
        eb_content.pop();
    }
    
    eb_content.push_str("~");
    eb_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_eb() {
        let eb_content = "1*30*98*MC*GOLD PLAN*27*2000*20*FA*12*Y*N".to_string();
        let eb = get_eb(eb_content);
        
        assert_eq!(eb.eb01_eligibility_indicator, "1");
        assert_eq!(eb.eb02_benefit_type, "30");
        assert_eq!(eb.eb03_service_type_code, "98");
        assert_eq!(eb.eb04_insurance_type_code, "MC");
        assert_eq!(eb.eb05_plan_coverage_description, "GOLD PLAN");
        assert_eq!(eb.eb06_time_period_qualifier, "27");
        assert_eq!(eb.eb07_monetary_amount, "2000");
        assert_eq!(eb.eb08_percent, "20");
        assert_eq!(eb.eb09_quantity_qualifier, "FA");
        assert_eq!(eb.eb10_quantity, "12");
        assert_eq!(eb.eb11_yes_no_condition_response_code, "Y");
        assert_eq!(eb.eb12_yes_no_condition_response_code, "N");
    }
    
    #[test]
    fn test_get_eb_minimal() {
        let eb_content = "1*30".to_string();
        let eb = get_eb(eb_content);
        
        assert_eq!(eb.eb01_eligibility_indicator, "1");
        assert_eq!(eb.eb02_benefit_type, "30");
        assert_eq!(eb.eb03_service_type_code, "");
        assert_eq!(eb.eb04_insurance_type_code, "");
    }
    
    #[test]
    fn test_write_eb() {
        let eb = EB {
            eb01_eligibility_indicator: "1".to_string(),
            eb02_benefit_type: "30".to_string(),
            eb03_service_type_code: "98".to_string(),
            eb04_insurance_type_code: "MC".to_string(),
            eb05_plan_coverage_description: "GOLD PLAN".to_string(),
            eb06_time_period_qualifier: "27".to_string(),
            eb07_monetary_amount: "2000".to_string(),
            eb08_percent: "20".to_string(),
            eb09_quantity_qualifier: "FA".to_string(),
            eb10_quantity: "12".to_string(),
            eb11_yes_no_condition_response_code: "Y".to_string(),
            eb12_yes_no_condition_response_code: "N".to_string(),
        };
        
        let eb_content = write_eb(eb);
        assert_eq!(eb_content, "EB*1*30*98*MC*GOLD PLAN*27*2000*20*FA*12*Y*N~");
    }
    
    #[test]
    fn test_write_eb_minimal() {
        let eb = EB {
            eb01_eligibility_indicator: "1".to_string(),
            eb02_benefit_type: "30".to_string(),
            eb03_service_type_code: "".to_string(),
            eb04_insurance_type_code: "".to_string(),
            eb05_plan_coverage_description: "".to_string(),
            eb06_time_period_qualifier: "".to_string(),
            eb07_monetary_amount: "".to_string(),
            eb08_percent: "".to_string(),
            eb09_quantity_qualifier: "".to_string(),
            eb10_quantity: "".to_string(),
            eb11_yes_no_condition_response_code: "".to_string(),
            eb12_yes_no_condition_response_code: "".to_string(),
        };
        
        let eb_content = write_eb(eb);
        assert_eq!(eb_content, "EB*1*30~");
    }
}
