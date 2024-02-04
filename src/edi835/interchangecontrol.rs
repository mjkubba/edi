use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::isa::*;
use crate::segments::gs::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa_segments: ISA,
    pub gs_segments: GS,
}


pub fn get_interchange_control(contents:String) -> (ISA, GS) {
    let mut isa_segments = ISA::default();
    let mut gs_segments = GS::default();
    if contents.contains("ISA") {
        info!("ISA segment found, ");
        isa_segments = get_isa(get_segment_contents("ISA", &contents));
        info!("ISA segment parsed");
    }
        if contents.contains("GS") {
        info!("GS segment found, ");
        gs_segments = get_gs(get_segment_contents("GS", &contents));
        info!("GS segment parsed");
    }
    
    info!("Interchange Control parsed\n");
    return (isa_segments, gs_segments)
}

pub fn get_interchange_header(mut contents:String) -> (InterchangeHeader, String) {
    let loop_content = get_loop_content(contents.clone(), "ISA", "ST");
    let (isa_segments, gs_segments) = get_interchange_control(loop_content.clone());
    contents = contents.replace(&loop_content, "");
    let header = InterchangeHeader {
        isa_segments,
        gs_segments,
    };
    return (header, contents)
}

pub fn write_interchange_control(header:InterchangeHeader) -> String {
    let mut contents = String::new();
    contents.push_str(&write_isa(header.isa_segments));
    contents.push_str(&write_gs(header.gs_segments));
    return contents
}

// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interchange_control() {
        let contents = String::from("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~");
        let (isa_segments, gs_segments, contents) = get_interchange_control(contents);
        assert_eq!(isa_segments.sender_id, "SUBMITTERS ID  ");
        assert_eq!(isa_segments.receiver_id, "RECEIVERS ID   ");
        assert_eq!(gs_segments.app_sender_id, "SENDER CODE");
        assert_eq!(gs_segments.app_receiver_id, "RECEIVER CODE");
        assert_eq!(contents, "");
    }
    #[test]
    fn test_get_interchange_header() {
        let contents = String::from("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~");
        let (header, contents) = get_interchange_header(contents);
        assert_eq!(header.isa_segments.sender_id, "SUBMITTERS ID  ");
        assert_eq!(header.isa_segments.receiver_id, "RECEIVERS ID   ");
        assert_eq!(header.gs_segments.app_sender_id, "SENDER CODE");
        assert_eq!(header.gs_segments.app_receiver_id, "RECEIVER CODE");
        assert_eq!(contents, "");
    }
}