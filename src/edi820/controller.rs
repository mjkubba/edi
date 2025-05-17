use log::info;
use serde::{Serialize, Deserialize};

use crate::edi820::interchangecontrol::*;
use crate::edi820::table1::*;
use crate::edi820::loop1000a::*;
use crate::edi820::loop1000b::*;
use crate::edi820::loop2000::*;
use crate::edi820::interchangecontroltrailer::*;
use crate::error::EdiResult;
use crate::transaction_processor::TransactionSet;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1Combined {
    pub table1: Table1s,
    pub loop1000as: Loop1000as,
    pub loop1000bs: Loop1000bs,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi820 {
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub table2s: Vec<Table2>,
    pub interchange_trailer: InterchangeTrailer,
    pub transaction_set_id: String,
}

impl TransactionSet for Edi820 {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        let mut edi820 = Edi820::default();
        edi820.transaction_set_id = "820".to_string();
        
        // Remove BOM if present
        let mut contents = contents.trim_start_matches("\u{feff}").to_string();
        
        // Remove carriage returns and line feeds
        contents = contents.replace("\r", "").replace("\n", "");
        
        // Parse interchange control header
        let (interchange_header, new_contents) = get_interchange_header(contents);
        edi820.interchange_header = interchange_header;
        contents = new_contents;
        
        // Parse table 1
        let (table1s, new_contents) = get_table1s(contents);
        contents = new_contents;
        
        // Parse loop 1000A (payer identification)
        let (loop1000as, new_contents) = get_1000as(contents);
        contents = new_contents;
        
        // Parse loop 1000B (payee identification)
        let (loop1000bs, new_contents) = get_1000bs(contents);
        contents = new_contents;
        
        // Combine Table 1
        edi820.table1_combined = Table1Combined {
            table1: table1s,
            loop1000as,
            loop1000bs,
        };
        
        // Parse loop 2000 (entity)
        let (table2s, new_contents) = get_loop_2000s(contents);
        edi820.table2s = table2s;
        contents = new_contents;
        
        // Parse interchange control trailer
        let (interchange_trailer, remaining) = get_interchange_trailer(contents);
        edi820.interchange_trailer = interchange_trailer;
        
        Ok((edi820, remaining))
    }
    
    fn to_edi(&self) -> String {
        let mut new_edi = String::new();
        
        // Write interchange header
        new_edi.push_str(&write_interchange_control(self.interchange_header.clone()));
        
        // Write table 1 segments
        new_edi.push_str(&write_table1(self.table1_combined.table1.clone()));
        
        // Write loop 1000A segments (payer identification)
        new_edi.push_str(&write_loop1000a(self.table1_combined.loop1000as.clone()));
        
        // Write loop 1000B segments (payee identification)
        new_edi.push_str(&write_loop1000b(self.table1_combined.loop1000bs.clone()));
        
        // Write loop 2000 segments (entity)
        new_edi.push_str(&write_loop2000(self.table2s.clone()));
        
        // Write interchange control trailer
        new_edi.push_str(&write_interchange_trailer(self.interchange_trailer.clone()));
        
        info!("Generated EDI 820: {}", new_edi);
        new_edi
    }
    
    fn get_transaction_type() -> &'static str {
        "820"
    }
    
    fn detect(contents: &str) -> bool {
        is_820(contents)
    }
}

pub fn get_820(contents: String) -> EdiResult<Edi820> {
    match Edi820::parse(contents) {
        Ok((edi820, _)) => Ok(edi820),
        Err(e) => Err(e),
    }
}

pub fn write_820(edi820: &Edi820) -> String {
    edi820.to_edi()
}

// Function to detect if content is EDI 820 format
pub fn is_820(contents: &str) -> bool {
    // Check if the content contains ST*820 which indicates an EDI 820 transaction set
    if contents.contains("ST*820") {
        return true;
    }
    
    // Check for other 820-specific indicators
    if contents.contains("BPR*") && (contents.contains("ENT*") || contents.contains("RMR*")) {
        return true;
    }
    
    false
}

// Function to detect if content is EDI 820 JSON
pub fn is_820_json(contents: &str) -> bool {
    contents.contains("\"transaction_set_id\":\"820\"")
}
