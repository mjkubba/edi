use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::st::*;
use crate::segments::bht::*;
use crate::helper::edihelper::*;
use crate::error::{EdiResult, EdiError};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1 {
    pub st_segments: ST,
    pub bht_segments: BHT,
}

pub fn get_table1(mut contents: String) -> EdiResult<(Table1, String)> {
    let mut table1 = Table1::default();
    
    // Process ST segment (required)
    if contents.contains("ST") {
        info!("ST segment found");
        let st_content = get_segment_contents("ST", &contents);
        if st_content.is_empty() {
            return Err(EdiError::MissingSegment("ST".to_string()));
        }
        table1.st_segments = get_st(st_content);
        
        // Verify this is a 271 transaction set
        if table1.st_segments.transaction_set_id != "271" {
            return Err(EdiError::ValidationError(format!(
                "Expected transaction set ID '271', got '{}'",
                table1.st_segments.transaction_set_id
            )));
        }
        
        info!("ST segment parsed");
        contents = content_trim("ST", contents);
    } else {
        return Err(EdiError::MissingSegment("ST".to_string()));
    }
    
    // Process BHT segment (required)
    if contents.contains("BHT") {
        info!("BHT segment found");
        let bht_content = get_segment_contents("BHT", &contents);
        if bht_content.is_empty() {
            return Err(EdiError::MissingSegment("BHT".to_string()));
        }
        table1.bht_segments = get_bht(bht_content);
        info!("BHT segment parsed");
        contents = content_trim("BHT", contents);
    } else {
        return Err(EdiError::MissingSegment("BHT".to_string()));
    }
    
    info!("Table 1 parsed");
    Ok((table1, contents))
}

pub fn write_table1(table1: &Table1) -> String {
    let mut contents = String::new();
    
    // Write ST segment
    contents.push_str(&write_st(table1.st_segments.clone()));
    
    // Write BHT segment
    contents.push_str(&write_bht(table1.bht_segments.clone()));
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_table1() -> EdiResult<()> {
        let contents = "ST*271*1234*005010X279A1~BHT*0022*11*10001234*20060501*1319*DG~".to_string();
        let (table1, remaining) = get_table1(contents)?;
        
        assert_eq!(table1.st_segments.transaction_set_id, "271");
        assert_eq!(table1.st_segments.transaction_set_control_number, "1234");
        assert_eq!(table1.st_segments.implementation_conven_ref, "005010X279A1");
        
        assert_eq!(table1.bht_segments.bht01_hierarchical_structure_code, "0022");
        assert_eq!(table1.bht_segments.bht02_transaction_set_purpose_code, "11");
        assert_eq!(table1.bht_segments.bht03_reference_identification, "10001234");
        assert_eq!(table1.bht_segments.bht04_date, "20060501");
        assert_eq!(table1.bht_segments.bht05_time, "1319");
        assert_eq!(table1.bht_segments.bht06_transaction_type_code, "DG");
        
        assert_eq!(remaining, "");
        
        Ok(())
    }
    
    #[test]
    fn test_write_table1() -> EdiResult<()> {
        let table1 = Table1 {
            st_segments: ST {
                transaction_set_id: "271".to_string(),
                transaction_set_control_number: "1234".to_string(),
                implementation_conven_ref: "005010X279A1".to_string(),
            },
            bht_segments: BHT {
                bht01_hierarchical_structure_code: "0022".to_string(),
                bht02_transaction_set_purpose_code: "11".to_string(),
                bht03_reference_identification: "10001234".to_string(),
                bht04_date: "20060501".to_string(),
                bht05_time: "1319".to_string(),
                bht06_transaction_type_code: "DG".to_string(),
            },
        };
        
        let contents = write_table1(&table1);
        assert_eq!(contents, "ST*271*1234*005010X279A1~BHT*0022*11*10001234*20060501*1319*DG~");
        
        Ok(())
    }
}
