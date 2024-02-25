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
pub struct Edi999{
    pub interchange_header: InterchangeHeader,
    pub table1s: Table1s,
    pub loop2000s: Vec<Loop2000>,
    pub interchange_trailer: InterchangeTrailer,
    pub table1trailer: Table1trailer,
}

pub fn get_999(mut contents: String) -> Edi999 {
    let interchange_header;
    let table1s;
    let loop2000s;
    let table1trailer;
    let interchange_trailer;

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


    let edi999 = Edi999 {
        interchange_header,
        interchange_trailer,
        table1s,
        loop2000s,
        table1trailer,
    };
    

    info!("Unprocessed segments: {:?}", contents);
    edi999
}


pub fn write_999(contents: String) -> String {
    let edi_json: Edi999 = serde_json::from_str(&contents.clone()).unwrap();
    let mut new_edi = String::new();
    let new_ich = write_interchange_control(edi_json.interchange_header.clone());
    let new_t1 = write_table1(edi_json.table1s.clone());
    // let new_l1a = write_loop1000a(edi_json.loop1000as.clone());
    // let new_l1b = write_loop1000b(edi_json.loop1000bs.clone());
    let new_l2 = write_loop2000(edi_json.loop2000s.clone());
    let new_t1t = write_table1trailer(edi_json.table1trailer.clone());
    // let new_t3 = write_table3(edi_json.table3s.clone());
    let new_ict = write_interchange_trailer(edi_json.interchange_trailer.clone());
    new_edi.push_str(&new_ich);
    new_edi.push_str(&new_t1);
    // new_edi.push_str(&new_l1a);
    // new_edi.push_str(&new_l1b);
    new_edi.push_str(&new_l2);
    new_edi.push_str(&new_t1t);
    new_edi.push_str(&new_ict);
    println!("{:?}", new_edi.clone());
    new_edi
}