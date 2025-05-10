use log::info;
use serde::{Serialize, Deserialize};

use crate::edi278::interchangecontrol::*;
use crate::edi278::table1::*;
use crate::edi278::loop2000a::*;
use crate::edi278::loop2010a::*;
use crate::edi278::loop2000b::*;
use crate::edi278::loop2010b::*;
use crate::edi278::loop2000c::*;
use crate::edi278::loop2010c::*;
use crate::edi278::loop2000d::*;
use crate::edi278::loop2010d::*;
use crate::edi278::loop2000e::*;
use crate::edi278::loop2100e::*;
use crate::edi278::loop2110e::*;
use crate::edi278::loop2000f::*;
use crate::edi278::loop2010f::*;
use crate::edi278::loop2100f::*;
use crate::edi278::interchangecontroltrailer::*;
use crate::error::EdiResult;
use crate::transaction_processor::TransactionSet;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1Combined {
    pub table1: Table1s,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi278 {
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub loop2000a: Option<Loop2000A>,
    pub loop2010a: Option<Loop2010A>,
    pub loop2000b: Option<Loop2000B>,
    pub loop2010b: Option<Loop2010B>,
    pub loop2000c: Option<Loop2000C>,
    pub loop2010c: Option<Loop2010C>,
    pub loop2000d: Option<Loop2000D>,
    pub loop2010d: Option<Loop2010D>,
    pub loop2000e: Option<Loop2000E>,
    pub loop2100e: Option<Loop2100E>,
    pub loop2110e: Option<Loop2110E>,
    pub loop2000f: Option<Loop2000F>,
    pub loop2010f: Option<Loop2010F>,
    pub loop2100f: Option<Loop2100F>,
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
        
        // Combine Table 1
        edi278.table1_combined = Table1Combined {
            table1: table1s,
        };
        
        // Parse Loop 2000A (UMO Level)
        let (loop2000a, new_contents) = get_loop2000a(contents.clone());
        if loop2000a.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000a = Some(loop2000a);
            contents = new_contents;
            
            // Parse Loop 2010A (UMO Name)
            let (loop2010a, new_contents) = get_loop2010a(contents.clone());
            if loop2010a.nm1_segments.entity_id != "" {
                edi278.loop2010a = Some(loop2010a);
                contents = new_contents;
            }
        }
        
        // Parse Loop 2000B (Requester Level)
        let (loop2000b, new_contents) = get_loop2000b(contents.clone());
        if loop2000b.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000b = Some(loop2000b);
            contents = new_contents;
            
            // Parse Loop 2010B (Requester Name)
            let (loop2010b, new_contents) = get_loop2010b(contents.clone());
            if loop2010b.nm1_segments.entity_id != "" {
                edi278.loop2010b = Some(loop2010b);
                contents = new_contents;
            }
        }
        
        // Parse Loop 2000C (Subscriber Level)
        let (loop2000c, new_contents) = get_loop2000c(contents.clone());
        if loop2000c.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000c = Some(loop2000c);
            contents = new_contents;
            
            // Parse Loop 2010C (Subscriber Name)
            let (loop2010c, new_contents) = get_loop2010c(contents.clone());
            if loop2010c.nm1_segments.entity_id != "" {
                edi278.loop2010c = Some(loop2010c);
                contents = new_contents;
            }
        }
        
        // Parse Loop 2000D (Dependent Level)
        let (loop2000d, new_contents) = get_loop2000d(contents.clone());
        if loop2000d.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000d = Some(loop2000d);
            contents = new_contents;
            
            // Parse Loop 2010D (Dependent Name)
            let (loop2010d, new_contents) = get_loop2010d(contents.clone());
            if loop2010d.nm1_segments.entity_id != "" {
                edi278.loop2010d = Some(loop2010d);
                contents = new_contents;
            }
        }
        
        // Parse Loop 2000E (Service Level)
        let (loop2000e, new_contents) = get_loop2000e(contents.clone());
        if loop2000e.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000e = Some(loop2000e);
            contents = new_contents;
            
            // Parse Loop 2100E (Service Level Detail)
            let (loop2100e, new_contents) = get_loop2100e(contents.clone());
            if !loop2100e.dtp_segments.is_empty() || loop2100e.hi_segments.is_some() || 
               loop2100e.hsd_segments.is_some() || loop2100e.cl1_segments.is_some() {
                edi278.loop2100e = Some(loop2100e);
                contents = new_contents;
            }
            
            // Parse Loop 2110E (Service Provider)
            let (loop2110e, new_contents) = get_loop2110e(contents.clone());
            if loop2110e.nm1_segments.entity_id != "" {
                edi278.loop2110e = Some(loop2110e);
                contents = new_contents;
            }
        }
        
        // Parse Loop 2000F (Service Provider Level)
        let (loop2000f, new_contents) = get_loop2000f(contents.clone());
        if loop2000f.hl_segments.hl01_hierarchical_id_number != "" {
            edi278.loop2000f = Some(loop2000f);
            contents = new_contents;
            
            // Parse Loop 2010F (Service Provider Name)
            let (loop2010f, new_contents) = get_loop2010f(contents.clone());
            if loop2010f.nm1_segments.entity_id != "" {
                edi278.loop2010f = Some(loop2010f);
                contents = new_contents;
            }
            
            // Parse Loop 2100F (Service Provider Detail)
            let (loop2100f, new_contents) = get_loop2100f(contents.clone());
            if !loop2100f.dtp_segments.is_empty() || loop2100f.sv2_segments.is_some() {
                edi278.loop2100f = Some(loop2100f);
                contents = new_contents;
            }
        }
        
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
        if let Some(loop2000a) = &self.loop2000a {
            new_edi.push_str(&write_loop2000a(loop2000a.clone()));
            
            // Write Loop 2010A (UMO Name)
            if let Some(loop2010a) = &self.loop2010a {
                new_edi.push_str(&write_loop2010a(loop2010a.clone()));
            }
        }
        
        // Write Loop 2000B (Requester Level)
        if let Some(loop2000b) = &self.loop2000b {
            new_edi.push_str(&write_loop2000b(loop2000b.clone()));
            
            // Write Loop 2010B (Requester Name)
            if let Some(loop2010b) = &self.loop2010b {
                new_edi.push_str(&write_loop2010b(loop2010b.clone()));
            }
        }
        
        // Write Loop 2000C (Subscriber Level)
        if let Some(loop2000c) = &self.loop2000c {
            new_edi.push_str(&write_loop2000c(loop2000c.clone()));
            
            // Write Loop 2010C (Subscriber Name)
            if let Some(loop2010c) = &self.loop2010c {
                new_edi.push_str(&write_loop2010c(loop2010c.clone()));
            }
        }
        
        // Write Loop 2000D (Dependent Level)
        if let Some(loop2000d) = &self.loop2000d {
            new_edi.push_str(&write_loop2000d(loop2000d.clone()));
            
            // Write Loop 2010D (Dependent Name)
            if let Some(loop2010d) = &self.loop2010d {
                new_edi.push_str(&write_loop2010d(loop2010d.clone()));
            }
        }
        
        // Write Loop 2000E (Service Level)
        if let Some(loop2000e) = &self.loop2000e {
            new_edi.push_str(&write_loop2000e(loop2000e.clone()));
            
            // Write Loop 2100E (Service Level Detail)
            if let Some(loop2100e) = &self.loop2100e {
                new_edi.push_str(&write_loop2100e(loop2100e.clone()));
            }
            
            // Write Loop 2110E (Service Provider)
            if let Some(loop2110e) = &self.loop2110e {
                new_edi.push_str(&write_loop2110e(loop2110e.clone()));
            }
        }
        
        // Write Loop 2000F (Service Provider Level)
        if let Some(loop2000f) = &self.loop2000f {
            new_edi.push_str(&write_loop2000f(loop2000f.clone()));
            
            // Write Loop 2010F (Service Provider Name)
            if let Some(loop2010f) = &self.loop2010f {
                new_edi.push_str(&write_loop2010f(loop2010f.clone()));
            }
            
            // Write Loop 2100F (Service Provider Detail)
            if let Some(loop2100f) = &self.loop2100f {
                new_edi.push_str(&write_loop2100f(loop2100f.clone()));
            }
        }
        
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
    
    // Check for other 278-specific indicators
    if contents.contains("UM*") && (contents.contains("HL*") && contents.contains("*20*")) {
        return true;
    }
    
    false
}

