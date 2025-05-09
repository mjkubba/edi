use log::info;
use serde::{Serialize, Deserialize};

use crate::edi278::interchangecontrol::*;
use crate::edi278::table1::*;
use crate::edi278::loop2000a::*;
use crate::edi278::loop2010a::*;
use crate::edi278::interchangecontroltrailer::*;
use crate::error::EdiResult;
use crate::transaction_processor::TransactionSet;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1Combined {
    pub table1: Table1s,
    pub loop2000a: Loop2000A,
    pub loop2010a: Loop2010A,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi278 {
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub interchange_trailer: InterchangeTrailer,
    pub transaction_set_id: String,
}

impl TransactionSet for Edi278 {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        let mut edi278 = Edi278::default();
        edi278.transaction_set_id = "278".to_string();
        
        // Remove BOM if present
        let mut contents = contents.trim_start_matches("\u{feff}").to_string();
        
        // Remove carriage returns and line feeds
        contents = contents.replace("\r", "").replace("\n", "");
        
        // Parse interchange control header
        let (interchange_header, new_contents) = get_interchange_header(contents);
        edi278.interchange_header = interchange_header;
        contents = new_contents;
        
        // Parse table 1
        let (table1s, new_contents) = get_table1s(contents);
        contents = new_contents;
        
        // Parse Loop 2000A (UMO Level)
        let (loop2000a, new_contents) = get_loop2000a(contents);
        contents = new_contents;
        
        // Parse Loop 2010A (UMO Name)
        let (loop2010a, new_contents) = get_loop2010a(contents);
        contents = new_contents;
        
        // Combine Table 1 and Loop 2000A/2010A
        edi278.table1_combined = Table1Combined {
            table1: table1s,
            loop2000a,
            loop2010a,
        };
        
        // Parse interchange control trailer
        let (interchange_trailer, remaining) = get_interchange_trailer(contents);
        edi278.interchange_trailer = interchange_trailer;
        
        Ok((edi278, remaining))
    }
    
    fn to_edi(&self) -> String {
        let mut new_edi = String::new();
        
        // Write interchange header
        new_edi.push_str(&write_interchange_control(self.interchange_header.clone()));
        
        // Write Table 1
        new_edi.push_str(&write_table1(self.table1_combined.table1.clone()));
        
        // Write Loop 2000A (UMO Level)
        new_edi.push_str(&write_loop2000a(self.table1_combined.loop2000a.clone()));
        
        // Write Loop 2010A (UMO Name)
        new_edi.push_str(&write_loop2010a(self.table1_combined.loop2010a.clone()));
        
        // Write interchange trailer
        new_edi.push_str(&write_interchange_trailer(self.interchange_trailer.clone()));
        
        // Add line breaks between segments for better readability
        let new_edi_with_breaks = new_edi.replace("~", "~\n");
        
        info!("Generated EDI 278: {}", new_edi_with_breaks);
        new_edi_with_breaks
    }
    
    fn get_transaction_type() -> &'static str {
        "278"
    }
    
    fn detect(contents: &str) -> bool {
        is_278(contents)
    }
}

pub fn get_278(contents: String) -> EdiResult<Edi278> {
    match Edi278::parse(contents) {
        Ok((edi278, _)) => Ok(edi278),
        Err(e) => Err(e),
    }
}

pub fn write_278(edi278: &Edi278) -> String {
    edi278.to_edi()
}

// Function to detect if content is EDI 278 format
pub fn is_278(contents: &str) -> bool {
    // Check if the content contains ST*278 which indicates an EDI 278 transaction set
    if contents.contains("ST*278") {
        return true;
    }
    
    // Additional checks could be added here to improve detection accuracy
    
    false
}

// Function to detect if JSON contains 278 format data
pub fn is_278_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 278 format
    contents.contains("\"transaction_set_id\":\"278\"") || 
    contents.contains("\"bht01_hierarchical_structure_code\":\"0007\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~ST*278*0001*005010X217~BHT*0007*11*123456*20200101*1200~";
        assert!(is_278(content));
        
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X221~ST*835*0001*005010X221~";
        assert!(!is_278(content));
    }
    
    #[test]
    fn test_is_278_json() {
        let content = r#"{"transaction_set_id":"278","interchange_control":{"segment_id":"ISA"}}"#;
        assert!(is_278_json(content));
        
        let content = r#"{"transaction_set_id":"835","interchange_control":{"segment_id":"ISA"}}"#;
        assert!(!is_278_json(content));
    }
    
    #[test]
    fn test_parse_basic_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~
GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~
ST*278*0001*005010X217~
BHT*0007*11*123456*20200101*1200~
HL*1**20*1~
NM1*X3*2*UMO NAME*****PI*12345~
PER*IC*CONTACT NAME*TE*5551234567~
SE*7*0001~
GE*1*1~
IEA*1*000000001~";

        let result = Edi278::parse(content.to_string());
        assert!(result.is_ok());
        
        let (edi278, _) = result.unwrap();
        assert_eq!(edi278.transaction_set_id, "278");
        assert_eq!(edi278.interchange_header.isa_segments.sender_id, "SUBMITTER      ");
        assert_eq!(edi278.interchange_header.gs_segments.app_sender_id, "SUBMITTER");
        assert_eq!(edi278.table1_combined.table1.st_segments.transaction_set_id, "278");
        assert_eq!(edi278.table1_combined.table1.bht_segments.bht03_reference_identification, "123456");
        assert_eq!(edi278.table1_combined.loop2000a.hl_segments.hl03_hierarchical_level_code, "20");
        assert_eq!(edi278.table1_combined.loop2010a.nm1_segments.lastname, "UMO NAME");
        assert_eq!(edi278.interchange_trailer.se_segments.number_of_segment, "7");
    }
    
    #[test]
    fn test_roundtrip_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~
GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~
ST*278*0001*005010X217~
BHT*0007*11*123456*20200101*1200~
HL*1**20*1~
NM1*X3*2*UMO NAME*****PI*12345~
PER*IC*CONTACT NAME*TE*5551234567~
SE*7*0001~
GE*1*1~
IEA*1*000000001~";

        // Parse the content
        let result = get_278(content.to_string());
        assert!(result.is_ok());
        
        let edi278 = result.unwrap();
        
        // Generate EDI from the parsed object
        let generated = write_278(&edi278);
        
        // Check that the generated content contains the expected segments
        assert!(generated.contains("ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200"));
        assert!(generated.contains("GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217"));
        assert!(generated.contains("ST*278*0001*005010X217"));
        assert!(generated.contains("BHT*0007*11*123456*20200101*1200"));
        assert!(generated.contains("HL*1**20*1"));
        assert!(generated.contains("NM1*X3*2*UMO NAME"));
        assert!(generated.contains("PER*IC*CONTACT NAME*TE*5551234567"));
        assert!(generated.contains("SE*7*0001"));
        assert!(generated.contains("GE*1*1"));
        assert!(generated.contains("IEA*1*000000001"));
    }
}
