use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HD {
    pub hd01_maintenance_type_code: String,
    pub hd02_maintenance_reason_code: String,
    pub hd03_insurance_line_code: String,
    pub hd04_plan_coverage_description: String,
    pub hd05_coverage_level_code: String,
    pub hd06_count: String,
    pub hd07_count_2: String,
    pub hd08_underwriting_decision_code: String,
    pub hd09_yes_no_condition_or_response_code: String,
    pub hd10_drug_house_code: String,
    pub hd11_yes_no_condition_or_response_code_2: String,
}

pub fn get_hd(hd_content: String) -> HD {
    let hd_parts: Vec<&str> = hd_content.split("*").collect();

    let mut hd = HD::default();

    // HD01 - Maintenance Type Code
    if !hd_parts.is_empty() && !hd_parts[0].is_empty() {
        hd.hd01_maintenance_type_code = hd_parts[0].to_string();
    }

    // HD02 - Maintenance Reason Code
    if hd_parts.len() > 1 && !hd_parts[1].is_empty() {
        hd.hd02_maintenance_reason_code = hd_parts[1].to_string();
    }

    // HD03 - Insurance Line Code
    if hd_parts.len() > 2 && !hd_parts[2].is_empty() {
        hd.hd03_insurance_line_code = hd_parts[2].to_string();
    }

    // HD04 - Plan Coverage Description
    if hd_parts.len() > 3 && !hd_parts[3].is_empty() {
        hd.hd04_plan_coverage_description = hd_parts[3].to_string();
    }

    // HD05 - Coverage Level Code
    if hd_parts.len() > 4 && !hd_parts[4].is_empty() {
        hd.hd05_coverage_level_code = hd_parts[4].to_string();
    }

    // HD06 - Count
    if hd_parts.len() > 5 && !hd_parts[5].is_empty() {
        hd.hd06_count = hd_parts[5].to_string();
    }

    // HD07 - Count 2
    if hd_parts.len() > 6 && !hd_parts[6].is_empty() {
        hd.hd07_count_2 = hd_parts[6].to_string();
    }

    // HD08 - Underwriting Decision Code
    if hd_parts.len() > 7 && !hd_parts[7].is_empty() {
        hd.hd08_underwriting_decision_code = hd_parts[7].to_string();
    }

    // HD09 - Yes/No Condition or Response Code
    if hd_parts.len() > 8 && !hd_parts[8].is_empty() {
        hd.hd09_yes_no_condition_or_response_code = hd_parts[8].to_string();
    }

    // HD10 - Drug House Code
    if hd_parts.len() > 9 && !hd_parts[9].is_empty() {
        hd.hd10_drug_house_code = hd_parts[9].to_string();
    }

    // HD11 - Yes/No Condition or Response Code 2
    if hd_parts.len() > 10 && !hd_parts[10].is_empty() {
        hd.hd11_yes_no_condition_or_response_code_2 = hd_parts[10].to_string();
    }

    info!("Parsed HD segment: {:?}", hd);
    hd
}

pub fn write_hd(hd: HD) -> String {
    let mut hd_content = String::new();

    hd_content.push_str("HD*");
    hd_content.push_str(&hd.hd01_maintenance_type_code);

    // Include HD02 if not empty
    if !hd.hd02_maintenance_reason_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd02_maintenance_reason_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD03 if not empty
    if !hd.hd03_insurance_line_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd03_insurance_line_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD04 if not empty
    if !hd.hd04_plan_coverage_description.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd04_plan_coverage_description);
    } else {
        hd_content.push_str("*");
    }

    // Include HD05 if not empty
    if !hd.hd05_coverage_level_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd05_coverage_level_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD06 if not empty
    if !hd.hd06_count.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd06_count);
    } else {
        hd_content.push_str("*");
    }

    // Include HD07 if not empty
    if !hd.hd07_count_2.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd07_count_2);
    } else {
        hd_content.push_str("*");
    }

    // Include HD08 if not empty
    if !hd.hd08_underwriting_decision_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd08_underwriting_decision_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD09 if not empty
    if !hd.hd09_yes_no_condition_or_response_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd09_yes_no_condition_or_response_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD10 if not empty
    if !hd.hd10_drug_house_code.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd10_drug_house_code);
    } else {
        hd_content.push_str("*");
    }

    // Include HD11 if not empty
    if !hd.hd11_yes_no_condition_or_response_code_2.is_empty() {
        hd_content.push_str("*");
        hd_content.push_str(&hd.hd11_yes_no_condition_or_response_code_2);
    }

    // Remove trailing asterisks
    while hd_content.ends_with("*") {
        hd_content.pop();
    }

    hd_content.push_str("~");
    hd_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hd() {
        let hd_content = "021*01*HLT*HEALTH PLAN*EMP*1*2*A*Y*RX*N".to_string();
        let hd = get_hd(hd_content);

        assert_eq!(hd.hd01_maintenance_type_code, "021");
        assert_eq!(hd.hd02_maintenance_reason_code, "01");
        assert_eq!(hd.hd03_insurance_line_code, "HLT");
        assert_eq!(hd.hd04_plan_coverage_description, "HEALTH PLAN");
        assert_eq!(hd.hd05_coverage_level_code, "EMP");
        assert_eq!(hd.hd06_count, "1");
        assert_eq!(hd.hd07_count_2, "2");
        assert_eq!(hd.hd08_underwriting_decision_code, "A");
        assert_eq!(hd.hd09_yes_no_condition_or_response_code, "Y");
        assert_eq!(hd.hd10_drug_house_code, "RX");
        assert_eq!(hd.hd11_yes_no_condition_or_response_code_2, "N");
    }

    #[test]
    fn test_write_hd() {
        let hd = HD {
            hd01_maintenance_type_code: "021".to_string(),
            hd02_maintenance_reason_code: "01".to_string(),
            hd03_insurance_line_code: "HLT".to_string(),
            hd04_plan_coverage_description: "HEALTH PLAN".to_string(),
            hd05_coverage_level_code: "EMP".to_string(),
            hd06_count: "1".to_string(),
            hd07_count_2: "2".to_string(),
            hd08_underwriting_decision_code: "A".to_string(),
            hd09_yes_no_condition_or_response_code: "Y".to_string(),
            hd10_drug_house_code: "RX".to_string(),
            hd11_yes_no_condition_or_response_code_2: "N".to_string(),
        };

        let hd_content = write_hd(hd);
        assert_eq!(hd_content, "HD*021*01*HLT*HEALTH PLAN*EMP*1*2*A*Y*RX*N~");
    }
}
