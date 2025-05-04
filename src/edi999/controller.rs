use log::info;

use serde::{Serialize, Deserialize};

use crate::edi999::interchangecontrol::*;
use crate::edi999::table1::*;
use crate::edi999::loop2000::*;
use crate::edi999::table1trailer::*;
use crate::edi999::interchangecontroltrailer::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1Combined{
    pub table1: Table1s,
    pub loop2000s: Vec<Loop2000>,
    pub table1trailer: Table1trailer,
}

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Edi999{
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_999(mut contents: String) -> (Edi999, String) {
    let interchange_header;
    let table1s;
    let loop2000s;
    let table1trailer;
    let interchange_trailer;
    let table1_combined;
    
    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();
    
    // Remove carriage returns and line feeds
    contents = contents.replace("\r", "").replace("\n", "");

    // Control Segments
    (interchange_header, contents) = get_interchange_header(contents.clone());

    // Table 1
    (table1s, contents) = get_table1s(contents.clone());
    
    // loop 2000
    (loop2000s, contents) = get_loop_2000s(contents.clone());
    
    // Table 1 trailer
    (table1trailer, contents) = get_table1trailer(contents.clone());

    // Control Trailer
    (interchange_trailer, contents) = get_interchange_trailer(contents.clone());

    // Combined Table 1 and Loop 2000
    table1_combined = Table1Combined{
        table1: table1s.clone(),
        loop2000s: loop2000s.clone(),
        table1trailer: table1trailer.clone(),
    };

    let edi999 = Edi999 {
        interchange_header,
        interchange_trailer,
        table1_combined,
    };
    
    info!("Unprocessed segments: {:?}", contents);
    (edi999, contents)
}

pub fn write_999(edi999: &Edi999) -> String {
    let mut new_edi = String::new();
    
    // Write interchange header
    let new_ich = write_interchange_control(&edi999.interchange_header);
    new_edi.push_str(&new_ich);
    
    // Write Table 1
    let new_table1s = write_table1(&edi999.table1_combined.table1);
    new_edi.push_str(&new_table1s);
    
    // Write Loop 2000s
    let new_loop2000s = write_loop2000(edi999.table1_combined.loop2000s.clone());
    new_edi.push_str(&new_loop2000s);
    
    // Write Table 1 trailer
    let new_table1trailer = write_table1trailer(&edi999.table1_combined.table1trailer);
    new_edi.push_str(&new_table1trailer);
    
    // Write interchange trailer
    let new_ict = write_interchange_trailer(&edi999.interchange_trailer);
    new_edi.push_str(&new_ict);
    
    // Add line breaks between segments for better readability
    let new_edi_with_breaks = new_edi.replace("~", "~\n");
    
    info!("Generated EDI 999: {}", new_edi_with_breaks);
    new_edi_with_breaks
}

// Function to detect if JSON contains 999 format data
#[allow(dead_code)]
pub fn is_999_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 999 format
    contents.contains("\"transaction_set_id\":\"999\"") || 
    contents.contains("\"ak01_functional_id_group\":") ||
    contents.contains("\"ak201_transaction_set_identifier_code\":")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segments::ctx::CTX;
    use crate::edi999::loop2100::Loop2100;
    use crate::edi999::loop2000::Loop2000;
    
    #[test]
    fn test_write_999_with_ctx_segments() {
        // Create a minimal Edi999 structure with CTX segments
        let mut edi999 = Edi999::default();
        
        // Create a Loop2100 with CTX segments
        let mut loop2100 = Loop2100::default();
        
        // Add a special format CTX segment
        let ctx_special = CTX {
            ctx01_context_name: "CLM01:123456789".to_string(),
            ctx02_segment_id_code: "".to_string(),
            ctx03_segment_position_in_transaction: "".to_string(),
            ctx04_loop_id_code: "".to_string(),
            ctx05_position_in_segment: "".to_string(),
            ctx06_reference_in_segment: "".to_string(),
        };
        loop2100.ctx_segments.push(ctx_special);
        
        // Add a complex format CTX segment
        let ctx_complex = CTX {
            ctx01_context_name: "SITUATIONAL TRIGGER".to_string(),
            ctx02_segment_id_code: "CLM".to_string(),
            ctx03_segment_position_in_transaction: "43".to_string(),
            ctx04_loop_id_code: "".to_string(),
            ctx05_position_in_segment: "5:3".to_string(),
            ctx06_reference_in_segment: "C023:1325".to_string(),
        };
        loop2100.ctx_segments.push(ctx_complex);
        
        // Add the Loop2100 to a Loop2000
        let mut loop2000 = Loop2000::default();
        loop2000.loop2100s.push(loop2100);
        
        // Add the Loop2000 to the Edi999
        edi999.table1_combined.loop2000s.push(loop2000);
        
        // Write the EDI999
        let output = write_999(&edi999);
        
        // Check if CTX segments are formatted correctly
        assert!(output.contains("CTX*CLM01:123456789~"), "Special format CTX segment not found in output");
        assert!(output.contains("CTX*SITUATIONAL TRIGGER*CLM*43**5:3*C023:1325~"), 
                "Complex format CTX segment not found in output");
        
        // Check if line breaks are added
        assert!(output.contains("~\n"), "Line breaks not found in output");
        
        // Check if trailer segments have proper values
        assert!(output.contains("SE*16*"), "SE segment doesn't have proper values");
        assert!(output.contains("AK9*P*3*3*1~"), "AK9 segment doesn't have proper values");
        assert!(output.contains("GE*1*"), "GE segment doesn't have proper values");
        assert!(output.contains("IEA*1*"), "IEA segment doesn't have proper values");
    }
    
    #[test]
    fn test_is_999_json() {
        let json = r#"{"transaction_set_id":"999","ak01_functional_id_group":"HC"}"#;
        assert!(is_999_json(json));
        
        let json = r#"{"transaction_set_id":"835"}"#;
        assert!(!is_999_json(json));
    }
}