// Function to detect if JSON contains 278 format data
pub fn is_278_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 278 format
    contents.contains("\"transaction_set_id\":\"278\"") || 
    contents.contains("\"um01_request_category_code\":") ||
    contents.contains("\"hl03_hierarchical_level_code\":\"20\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~ST*278*0001*005010X217~BHT*0007*11*123456*20060501*1319~";
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
BHT*0007*11*123456*20060501*1319~
HL*1**20*1~
NM1*X3*2*UMO NAME*****PI*12345~
PER*IC*CONTACT NAME*TE*5551234567~
HL*2*1*21*1~
NM1*1P*2*BONE AND JOINT CLINIC*****SV*2000035~
REF*XZ*7654321~
SE*10*0001~
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
        
        // Check Loop 2000A
        assert!(edi278.loop2000a.is_some());
        let loop2000a = edi278.loop2000a.unwrap();
        assert_eq!(loop2000a.hl_segments.hl03_hierarchical_level_code, "20");
        
        // Check Loop 2010A
        assert!(edi278.loop2010a.is_some());
        let loop2010a = edi278.loop2010a.unwrap();
        assert_eq!(loop2010a.nm1_segments.lastname, "UMO NAME");
        
        // Check Loop 2000B
        assert!(edi278.loop2000b.is_some());
        let loop2000b = edi278.loop2000b.unwrap();
        assert_eq!(loop2000b.hl_segments.hl03_hierarchical_level_code, "21");
        
        // Check Loop 2010B
        assert!(edi278.loop2010b.is_some());
        let loop2010b = edi278.loop2010b.unwrap();
        assert_eq!(loop2010b.nm1_segments.lastname, "BONE AND JOINT CLINIC");
        assert_eq!(loop2010b.ref_segments.len(), 1);
        assert_eq!(loop2010b.ref_segments[0].reference_id_number, "7654321");
        
        assert_eq!(edi278.interchange_trailer.se_segments.number_of_segment, "10");
    }
    
    #[test]
    fn test_parse_complex_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~
GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~
ST*278*0001*005010X217~
BHT*0007*11*123456*20060501*1319~
HL*1**20*1~
NM1*X3*2*UMO NAME*****PI*12345~
PER*IC*CONTACT NAME*TE*5551234567~
HL*2*1*21*1~
NM1*1P*2*BONE AND JOINT CLINIC*****SV*2000035~
REF*XZ*7654321~
HL*3*2*22*1~
TRN*1*12345*1512345678~
NM1*IL*1*DOE*JOHN****MI*123456789A~
REF*SY*123456789~
DMG*D8*19800519*M~
HL*4*3*23*0~
TRN*1*67890*1512345678~
NM1*QC*1*DOE*JANE****MI*123456789B~
REF*SY*987654321~
DMG*D8*20100519*F~
HL*5*4*SS*0~
TRN*1*12345*1512345678~
UM*HS*I*1*2*3*4*5*6*7*Y~
DTP*435*D8*20050516~
HI*BF:41090:D8:20050125~
HSD*DY*7~
CL1*2~
NM1*71*1*SMITH*JOHN*A***XX*1234567890~
REF*1J*12345~
PRV*PE*ZZ*207Q00000X~
HL*6*5*PT*0~
NM1*1P*2*PROVIDER GROUP*****XX*1234567890~
REF*TJ*123456789~
PRV*PE*ZZ*207Q00000X~
DTP*472*D8*20050516~
SV2**HC:33510~
SE*33*0001~
GE*1*1~
IEA*1*000000001~";

        let result = Edi278::parse(content.to_string());
        assert!(result.is_ok());
        
        let (edi278, _) = result.unwrap();
        
        // Check Loop 2000C
        assert!(edi278.loop2000c.is_some());
        let loop2000c = edi278.loop2000c.unwrap();
        assert_eq!(loop2000c.hl_segments.hl03_hierarchical_level_code, "22");
        assert_eq!(loop2000c.trn_segments.len(), 1);
        assert_eq!(loop2000c.trn_segments[0].reference_id, "12345");
        
        // Check Loop 2010C
        assert!(edi278.loop2010c.is_some());
        let loop2010c = edi278.loop2010c.unwrap();
        assert_eq!(loop2010c.nm1_segments.lastname, "DOE");
        assert_eq!(loop2010c.nm1_segments.firstname, "JOHN");
        assert!(loop2010c.dmg_segments.is_some());
        assert_eq!(loop2010c.dmg_segments.as_ref().unwrap().gender_code, "M");
        
        // Check Loop 2000D
        assert!(edi278.loop2000d.is_some());
        let loop2000d = edi278.loop2000d.unwrap();
        assert_eq!(loop2000d.hl_segments.hl03_hierarchical_level_code, "23");
        
        // Check Loop 2010D
        assert!(edi278.loop2010d.is_some());
        let loop2010d = edi278.loop2010d.unwrap();
        assert_eq!(loop2010d.nm1_segments.lastname, "DOE");
        assert_eq!(loop2010d.nm1_segments.firstname, "JANE");
        assert!(loop2010d.dmg_segments.is_some());
        assert_eq!(loop2010d.dmg_segments.as_ref().unwrap().gender_code, "F");
        
        // Check Loop 2000E
        assert!(edi278.loop2000e.is_some());
        let loop2000e = edi278.loop2000e.unwrap();
        assert_eq!(loop2000e.hl_segments.hl03_hierarchical_level_code, "SS");
        assert!(loop2000e.um_segments.is_some());
        assert_eq!(loop2000e.um_segments.as_ref().unwrap().um01_request_category_code, "HS");
        
        // Check Loop 2100E
        assert!(edi278.loop2100e.is_some());
        let loop2100e = edi278.loop2100e.unwrap();
        assert_eq!(loop2100e.dtp_segments.len(), 1);
        assert_eq!(loop2100e.dtp_segments[0].dtp01_date_time_qualifier, "435");
        assert!(loop2100e.hi_segments.is_some());
        assert!(loop2100e.hsd_segments.is_some());
        assert!(loop2100e.cl1_segments.is_some());
        
        // Check Loop 2110E
        assert!(edi278.loop2110e.is_some());
        let loop2110e = edi278.loop2110e.unwrap();
        assert_eq!(loop2110e.nm1_segments.entity_id, "71");
        assert_eq!(loop2110e.nm1_segments.lastname, "SMITH");
        assert_eq!(loop2110e.nm1_segments.firstname, "JOHN");
        assert!(loop2110e.prv_segments.is_some());
        assert_eq!(loop2110e.prv_segments.as_ref().unwrap().provider_code, "PE");
        
        // Check Loop 2000F
        assert!(edi278.loop2000f.is_some());
        let loop2000f = edi278.loop2000f.unwrap();
        assert_eq!(loop2000f.hl_segments.hl03_hierarchical_level_code, "PT");
        
        // Check Loop 2010F
        assert!(edi278.loop2010f.is_some());
        let loop2010f = edi278.loop2010f.unwrap();
        assert_eq!(loop2010f.nm1_segments.entity_id, "1P");
        assert_eq!(loop2010f.nm1_segments.lastname, "PROVIDER GROUP");
        assert!(loop2010f.prv_segments.is_some());
        assert_eq!(loop2010f.prv_segments.as_ref().unwrap().provider_code, "PE");
        
        // Check Loop 2100F
        assert!(edi278.loop2100f.is_some());
        let loop2100f = edi278.loop2100f.unwrap();
        assert_eq!(loop2100f.dtp_segments.len(), 1);
        assert_eq!(loop2100f.dtp_segments[0].dtp01_date_time_qualifier, "472");
        assert!(loop2100f.sv2_segments.is_some());
        assert_eq!(loop2100f.sv2_segments.as_ref().unwrap().sv202_procedure_code, "HC:33510");
    }
    
    #[test]
    fn test_roundtrip_278() {
        let content = "ISA*00*          *00*          *ZZ*SUBMITTER      *ZZ*RECEIVER       *200101*1200*^*00501*000000001*0*T*:~
GS*HI*SUBMITTER*RECEIVER*20200101*1200*1*X*005010X217~
ST*278*0001*005010X217~
BHT*0007*11*123456*20060501*1319~
HL*1**20*1~
NM1*X3*2*UMO NAME*****PI*12345~
PER*IC*CONTACT NAME*TE*5551234567~
HL*2*1*21*1~
NM1*1P*2*BONE AND JOINT CLINIC*****SV*2000035~
REF*XZ*7654321~
SE*10*0001~
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
        assert!(generated.contains("BHT*0007*11*123456*20060501*1319"));
        assert!(generated.contains("HL*1**20*1"));
        assert!(generated.contains("NM1*X3*2*UMO NAME"));
        assert!(generated.contains("PER*IC*CONTACT NAME*TE*5551234567"));
        assert!(generated.contains("HL*2*1*21*1"));
        assert!(generated.contains("NM1*1P*2*BONE AND JOINT CLINIC"));
        assert!(generated.contains("REF*XZ*7654321"));
        assert!(generated.contains("SE*10*0001"));
        assert!(generated.contains("GE*1*1"));
        assert!(generated.contains("IEA*1*000000001"));
    }
}
