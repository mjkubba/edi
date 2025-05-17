use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100 {
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub rmr_segments: Vec<RMR>,
    pub dtm_segments: Vec<DTM>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NM1 {
    pub entity_id: String,
    pub entity_type: String,
    pub lastname: String,
    pub firstname: String,
    pub middle_initial: String,
    pub suffix: String,
    pub title: String,
    pub id_code_qualifier: String,
    pub id_code: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct REF {
    pub reference_id_qualifier: String,
    pub reference_id: String,
    pub description: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RMR {
    pub rmr01_reference_id_qualifier: String,
    pub rmr02_reference_id: String,
    pub rmr03_payment_action_code: String,
    pub rmr04_monetary_amount: String,
    pub rmr05_credit_debit_flag_code: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTM {
    pub dtm01_date_time_qualifier: String,
    pub dtm02_date: String,
    pub dtm03_time: String,
    pub dtm04_time_code: String,
    pub dtm05_date_time_period_format_qualifier: String,
    pub dtm06_date_time_period: String,
}

pub fn get_loop_2100s(mut contents: String) -> (Vec<Loop2100>, String) {
    let mut loop2100s = Vec::new();
    let original_length = contents.len();
    let mut safety_counter = 0;
    let max_iterations = 100; // Safety limit to prevent infinite loops
    
    // Parse NM1 segments and their associated data
    while contents.contains("NM1") && 
          safety_counter < max_iterations && 
          contents.len() < original_length + 100 { // Additional safety check
        
        safety_counter += 1;
        info!("Loop2100 iteration {}, content length: {}", safety_counter, contents.len());
        
        // Check if this NM1 is part of the current loop or a new section
        if !check_if_segement_in_loop("NM1", "ENT", contents.clone()) && 
           !check_if_segement_in_loop("NM1", "SE", contents.clone()) {
            info!("NM1 segment is not part of the current loop, breaking");
            break;
        }
        
        let nm1_content = get_segment_contents("NM1", &contents);
        let nm1_parts: Vec<&str> = nm1_content.split('*').collect();
        
        // Check if this is an individual (NM101=IL)
        if nm1_parts.len() > 1 && nm1_parts[1] == "IL" {
            info!("NM1 segment found for Individual, ");
            
            let nm1_segment = NM1 {
                entity_id: nm1_parts[1].to_string(),
                entity_type: if nm1_parts.len() > 2 { nm1_parts[2].to_string() } else { String::new() },
                lastname: if nm1_parts.len() > 3 { nm1_parts[3].to_string() } else { String::new() },
                firstname: if nm1_parts.len() > 4 { nm1_parts[4].to_string() } else { String::new() },
                middle_initial: if nm1_parts.len() > 5 { nm1_parts[5].to_string() } else { String::new() },
                suffix: if nm1_parts.len() > 6 { nm1_parts[6].to_string() } else { String::new() },
                title: if nm1_parts.len() > 7 { nm1_parts[7].to_string() } else { String::new() },
                id_code_qualifier: if nm1_parts.len() > 8 { nm1_parts[8].to_string() } else { String::new() },
                id_code: if nm1_parts.len() > 9 { nm1_parts[9].to_string() } else { String::new() },
            };
            
            info!("NM1 segment parsed");
            let old_contents = contents.clone();
            contents = content_trim("NM1", contents);
            
            // Safety check - ensure content is actually being trimmed
            if contents == old_contents {
                info!("Warning: Content not trimmed, breaking to avoid infinite loop");
                break;
            }
            
            let mut ref_segments = Vec::new();
            let mut rmr_segments = Vec::new();
            let mut dtm_segments = Vec::new();
            let mut ref_safety_counter = 0;
            
            // Parse REF segments
            while contents.contains("REF") && 
                  ref_safety_counter < 20 && // Safety limit for REF segments
                  check_if_segement_in_loop("REF", "RMR", contents.clone()) && 
                  check_if_segement_in_loop("REF", "DTM", contents.clone()) && 
                  check_if_segement_in_loop("REF", "NM1", contents.clone()) && 
                  check_if_segement_in_loop("REF", "ENT", contents.clone()) && 
                  check_if_segement_in_loop("REF", "SE", contents.clone()) {
                
                ref_safety_counter += 1;
                info!("REF segment found, iteration {}", ref_safety_counter);
                let ref_content = get_segment_contents("REF", &contents);
                let ref_parts: Vec<&str> = ref_content.split('*').collect();
                
                let ref_segment = REF {
                    reference_id_qualifier: if ref_parts.len() > 1 { ref_parts[1].to_string() } else { String::new() },
                    reference_id: if ref_parts.len() > 2 { ref_parts[2].to_string() } else { String::new() },
                    description: if ref_parts.len() > 3 { ref_parts[3].to_string() } else { String::new() },
                };
                
                ref_segments.push(ref_segment);
                
                info!("REF segment parsed");
                let old_contents = contents.clone();
                contents = content_trim("REF", contents);
                
                // Safety check - ensure content is actually being trimmed
                if contents == old_contents {
                    info!("Warning: REF content not trimmed, breaking to avoid infinite loop");
                    break;
                }
            }
            
            let mut rmr_safety_counter = 0;
            
            // Parse RMR segments
            while contents.contains("RMR") && 
                  rmr_safety_counter < 20 && // Safety limit for RMR segments
                  check_if_segement_in_loop("RMR", "DTM", contents.clone()) && 
                  check_if_segement_in_loop("RMR", "NM1", contents.clone()) && 
                  check_if_segement_in_loop("RMR", "ENT", contents.clone()) && 
                  check_if_segement_in_loop("RMR", "SE", contents.clone()) {
                
                rmr_safety_counter += 1;
                info!("RMR segment found, iteration {}", rmr_safety_counter);
                let rmr_content = get_segment_contents("RMR", &contents);
                let rmr_parts: Vec<&str> = rmr_content.split('*').collect();
                
                let rmr_segment = RMR {
                    rmr01_reference_id_qualifier: if rmr_parts.len() > 1 { rmr_parts[1].to_string() } else { String::new() },
                    rmr02_reference_id: if rmr_parts.len() > 2 { rmr_parts[2].to_string() } else { String::new() },
                    rmr03_payment_action_code: if rmr_parts.len() > 3 { rmr_parts[3].to_string() } else { String::new() },
                    rmr04_monetary_amount: if rmr_parts.len() > 4 { rmr_parts[4].to_string() } else { String::new() },
                    rmr05_credit_debit_flag_code: if rmr_parts.len() > 5 { rmr_parts[5].to_string() } else { String::new() },
                };
                
                rmr_segments.push(rmr_segment);
                
                info!("RMR segment parsed");
                let old_contents = contents.clone();
                contents = content_trim("RMR", contents);
                
                // Safety check - ensure content is actually being trimmed
                if contents == old_contents {
                    info!("Warning: RMR content not trimmed, breaking to avoid infinite loop");
                    break;
                }
                
                let mut dtm_safety_counter = 0;
                
                // Parse DTM segments associated with this RMR
                while contents.contains("DTM") && 
                      dtm_safety_counter < 20 && // Safety limit for DTM segments
                      check_if_segement_in_loop("DTM", "RMR", contents.clone()) && 
                      check_if_segement_in_loop("DTM", "NM1", contents.clone()) && 
                      check_if_segement_in_loop("DTM", "ENT", contents.clone()) && 
                      check_if_segement_in_loop("DTM", "SE", contents.clone()) {
                    
                    dtm_safety_counter += 1;
                    info!("DTM segment found, iteration {}", dtm_safety_counter);
                    let dtm_content = get_segment_contents("DTM", &contents);
                    let dtm_parts: Vec<&str> = dtm_content.split('*').collect();
                    
                    let dtm_segment = DTM {
                        dtm01_date_time_qualifier: if dtm_parts.len() > 1 { dtm_parts[1].to_string() } else { String::new() },
                        dtm02_date: if dtm_parts.len() > 2 { dtm_parts[2].to_string() } else { String::new() },
                        dtm03_time: if dtm_parts.len() > 3 { dtm_parts[3].to_string() } else { String::new() },
                        dtm04_time_code: if dtm_parts.len() > 4 { dtm_parts[4].to_string() } else { String::new() },
                        dtm05_date_time_period_format_qualifier: if dtm_parts.len() > 5 { dtm_parts[5].to_string() } else { String::new() },
                        dtm06_date_time_period: if dtm_parts.len() > 6 { dtm_parts[6].to_string() } else { String::new() },
                    };
                    
                    dtm_segments.push(dtm_segment);
                    
                    info!("DTM segment parsed");
                    let old_contents = contents.clone();
                    contents = content_trim("DTM", contents);
                    
                    // Safety check - ensure content is actually being trimmed
                    if contents == old_contents {
                        info!("Warning: DTM content not trimmed, breaking to avoid infinite loop");
                        break;
                    }
                }
            }
            
            let loop2100 = Loop2100 {
                nm1_segments: nm1_segment,
                ref_segments,
                rmr_segments,
                dtm_segments,
            };
            
            loop2100s.push(loop2100);
        } else {
            // Skip this NM1 segment as it's not part of Loop2100
            info!("NM1 segment is not for an individual (IL), breaking");
            break;
        }
    }
    
    if safety_counter >= max_iterations {
        info!("Warning: Maximum iterations reached in get_loop_2100s, possible infinite loop detected");
    }
    
    info!("Loop 2100 parsed, found {} loops", loop2100s.len());
    
    return (loop2100s, contents);
}

pub fn write_loop2100s(loop2100s: Vec<Loop2100>) -> String {
    let mut result = String::new();
    
    for loop2100 in loop2100s {
        // Write NM1 segment
        result.push_str("NM1*");
        result.push_str(&loop2100.nm1_segments.entity_id);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.entity_type);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.lastname);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.firstname);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.middle_initial);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.suffix);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.title);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.id_code_qualifier);
        result.push_str("*");
        result.push_str(&loop2100.nm1_segments.id_code);
        result.push_str("~\n");
        
        // Write REF segments
        for ref_segment in &loop2100.ref_segments {
            result.push_str("REF*");
            result.push_str(&ref_segment.reference_id_qualifier);
            result.push_str("*");
            result.push_str(&ref_segment.reference_id);
            
            if !ref_segment.description.is_empty() {
                result.push_str("*");
                result.push_str(&ref_segment.description);
            }
            
            result.push_str("~\n");
        }
        
        // Write RMR and associated DTM segments
        for (i, rmr_segment) in loop2100.rmr_segments.iter().enumerate() {
            result.push_str("RMR*");
            result.push_str(&rmr_segment.rmr01_reference_id_qualifier);
            result.push_str("*");
            result.push_str(&rmr_segment.rmr02_reference_id);
            
            if !rmr_segment.rmr03_payment_action_code.is_empty() ||
               !rmr_segment.rmr04_monetary_amount.is_empty() ||
               !rmr_segment.rmr05_credit_debit_flag_code.is_empty() {
                
                result.push_str("*");
                result.push_str(&rmr_segment.rmr03_payment_action_code);
                
                if !rmr_segment.rmr04_monetary_amount.is_empty() ||
                   !rmr_segment.rmr05_credit_debit_flag_code.is_empty() {
                    
                    result.push_str("*");
                    result.push_str(&rmr_segment.rmr04_monetary_amount);
                    
                    if !rmr_segment.rmr05_credit_debit_flag_code.is_empty() {
                        result.push_str("*");
                        result.push_str(&rmr_segment.rmr05_credit_debit_flag_code);
                    }
                }
            }
            
            result.push_str("~\n");
            
            // Write DTM segments associated with this RMR
            // For simplicity, we'll assume DTM segments are in the same order as RMR segments
            // In a more complex implementation, you might want to match DTM to RMR based on some identifier
            if i < loop2100.dtm_segments.len() {
                let dtm_segment = &loop2100.dtm_segments[i];
                
                result.push_str("DTM*");
                result.push_str(&dtm_segment.dtm01_date_time_qualifier);
                
                if !dtm_segment.dtm02_date.is_empty() ||
                   !dtm_segment.dtm03_time.is_empty() ||
                   !dtm_segment.dtm04_time_code.is_empty() ||
                   !dtm_segment.dtm05_date_time_period_format_qualifier.is_empty() ||
                   !dtm_segment.dtm06_date_time_period.is_empty() {
                    
                    result.push_str("*");
                    result.push_str(&dtm_segment.dtm02_date);
                    
                    if !dtm_segment.dtm03_time.is_empty() ||
                       !dtm_segment.dtm04_time_code.is_empty() ||
                       !dtm_segment.dtm05_date_time_period_format_qualifier.is_empty() ||
                       !dtm_segment.dtm06_date_time_period.is_empty() {
                        
                        result.push_str("*");
                        result.push_str(&dtm_segment.dtm03_time);
                        
                        if !dtm_segment.dtm04_time_code.is_empty() ||
                           !dtm_segment.dtm05_date_time_period_format_qualifier.is_empty() ||
                           !dtm_segment.dtm06_date_time_period.is_empty() {
                            
                            result.push_str("*");
                            result.push_str(&dtm_segment.dtm04_time_code);
                            
                            if !dtm_segment.dtm05_date_time_period_format_qualifier.is_empty() ||
                               !dtm_segment.dtm06_date_time_period.is_empty() {
                                
                                result.push_str("*");
                                result.push_str(&dtm_segment.dtm05_date_time_period_format_qualifier);
                                
                                if !dtm_segment.dtm06_date_time_period.is_empty() {
                                    result.push_str("*");
                                    result.push_str(&dtm_segment.dtm06_date_time_period);
                                }
                            }
                        }
                    }
                }
                
                result.push_str("~\n");
            }
        }
    }
    
    return result;
}
