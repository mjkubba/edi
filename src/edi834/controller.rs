use log::info;
use serde::{Deserialize, Serialize};

use crate::edi834::interchangecontrol::*;
use crate::edi834::loop1000a::*;
use crate::edi834::loop1000b::*;
use crate::edi834::loop2000::*;
use crate::edi834::table1::*;
use crate::error::{EdiError, EdiResult};
use crate::helper::edihelper::*;
use crate::segments::se::*;
use crate::transaction_processor::TransactionSet;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi834 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop1000a: Option<Loop1000A>,     // Sponsor
    pub loop1000b: Option<Loop1000B>,     // Payer
    pub loop2000_segments: Vec<Loop2000>, // Member Level
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

impl TransactionSet for Edi834 {
    fn parse(contents: String) -> EdiResult<(Self, String)>
    where
        Self: Sized,
    {
        get_834(contents)
    }

    fn to_edi(&self) -> String {
        write_834(self)
    }

    fn get_transaction_type() -> &'static str {
        "834"
    }

    fn detect(contents: &str) -> bool {
        contents.contains("ST*834*") || contents.contains("~ST*834*")
    }
}

pub fn get_834(mut contents: String) -> EdiResult<(Edi834, String)> {
    let mut edi834 = Edi834::default();

    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();

    // Parse Interchange Header
    let (interchange_header, new_contents) = get_interchange_header(contents.clone());
    edi834.interchange_header = interchange_header;
    contents = new_contents;

    // Parse Table 1
    match get_table1(contents.clone()) {
        Ok((table1, new_contents)) => {
            edi834.table1 = table1;
            contents = new_contents;
        }
        Err(e) => return Err(e),
    }

    // Parse Loop1000A (Sponsor)
    if contents.contains("N1*P5*") || contents.contains("N1*IN*") {
        let (loop1000a, new_contents) = get_loop1000a(contents);
        edi834.loop1000a = Some(loop1000a);
        contents = new_contents;
    }

    // Parse Loop1000B (Payer)
    if contents.contains("N1*IN*") && edi834.loop1000a.is_some() {
        let (loop1000b, new_contents) = get_loop1000b(contents);
        edi834.loop1000b = Some(loop1000b);
        contents = new_contents;
    }

    // Parse Loop2000 segments (Member Level)
    while contents.contains("INS*") {
        let (loop2000, new_contents) = get_loop2000(contents);
        edi834.loop2000_segments.push(loop2000);
        contents = new_contents;
    }

    // Parse SE segment
    if let Some(se_start) = contents.find("SE*") {
        if let Some(se_end) = contents[se_start..].find("~") {
            let se_content = &contents[se_start + 3..se_start + se_end];
            edi834.se_segments = get_se(se_content.to_string());
            contents = contents[se_start + se_end + 1..].to_string();
        }
    }

    // Parse Interchange Trailer
    let (interchange_trailer, new_contents) = get_interchange_trailer(contents.clone());
    edi834.interchange_trailer = interchange_trailer;
    contents = new_contents;

    info!("Parsed EDI834: {:?}", edi834);
    Ok((edi834, contents))
}

pub fn write_834(edi834: &Edi834) -> String {
    let mut result = String::new();

    result.push_str(&write_interchange_header(edi834.interchange_header.clone()));
    result.push_str(&write_table1(edi834.table1.clone()));

    if let Some(loop1000a) = &edi834.loop1000a {
        result.push_str(&write_loop1000a(loop1000a.clone()));
    }

    if let Some(loop1000b) = &edi834.loop1000b {
        result.push_str(&write_loop1000b(loop1000b.clone()));
    }

    for loop2000 in &edi834.loop2000_segments {
        result.push_str(&write_loop2000(loop2000.clone()));
    }

    result.push_str(&write_se(edi834.se_segments.clone()));
    result.push_str("\n");
    result.push_str(&write_interchange_trailer(
        edi834.interchange_trailer.clone(),
    ));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_834() {
        let contents = "ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*BE*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X220A1~ST*834*35681~";
        assert!(Edi834::detect(contents));

        let contents_no_834 = "ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~";
        assert!(!Edi834::detect(contents_no_834));
    }

    #[test]
    fn test_get_transaction_type() {
        assert_eq!(Edi834::get_transaction_type(), "834");
    }
}
