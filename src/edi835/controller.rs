use log::info;

use serde::{Serialize, Deserialize};

use crate::edi835::interchangecontrol::*;
use crate::edi835::table1::*;
use crate::edi835::loop1000a::*;
use crate::edi835::loop1000b::*;
use crate::edi835::loop2000::*;
use crate::edi835::loop2100::*;
use crate::edi835::loop2110::*;
use crate::edi835::table3::*;
use crate::edi835::interchangecontroltrailer::*;


#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Edi835{
    pub interchange_header: InterchangeHeader,
    pub table1s: Table1s,
    pub loop1000as: Loop1000as,
    pub loop1000bs: Loop1000bs,
    pub loop2000s: Vec<Loop2000s>,
    pub loop2100s: Vec<Loop2100s>,
    pub loop2110s: Vec<Loop2110s>,
    pub table3s: Table3s,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_835(mut contents: String) -> Edi835 {
    let interchange_header;
    let table1s;
    let loop1000as;
    let loop1000bs;
    let loop2000s;
    let loop2100s;
    let loop2110s;
    let table3s;
    let interchange_trailer;

    // Control Segments
    (interchange_header, contents) = get_interchange_header(contents.clone());

    // Table 1
    (table1s, contents) = get_table1s(contents.clone());
    
    // Loop 1000A Payer Identification
    (loop1000as, contents) = get_1000as(contents.clone());
    
    // Loop 1000B Payee Identification
    (loop1000bs, contents) = get_1000bs(contents.clone());

    // loop 2000
    (loop2000s, contents) = get_loop_2000s(contents.clone());

    // loop 2100
    (loop2100s, contents) = get_loop_2100s(contents.clone());

    // loop 2110
    (loop2110s, contents) = get_loop_2110s(contents.clone());

    // Table 3
    (table3s, contents) = get_table3s(contents.clone());

    // Control Trailer
    (interchange_trailer, contents) = get_interchange_trailer(contents.clone());

    let edi835 = Edi835 {
        interchange_header,
        table1s,
        loop1000as,
        loop1000bs,
        loop2000s,
        loop2100s,
        loop2110s,
        table3s,
        interchange_trailer,
    };
    

    info!("Unprocessed segments: {:?}", contents);
    edi835
}


pub fn write_edi(contents: String) -> String {
    let edi_json: Edi835 = serde_json::from_str(&contents.clone()).unwrap();
    let mut new_edi = String::new();
    let new_ich = write_interchange_control(edi_json.interchange_header.clone());
    let new_t1 = write_table1(edi_json.table1s.clone());
    let new_l1a = write_loop1000a(edi_json.loop1000as.clone());
    let new_l1b = write_loop1000b(edi_json.loop1000bs.clone());
    let new_l2 = write_loop2000(edi_json.loop2000s.clone());
    let new_l21 = write_loop2100(edi_json.loop2100s.clone());
    let new_l211 = write_loop2110(edi_json.loop2110s.clone());
    let new_t3 = write_table3(edi_json.table3s.clone());
    let new_ict = write_interchange_trailer(edi_json.interchange_trailer.clone());
    new_edi.push_str(&new_ich);
    new_edi.push_str(&new_t1);
    new_edi.push_str(&new_l1a);
    new_edi.push_str(&new_l1b);
    new_edi.push_str(&new_l2);
    new_edi.push_str(&new_l21);
    new_edi.push_str(&new_l211);
    new_edi.push_str(&new_t3);
    new_edi.push_str(&new_ict);
    println!("{:?}", new_edi.clone());
    new_edi
}