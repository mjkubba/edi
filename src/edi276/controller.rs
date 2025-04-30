use log::info;

use serde::{Serialize, Deserialize};

use crate::edi276::interchangecontrol::*;
use crate::edi276::table1::*;
use crate::edi276::loop2000::*;
use crate::edi276::interchangecontroltrailer::*;
use crate::error::EdiResult;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1Combined {
    pub table1: Table1s,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi276 {
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segment: String,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_276(mut contents: String) -> EdiResult<Edi276> {
    let interchange_header;
    let table1s;
    let loop2000a;
    let loop2000b_vec;
    let interchange_trailer;
    let table1_combined;
    let se_segment;
    
    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();
    
    // Remove carriage returns and line feeds
    contents = contents.replace("\r", "").replace("\n", "");

    // Control Segments
    (interchange_header, contents) = get_interchange_header(contents.clone());

    // Table 1
    (table1s, contents) = get_table1s(contents.clone());
    
    // Loop 2000A - Information Source
    (loop2000a, contents) = get_loop_2000a(contents.clone());
    
    // Loop 2000B - Information Receiver
    (loop2000b_vec, contents) = get_loop_2000b_vec(contents.clone());
    
    // Extract SE segment
    if let Some(se_segment_start) = contents.find("SE") {
        let se_segment_end = contents[se_segment_start..].find('~').unwrap_or(contents.len() - se_segment_start);
        se_segment = contents[se_segment_start..se_segment_start + se_segment_end].to_string();
        contents = contents[se_segment_start + se_segment_end + 1..].to_string();
    } else {
        se_segment = String::new();
    }

    // Control Trailer
    (interchange_trailer, contents) = get_interchange_trailer(contents.clone());

    // Combined Table 1
    table1_combined = Table1Combined {
        table1: table1s.clone(),
    };

    let edi276 = Edi276 {
        interchange_header,
        table1_combined,
        loop2000a,
        loop2000b: loop2000b_vec,
        se_segment,
        interchange_trailer,
    };
    
    info!("Unprocessed segments: {:?}", contents);
    Ok(edi276)
}

pub fn write_276(edi276: &Edi276) -> String {
    let mut new_edi = String::new();
    
    // Write interchange header
    let new_ich = write_interchange_control(&edi276.interchange_header);
    new_edi.push_str(&new_ich);
    
    // Write Table 1
    let new_table1s = write_table1(&edi276.table1_combined.table1);
    new_edi.push_str(&new_table1s);
    
    // Write Loop 2000A
    let new_loop2000a = write_loop_2000a(&edi276.loop2000a);
    new_edi.push_str(&new_loop2000a);
    
    // Write Loop 2000B
    let new_loop2000b = write_loop_2000b_vec(&edi276.loop2000b);
    new_edi.push_str(&new_loop2000b);
    
    // Write SE segment
    new_edi.push_str(&edi276.se_segment);
    new_edi.push_str("~");
    
    // Write interchange trailer
    let new_ict = write_interchange_trailer(&edi276.interchange_trailer);
    new_edi.push_str(&new_ict);
    
    info!("Generated EDI 276: {}", new_edi);
    new_edi
}

// Function to detect if JSON contains 276 format data
pub fn is_276_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 276 format
    contents.contains("\"st01_transaction_set_identifier_code\":\"276\"") || 
    contents.contains("\"bht06_transaction_type_code\":\"13\"")
}
