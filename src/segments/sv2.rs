use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SV2 {
    pub sv201_service_line_revenue_code: String,
    pub sv202_procedure_code: String,
    pub sv203_line_item_charge_amount: String,
    pub sv204_unit_or_basis_for_measurement_code: String,
    pub sv205_service_unit_count: String,
    pub sv206_unit_rate: String,
    pub sv207_amount: String,
    pub sv208_yes_no_condition_or_response_code: String,
    pub sv209_nursing_home_residential_status_code: String,
    pub sv210_level_of_care_code: String,
}

pub fn get_sv2(sv2_content: String) -> SV2 {
    let sv2_parts: Vec<&str> = sv2_content.split("*").collect();
    
    let mut sv2 = SV2::default();
    
    if sv2_parts.len() > 0 && !sv2_parts[0].is_empty() {
        sv2.sv201_service_line_revenue_code = sv2_parts[0].to_string();
    }
    
    if sv2_parts.len() > 1 && !sv2_parts[1].is_empty() {
        sv2.sv202_procedure_code = sv2_parts[1].to_string();
    }
    
    if sv2_parts.len() > 2 && !sv2_parts[2].is_empty() {
        sv2.sv203_line_item_charge_amount = sv2_parts[2].to_string();
    }
    
    if sv2_parts.len() > 3 && !sv2_parts[3].is_empty() {
        sv2.sv204_unit_or_basis_for_measurement_code = sv2_parts[3].to_string();
    }
    
    if sv2_parts.len() > 4 && !sv2_parts[4].is_empty() {
        sv2.sv205_service_unit_count = sv2_parts[4].to_string();
    }
    
    if sv2_parts.len() > 5 && !sv2_parts[5].is_empty() {
        sv2.sv206_unit_rate = sv2_parts[5].to_string();
    }
    
    if sv2_parts.len() > 6 && !sv2_parts[6].is_empty() {
        sv2.sv207_amount = sv2_parts[6].to_string();
    }
    
    if sv2_parts.len() > 7 && !sv2_parts[7].is_empty() {
        sv2.sv208_yes_no_condition_or_response_code = sv2_parts[7].to_string();
    }
    
    if sv2_parts.len() > 8 && !sv2_parts[8].is_empty() {
        sv2.sv209_nursing_home_residential_status_code = sv2_parts[8].to_string();
    }
    
    if sv2_parts.len() > 9 && !sv2_parts[9].is_empty() {
        sv2.sv210_level_of_care_code = sv2_parts[9].to_string();
    }
    
    sv2
}

pub fn write_sv2(sv2: SV2) -> String {
    let mut sv2_content = String::new();
    sv2_content.push_str("SV2*");
    sv2_content.push_str(&sv2.sv201_service_line_revenue_code);
    sv2_content.push_str("*");
    sv2_content.push_str(&sv2.sv202_procedure_code);
    
    if !sv2.sv203_line_item_charge_amount.is_empty() || 
       !sv2.sv204_unit_or_basis_for_measurement_code.is_empty() || 
       !sv2.sv205_service_unit_count.is_empty() || 
       !sv2.sv206_unit_rate.is_empty() || 
       !sv2.sv207_amount.is_empty() || 
       !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
       !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
       !sv2.sv210_level_of_care_code.is_empty() {
        
        sv2_content.push_str("*");
        sv2_content.push_str(&sv2.sv203_line_item_charge_amount);
        
        if !sv2.sv204_unit_or_basis_for_measurement_code.is_empty() || 
           !sv2.sv205_service_unit_count.is_empty() || 
           !sv2.sv206_unit_rate.is_empty() || 
           !sv2.sv207_amount.is_empty() || 
           !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
           !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
           !sv2.sv210_level_of_care_code.is_empty() {
            
            sv2_content.push_str("*");
            sv2_content.push_str(&sv2.sv204_unit_or_basis_for_measurement_code);
            
            if !sv2.sv205_service_unit_count.is_empty() || 
               !sv2.sv206_unit_rate.is_empty() || 
               !sv2.sv207_amount.is_empty() || 
               !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
               !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
               !sv2.sv210_level_of_care_code.is_empty() {
                
                sv2_content.push_str("*");
                sv2_content.push_str(&sv2.sv205_service_unit_count);
                
                if !sv2.sv206_unit_rate.is_empty() || 
                   !sv2.sv207_amount.is_empty() || 
                   !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
                   !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
                   !sv2.sv210_level_of_care_code.is_empty() {
                    
                    sv2_content.push_str("*");
                    sv2_content.push_str(&sv2.sv206_unit_rate);
                    
                    if !sv2.sv207_amount.is_empty() || 
                       !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
                       !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
                       !sv2.sv210_level_of_care_code.is_empty() {
                        
                        sv2_content.push_str("*");
                        sv2_content.push_str(&sv2.sv207_amount);
                        
                        if !sv2.sv208_yes_no_condition_or_response_code.is_empty() || 
                           !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
                           !sv2.sv210_level_of_care_code.is_empty() {
                            
                            sv2_content.push_str("*");
                            sv2_content.push_str(&sv2.sv208_yes_no_condition_or_response_code);
                            
                            if !sv2.sv209_nursing_home_residential_status_code.is_empty() || 
                               !sv2.sv210_level_of_care_code.is_empty() {
                                
                                sv2_content.push_str("*");
                                sv2_content.push_str(&sv2.sv209_nursing_home_residential_status_code);
                                
                                if !sv2.sv210_level_of_care_code.is_empty() {
                                    sv2_content.push_str("*");
                                    sv2_content.push_str(&sv2.sv210_level_of_care_code);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    sv2_content.push_str("~");
    sv2_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_sv2() {
        let sv2_content = "*HC:33510".to_string();
        let sv2 = get_sv2(sv2_content);
        assert_eq!(sv2.sv201_service_line_revenue_code, "");
        assert_eq!(sv2.sv202_procedure_code, "HC:33510");
    }
    
    #[test]
    fn test_write_sv2() {
        let sv2 = SV2 {
            sv201_service_line_revenue_code: "".to_string(),
            sv202_procedure_code: "HC:33510".to_string(),
            sv203_line_item_charge_amount: "".to_string(),
            sv204_unit_or_basis_for_measurement_code: "".to_string(),
            sv205_service_unit_count: "".to_string(),
            sv206_unit_rate: "".to_string(),
            sv207_amount: "".to_string(),
            sv208_yes_no_condition_or_response_code: "".to_string(),
            sv209_nursing_home_residential_status_code: "".to_string(),
            sv210_level_of_care_code: "".to_string(),
        };
        
        let sv2_content = write_sv2(sv2);
        assert_eq!(sv2_content, "SV2**HC:33510~");
    }
}
