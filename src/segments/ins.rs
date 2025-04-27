use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct INS {
    pub ins01_insured_indicator: String,
    pub ins02_individual_relationship_code: String,
    pub ins03_maintenance_type_code: String,
    pub ins04_maintenance_reason_code: String,
    pub ins05_benefit_status_code: String,
    pub ins06_medicare_plan_code: String,
    pub ins07_cobra_qualifying_event_code: String,
    pub ins08_employment_status_code: String,
    pub ins09_student_status_code: String,
    pub ins10_handicap_indicator: String,
    pub ins11_date_time_qualifier: String,
    pub ins12_date_time_period_format_qualifier: String,
    pub ins13_date_time_period: String,
    pub ins14_confidentiality_code: String,
    pub ins15_city_name: String,
    pub ins16_state_or_province_code: String,
    pub ins17_country_code: String,
}

pub fn get_ins(ins_content: String) -> INS {
    let ins_parts: Vec<&str> = ins_content.split("*").collect();
    
    let mut ins = INS::default();
    
    // INS01 - Insured Indicator
    if !ins_parts.is_empty() && !ins_parts[0].is_empty() {
        ins.ins01_insured_indicator = ins_parts[0].to_string();
    }
    
    // INS02 - Individual Relationship Code
    if ins_parts.len() > 1 && !ins_parts[1].is_empty() {
        ins.ins02_individual_relationship_code = ins_parts[1].to_string();
    }
    
    // INS03 - Maintenance Type Code
    if ins_parts.len() > 2 && !ins_parts[2].is_empty() {
        ins.ins03_maintenance_type_code = ins_parts[2].to_string();
    }
    
    // INS04 - Maintenance Reason Code
    if ins_parts.len() > 3 && !ins_parts[3].is_empty() {
        ins.ins04_maintenance_reason_code = ins_parts[3].to_string();
    }
    
    // INS05 - Benefit Status Code
    if ins_parts.len() > 4 && !ins_parts[4].is_empty() {
        ins.ins05_benefit_status_code = ins_parts[4].to_string();
    }
    
    // INS06 - Medicare Plan Code
    if ins_parts.len() > 5 && !ins_parts[5].is_empty() {
        ins.ins06_medicare_plan_code = ins_parts[5].to_string();
    }
    
    // INS07 - COBRA Qualifying Event Code
    if ins_parts.len() > 6 && !ins_parts[6].is_empty() {
        ins.ins07_cobra_qualifying_event_code = ins_parts[6].to_string();
    }
    
    // INS08 - Employment Status Code
    if ins_parts.len() > 7 && !ins_parts[7].is_empty() {
        ins.ins08_employment_status_code = ins_parts[7].to_string();
    }
    
    // INS09 - Student Status Code
    if ins_parts.len() > 8 && !ins_parts[8].is_empty() {
        ins.ins09_student_status_code = ins_parts[8].to_string();
    }
    
    // INS10 - Handicap Indicator
    if ins_parts.len() > 9 && !ins_parts[9].is_empty() {
        ins.ins10_handicap_indicator = ins_parts[9].to_string();
    }
    
    // INS11 - Date Time Qualifier
    if ins_parts.len() > 10 && !ins_parts[10].is_empty() {
        ins.ins11_date_time_qualifier = ins_parts[10].to_string();
    }
    
    // INS12 - Date Time Period Format Qualifier
    if ins_parts.len() > 11 && !ins_parts[11].is_empty() {
        ins.ins12_date_time_period_format_qualifier = ins_parts[11].to_string();
    }
    
    // INS13 - Date Time Period
    if ins_parts.len() > 12 && !ins_parts[12].is_empty() {
        ins.ins13_date_time_period = ins_parts[12].to_string();
    }
    
    // INS14 - Confidentiality Code
    if ins_parts.len() > 13 && !ins_parts[13].is_empty() {
        ins.ins14_confidentiality_code = ins_parts[13].to_string();
    }
    
    // INS15 - City Name
    if ins_parts.len() > 14 && !ins_parts[14].is_empty() {
        ins.ins15_city_name = ins_parts[14].to_string();
    }
    
    // INS16 - State or Province Code
    if ins_parts.len() > 15 && !ins_parts[15].is_empty() {
        ins.ins16_state_or_province_code = ins_parts[15].to_string();
    }
    
    // INS17 - Country Code
    if ins_parts.len() > 16 && !ins_parts[16].is_empty() {
        ins.ins17_country_code = ins_parts[16].to_string();
    }
    
    info!("Parsed INS segment: {:?}", ins);
    ins
}

