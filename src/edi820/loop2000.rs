use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;
use crate::edi820::loop2100::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table2 {
    pub ent_segments: ENT,
    pub loop2100s: Vec<Loop2100>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ENT {
    pub ent01_assigned_number: String,
    pub ent02_entity_id_code: String,
    pub ent03_identification_code_qualifier: String,
    pub ent04_identification_code: String,
    pub ent05_entity_type_qualifier: String,
    pub ent06_entity_type: String,
}

pub fn get_loop_2000s(mut contents: String) -> (Vec<Table2>, String) {
    let mut table2s = Vec::new();
    let original_length = contents.len();
    let mut safety_counter = 0;
    let max_iterations = 100; // Safety limit to prevent infinite loops
    
    // Parse ENT segments and their associated Loop2100s
    while contents.contains("ENT") && 
          safety_counter < max_iterations && 
          contents.len() < original_length + 100 { // Additional safety check
        
        safety_counter += 1;
        info!("Loop2000 iteration {}, content length: {}", safety_counter, contents.len());
        
        let ent_content = get_segment_contents("ENT", &contents);
        let ent_parts: Vec<&str> = ent_content.split('*').collect();
        
        let ent_segment = ENT {
            ent01_assigned_number: if ent_parts.len() > 1 { ent_parts[1].to_string() } else { String::new() },
            ent02_entity_id_code: if ent_parts.len() > 2 { ent_parts[2].to_string() } else { String::new() },
            ent03_identification_code_qualifier: if ent_parts.len() > 3 { ent_parts[3].to_string() } else { String::new() },
            ent04_identification_code: if ent_parts.len() > 4 { ent_parts[4].to_string() } else { String::new() },
            ent05_entity_type_qualifier: if ent_parts.len() > 5 { ent_parts[5].to_string() } else { String::new() },
            ent06_entity_type: if ent_parts.len() > 6 { ent_parts[6].to_string() } else { String::new() },
        };
        
        info!("ENT segment parsed");
        let old_contents = contents.clone();
        contents = content_trim("ENT", contents);
        
        // Safety check - ensure content is actually being trimmed
        if contents == old_contents {
            info!("Warning: ENT content not trimmed, breaking to avoid infinite loop");
            break;
        }
        
        // Parse Loop2100s associated with this ENT
        let (loop2100s, new_contents) = get_loop_2100s(contents.clone());
        
        // Safety check - ensure we're making progress
        if new_contents == contents {
            info!("Warning: No progress made in parsing Loop2100s, breaking to avoid infinite loop");
            contents = new_contents;
            break;
        }
        
        contents = new_contents;
        
        let table2 = Table2 {
            ent_segments: ent_segment,
            loop2100s,
        };
        
        table2s.push(table2);
    }
    
    if safety_counter >= max_iterations {
        info!("Warning: Maximum iterations reached in get_loop_2000s, possible infinite loop detected");
    }
    
    info!("Loop 2000 parsed, found {} entities", table2s.len());
    
    return (table2s, contents);
}

pub fn write_loop2000(table2s: Vec<Table2>) -> String {
    let mut result = String::new();
    
    for table2 in table2s {
        // Write ENT segment
        result.push_str("ENT*");
        result.push_str(&table2.ent_segments.ent01_assigned_number);
        
        if !table2.ent_segments.ent02_entity_id_code.is_empty() ||
           !table2.ent_segments.ent03_identification_code_qualifier.is_empty() ||
           !table2.ent_segments.ent04_identification_code.is_empty() ||
           !table2.ent_segments.ent05_entity_type_qualifier.is_empty() ||
           !table2.ent_segments.ent06_entity_type.is_empty() {
            
            result.push_str("*");
            result.push_str(&table2.ent_segments.ent02_entity_id_code);
            
            if !table2.ent_segments.ent03_identification_code_qualifier.is_empty() ||
               !table2.ent_segments.ent04_identification_code.is_empty() ||
               !table2.ent_segments.ent05_entity_type_qualifier.is_empty() ||
               !table2.ent_segments.ent06_entity_type.is_empty() {
                
                result.push_str("*");
                result.push_str(&table2.ent_segments.ent03_identification_code_qualifier);
                
                if !table2.ent_segments.ent04_identification_code.is_empty() ||
                   !table2.ent_segments.ent05_entity_type_qualifier.is_empty() ||
                   !table2.ent_segments.ent06_entity_type.is_empty() {
                    
                    result.push_str("*");
                    result.push_str(&table2.ent_segments.ent04_identification_code);
                    
                    if !table2.ent_segments.ent05_entity_type_qualifier.is_empty() ||
                       !table2.ent_segments.ent06_entity_type.is_empty() {
                        
                        result.push_str("*");
                        result.push_str(&table2.ent_segments.ent05_entity_type_qualifier);
                        
                        if !table2.ent_segments.ent06_entity_type.is_empty() {
                            result.push_str("*");
                            result.push_str(&table2.ent_segments.ent06_entity_type);
                        }
                    }
                }
            }
        }
        
        result.push_str("~\n");
        
        // Write Loop2100s
        result.push_str(&write_loop2100s(table2.loop2100s));
    }
    
    return result;
}
