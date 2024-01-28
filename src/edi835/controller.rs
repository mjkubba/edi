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
    interchange_header: InterchangeHeader,
    table1s: Table1s,
    loop1000as: Loop1000as,
    loop1000bs: Loop1000bs,
    loop2000s: Vec<Loop2000s>,
    loop2100s: Vec<Loop2100s>,
    loop2110s: Vec<Loop2110s>,
    table3s: Table3s,
    interchange_trailer: InterchangeTrailer,
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