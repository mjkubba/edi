use log::info;

use serde::{Serialize, Deserialize};

use crate::edi999::interchangecontrol::*;
use crate::edi999::table1::*;
// use crate::edi835::loop1000a::*;
// use crate::edi835::loop1000b::*;
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
    // pub table1s: Table1s,
    // ub loop2000s: Vec<Loop2000>,
    pub table1_combined: Table1Combined,
    pub interchange_trailer: InterchangeTrailer,
    // pub table1trailer: Table1trailer,
}

pub fn get_999(mut contents: String) -> Edi999 {
    let interchange_header;
    let table1s;
    let loop2000s;
    let table1trailer;
    let interchange_trailer;
    let table1_combined;
    
    contents = contents.trim_start_matches("\u{feff}").to_string();

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
    edi999
}


pub fn write_999(contents: String) -> String {
    let edi_json: Edi999 = serde_json::from_str(&contents.clone()).unwrap();
    let mut new_edi = String::new();
    let new_ich = write_interchange_control(edi_json.interchange_header.clone());
    let new_table1s = write_table1(edi_json.table1_combined.table1.clone());
    let new_loop2000s = write_loop2000(edi_json.table1_combined.loop2000s.clone());
    let new_table1trailer = write_table1trailer(edi_json.table1_combined.table1trailer.clone());
    let new_ict = write_interchange_trailer(edi_json.interchange_trailer.clone());
    new_edi.push_str(&new_ich);
    new_edi.push_str(&new_table1s);
    new_edi.push_str(&new_loop2000s);
    new_edi.push_str(&new_table1trailer);
    new_edi.push_str(&new_ict);
    println!("{:?}", new_edi.clone());
    new_edi
}