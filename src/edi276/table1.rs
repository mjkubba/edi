use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1s {
    pub st01_transaction_set_identifier_code: String,
    pub st02_transaction_set_control_number: String,
    pub st03_implementation_convention_reference: String,
    pub bht01_hierarchical_structure_code: String,
    pub bht02_transaction_set_purpose_code: String,
    pub bht03_reference_identification: String,
    pub bht04_date: String,
    pub bht05_time: String,
    pub bht06_transaction_type_code: String,
}

pub fn get_table1s(contents: String) -> (Table1s, String) {
    let mut table1s = Table1s::default();
    let mut remaining_content = contents.clone();

    // Process ST segment
    if let Some(st_segment_start) = contents.find("ST") {
        let st_segment_end = contents[st_segment_start..].find('~').unwrap_or(contents.len() - st_segment_start);
        let st_segment = &contents[st_segment_start..st_segment_start + st_segment_end];
        
        let st_elements: Vec<&str> = st_segment.split('*').collect();
        
        if st_elements.len() >= 3 {
            table1s.st01_transaction_set_identifier_code = st_elements[1].to_string();
            table1s.st02_transaction_set_control_number = st_elements[2].to_string();
            if st_elements.len() >= 4 {
                table1s.st03_implementation_convention_reference = st_elements[3].to_string();
            }
        }
        
        // Remove the ST segment from the remaining content
        remaining_content = contents[st_segment_start + st_segment_end + 1..].to_string();
    }

    // Process BHT segment
    if let Some(bht_segment_start) = remaining_content.find("BHT") {
        let bht_segment_end = remaining_content[bht_segment_start..].find('~').unwrap_or(remaining_content.len() - bht_segment_start);
        let bht_segment = &remaining_content[bht_segment_start..bht_segment_start + bht_segment_end];
        
        let bht_elements: Vec<&str> = bht_segment.split('*').collect();
        
        if bht_elements.len() >= 7 {
            table1s.bht01_hierarchical_structure_code = bht_elements[1].to_string();
            table1s.bht02_transaction_set_purpose_code = bht_elements[2].to_string();
            table1s.bht03_reference_identification = bht_elements[3].to_string();
            table1s.bht04_date = bht_elements[4].to_string();
            table1s.bht05_time = bht_elements[5].to_string();
            table1s.bht06_transaction_type_code = bht_elements[6].to_string();
        }
        
        // Remove the BHT segment from the remaining content
        remaining_content = remaining_content[bht_segment_start + bht_segment_end + 1..].to_string();
    }
    
    (table1s, remaining_content)
}

pub fn write_table1(table1s: &Table1s) -> String {
    let mut result = String::new();
    
    // Write ST segment
    let st_segment = format!(
        "ST*{}*{}*{}~",
        table1s.st01_transaction_set_identifier_code,
        table1s.st02_transaction_set_control_number,
        table1s.st03_implementation_convention_reference
    );
    result.push_str(&st_segment);
    
    // Write BHT segment
    let bht_segment = format!(
        "BHT*{}*{}*{}*{}*{}*{}~",
        table1s.bht01_hierarchical_structure_code,
        table1s.bht02_transaction_set_purpose_code,
        table1s.bht03_reference_identification,
        table1s.bht04_date,
        table1s.bht05_time,
        table1s.bht06_transaction_type_code
    );
    result.push_str(&bht_segment);
    
    result
}
