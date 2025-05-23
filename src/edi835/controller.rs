use log::info;

use serde::{Serialize, Deserialize};

use crate::edi835::interchangecontrol::*;
use crate::edi835::table1::*;
use crate::edi835::loop1000a::*;
use crate::edi835::loop1000b::*;
use crate::edi835::loop2000::*;
use crate::edi835::table3::*;
use crate::edi835::interchangecontroltrailer::*;


#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1 {
    pub table1: Table1s,
    pub loop1000as: Loop1000as,
    pub loop1000bs: Loop1000bs,
}


#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Edi835{
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub table2s: Vec<Table2>,
    pub table3s: Table3s,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_835(mut contents: String) -> Edi835 {
    let interchange_header;
    let table1s;
    let loop1000as;
    let loop1000bs;
    let table2s;
    let table3s;
    let interchange_trailer;
    let table1;

    // Control Segments
    (interchange_header, contents) = get_interchange_header(contents.clone());

    // Table 1
    (table1s, contents) = get_table1s(contents.clone());
    
    // Loop 1000A Payer Identification
    (loop1000as, contents) = get_1000as(contents.clone());
    
    // Loop 1000B Payee Identification
    (loop1000bs, contents) = get_1000bs(contents.clone());


    // table 1 combined
    table1 = Table1 {
        table1: table1s.clone(),
        loop1000as: loop1000as.clone(),
        loop1000bs: loop1000bs.clone(),
    };

    // loop 2000
    (table2s, contents) = get_loop_2000s(contents.clone());

    // Table 3
    (table3s, contents) = get_table3s(contents.clone());

    // Control Trailer
    (interchange_trailer, contents) = get_interchange_trailer(contents.clone());

    let edi835 = Edi835 {
        interchange_header,
        table1,
        table2s,
        table3s,
        interchange_trailer,
    };
    

    info!("Unprocessed segments: {:?}", contents);
    edi835
}


pub fn write_835(contents: String) -> String {
    let edi_json: Edi835 = serde_json::from_str(&contents.clone()).unwrap();
    let mut new_edi = String::new();
    
    // Write interchange control header
    new_edi.push_str(&write_interchange_control(edi_json.interchange_header.clone()));
    
    // Write table 1 segments
    new_edi.push_str(&write_table1(edi_json.table1.table1.clone()));
    
    // Write loop 1000A segments (payer identification)
    new_edi.push_str(&write_loop1000a(edi_json.table1.loop1000as.clone()));
    
    // Write loop 1000B segments (payee identification)
    new_edi.push_str(&write_loop1000b(edi_json.table1.loop1000bs.clone()));
    
    // Write loop 2000 segments (claim payment information)
    new_edi.push_str(&write_loop2000(edi_json.table2s.clone()));
    
    // Write table 3 segments (summary)
    new_edi.push_str(&write_table3(edi_json.table3s.clone()));
    
    // Write interchange control trailer
    new_edi.push_str(&write_interchange_trailer(edi_json.interchange_trailer.clone()));
    
    new_edi
}