pub fn write_ins(ins: INS) -> String {
    let mut ins_content = String::new();
    
    ins_content.push_str("INS*");
    ins_content.push_str(&ins.ins01_insured_indicator);
    ins_content.push_str("*");
    ins_content.push_str(&ins.ins02_individual_relationship_code);
    
    // Include INS03 if not empty
    if !ins.ins03_maintenance_type_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins03_maintenance_type_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS04 if not empty
    if !ins.ins04_maintenance_reason_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins04_maintenance_reason_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS05 if not empty
    if !ins.ins05_benefit_status_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins05_benefit_status_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS06 if not empty
    if !ins.ins06_medicare_plan_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins06_medicare_plan_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS07 if not empty
    if !ins.ins07_cobra_qualifying_event_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins07_cobra_qualifying_event_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS08 if not empty
    if !ins.ins08_employment_status_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins08_employment_status_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS09 if not empty
    if !ins.ins09_student_status_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins09_student_status_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS10 if not empty
    if !ins.ins10_handicap_indicator.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins10_handicap_indicator);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS11 if not empty
    if !ins.ins11_date_time_qualifier.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins11_date_time_qualifier);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS12 if not empty
    if !ins.ins12_date_time_period_format_qualifier.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins12_date_time_period_format_qualifier);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS13 if not empty
    if !ins.ins13_date_time_period.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins13_date_time_period);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS14 if not empty
    if !ins.ins14_confidentiality_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins14_confidentiality_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS15 if not empty
    if !ins.ins15_city_name.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins15_city_name);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS16 if not empty
    if !ins.ins16_state_or_province_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins16_state_or_province_code);
    } else {
        ins_content.push_str("*");
    }
    
    // Include INS17 if not empty
    if !ins.ins17_country_code.is_empty() {
        ins_content.push_str("*");
        ins_content.push_str(&ins.ins17_country_code);
    }
    
    // Remove trailing asterisks
    while ins_content.ends_with("*") {
        ins_content.pop();
    }
    
    ins_content.push_str("~");
    ins_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_ins() {
        let ins_content = "Y*18*021*01*A*A*7*FT*F*N*356*D8*20220101*Y*ANYTOWN*CA*US".to_string();
        let ins = get_ins(ins_content);
        
        assert_eq!(ins.ins01_insured_indicator, "Y");
        assert_eq!(ins.ins02_individual_relationship_code, "18");
        assert_eq!(ins.ins03_maintenance_type_code, "021");
        assert_eq!(ins.ins04_maintenance_reason_code, "01");
        assert_eq!(ins.ins05_benefit_status_code, "A");
        assert_eq!(ins.ins06_medicare_plan_code, "A");
        assert_eq!(ins.ins07_cobra_qualifying_event_code, "7");
        assert_eq!(ins.ins08_employment_status_code, "FT");
        assert_eq!(ins.ins09_student_status_code, "F");
        assert_eq!(ins.ins10_handicap_indicator, "N");
        assert_eq!(ins.ins11_date_time_qualifier, "356");
        assert_eq!(ins.ins12_date_time_period_format_qualifier, "D8");
        assert_eq!(ins.ins13_date_time_period, "20220101");
        assert_eq!(ins.ins14_confidentiality_code, "Y");
        assert_eq!(ins.ins15_city_name, "ANYTOWN");
        assert_eq!(ins.ins16_state_or_province_code, "CA");
        assert_eq!(ins.ins17_country_code, "US");
    }
    
    #[test]
    fn test_get_ins_minimal() {
        let ins_content = "Y*18".to_string();
        let ins = get_ins(ins_content);
        
        assert_eq!(ins.ins01_insured_indicator, "Y");
        assert_eq!(ins.ins02_individual_relationship_code, "18");
        assert_eq!(ins.ins03_maintenance_type_code, "");
        assert_eq!(ins.ins04_maintenance_reason_code, "");
    }
    
    #[test]
    fn test_write_ins() {
        let ins = INS {
            ins01_insured_indicator: "Y".to_string(),
            ins02_individual_relationship_code: "18".to_string(),
            ins03_maintenance_type_code: "021".to_string(),
            ins04_maintenance_reason_code: "01".to_string(),
            ins05_benefit_status_code: "A".to_string(),
            ins06_medicare_plan_code: "A".to_string(),
            ins07_cobra_qualifying_event_code: "7".to_string(),
            ins08_employment_status_code: "FT".to_string(),
            ins09_student_status_code: "F".to_string(),
            ins10_handicap_indicator: "N".to_string(),
            ins11_date_time_qualifier: "356".to_string(),
            ins12_date_time_period_format_qualifier: "D8".to_string(),
            ins13_date_time_period: "20220101".to_string(),
            ins14_confidentiality_code: "Y".to_string(),
            ins15_city_name: "ANYTOWN".to_string(),
            ins16_state_or_province_code: "CA".to_string(),
            ins17_country_code: "US".to_string(),
        };
        
        let ins_content = write_ins(ins);
        assert_eq!(ins_content, "INS*Y*18*021*01*A*A*7*FT*F*N*356*D8*20220101*Y*ANYTOWN*CA*US~");
    }
    
    #[test]
    fn test_write_ins_minimal() {
        let ins = INS {
            ins01_insured_indicator: "Y".to_string(),
            ins02_individual_relationship_code: "18".to_string(),
            ins03_maintenance_type_code: "".to_string(),
            ins04_maintenance_reason_code: "".to_string(),
            ins05_benefit_status_code: "".to_string(),
            ins06_medicare_plan_code: "".to_string(),
            ins07_cobra_qualifying_event_code: "".to_string(),
            ins08_employment_status_code: "".to_string(),
            ins09_student_status_code: "".to_string(),
            ins10_handicap_indicator: "".to_string(),
            ins11_date_time_qualifier: "".to_string(),
            ins12_date_time_period_format_qualifier: "".to_string(),
            ins13_date_time_period: "".to_string(),
            ins14_confidentiality_code: "".to_string(),
            ins15_city_name: "".to_string(),
            ins16_state_or_province_code: "".to_string(),
            ins17_country_code: "".to_string(),
        };
        
        let ins_content = write_ins(ins);
        assert_eq!(ins_content, "INS*Y*18~");
    }
}
