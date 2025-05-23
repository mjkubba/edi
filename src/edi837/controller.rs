use log::info;
use serde::{Serialize, Deserialize};
use crate::error::{EdiResult, EdiError};
use crate::transaction_processor::TransactionSet;

use crate::edi837::interchangecontrol::InterchangeHeader;
use crate::edi837::table1::Table1s;
use crate::edi837::loop2000a::{Loop2000a, parse_loop2000a, write_loop2000a};
use crate::edi837::loop2000b::{Loop2000b, parse_loop2000b, write_loop2000b};
use crate::edi837::loop2000c::{Loop2000c, parse_loop2000c, write_loop2000c};
use crate::edi837::loop2010aa::{Loop2010aa, parse_loop2010aa, write_loop2010aa};
use crate::edi837::loop2010ab::{Loop2010ab, parse_loop2010ab, write_loop2010ab};
use crate::edi837::loop2010ac::{Loop2010ac, parse_loop2010ac, write_loop2010ac};
use crate::edi837::loop2300::{Loop2300, parse_loop2300, write_loop2300};
use crate::edi837::loop2400::{Loop2400, parse_loop2400, write_loop2400};
use crate::edi837::interchangecontroltrailer::InterchangeTrailer;

/// Table1 structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1 {
    pub table1: Table1s,
    pub loop2000a: Loop2000a,
}

/// EDI837 Professional structure
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837P {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000b: Vec<Loop2000b>,
    pub loop2000c: Vec<Loop2000c>,
    pub loop2010aa: Loop2010aa,
    pub loop2010ab: Option<Loop2010ab>,
    pub loop2010ac: Option<Loop2010ac>,
    pub loop2300: Vec<Loop2300>,
    pub loop2400: Vec<Loop2400>,
    pub interchange_trailer: InterchangeTrailer,
    // Raw segments for segments we don't parse yet
    pub isa: String,
    pub gs: String,
    pub st: String,
    pub se: String,
    pub ge: String,
    pub iea: String,
}

/// EDI837 Institutional structure
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837I {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000b: Vec<Loop2000b>,
    pub loop2000c: Vec<Loop2000c>,
    pub loop2010aa: Loop2010aa,
    pub loop2010ab: Option<Loop2010ab>,
    pub loop2010ac: Option<Loop2010ac>,
    pub loop2300: Vec<Loop2300>,
    pub loop2400: Vec<Loop2400>,
    pub interchange_trailer: InterchangeTrailer,
    // Raw segments for segments we don't parse yet
    pub isa: String,
    pub gs: String,
    pub st: String,
    pub se: String,
    pub ge: String,
    pub iea: String,
}

/// EDI837 Dental structure
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837D {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000b: Vec<Loop2000b>,
    pub loop2000c: Vec<Loop2000c>,
    pub loop2010aa: Loop2010aa,
    pub loop2010ab: Option<Loop2010ab>,
    pub loop2010ac: Option<Loop2010ac>,
    pub loop2300: Vec<Loop2300>,
    pub loop2400: Vec<Loop2400>,
    pub interchange_trailer: InterchangeTrailer,
    // Raw segments for segments we don't parse yet
    pub isa: String,
    pub gs: String,
    pub st: String,
    pub se: String,
    pub ge: String,
    pub iea: String,
}

impl TransactionSet for Edi837P {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        info!("Parsing EDI837P content");
        
        let mut edi837p = Edi837P::default();
        let mut remaining_content = contents.clone();
        
        // Parse interchange header
        if let Some(isa_pos) = remaining_content.find("ISA*") {
            let isa_end = remaining_content[isa_pos..].find('~').unwrap_or(remaining_content.len()) + isa_pos;
            edi837p.isa = remaining_content[isa_pos..=isa_end].to_string();
            remaining_content = remaining_content[isa_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ISA segment not found".to_string()));
        }
        
        // Parse GS segment
        if let Some(gs_pos) = remaining_content.find("GS*") {
            let gs_end = remaining_content[gs_pos..].find('~').unwrap_or(remaining_content.len()) + gs_pos;
            edi837p.gs = remaining_content[gs_pos..=gs_end].to_string();
            remaining_content = remaining_content[gs_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("GS segment not found".to_string()));
        }
        
        // Parse ST segment
        if let Some(st_pos) = remaining_content.find("ST*") {
            let st_end = remaining_content[st_pos..].find('~').unwrap_or(remaining_content.len()) + st_pos;
            edi837p.st = remaining_content[st_pos..=st_end].to_string();
            remaining_content = remaining_content[st_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ST segment not found".to_string()));
        }
        
        // Parse BHT segment
        if let Some(bht_pos) = remaining_content.find("BHT*") {
            let bht_end = remaining_content[bht_pos..].find('~').unwrap_or(remaining_content.len()) + bht_pos;
            edi837p.table1.table1.bht = remaining_content[bht_pos..=bht_end].to_string();
            remaining_content = remaining_content[bht_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("BHT segment not found".to_string()));
        }
        
        // Parse Loop2000A (Billing Provider Hierarchical Level)
        let (loop2000a, remaining) = parse_loop2000a(&remaining_content);
        edi837p.table1.loop2000a = loop2000a;
        remaining_content = remaining;
        
        // Parse Loop2010AA (Billing Provider Name)
        let (loop2010aa, remaining) = parse_loop2010aa(&remaining_content);
        edi837p.loop2010aa = loop2010aa;
        remaining_content = remaining;
        
        // Parse Loop2010AB (Pay-to Address) if present
        if remaining_content.contains("NM1*87*") {
            let (loop2010ab, remaining) = parse_loop2010ab(&remaining_content);
            edi837p.loop2010ab = Some(loop2010ab);
            remaining_content = remaining;
        }
        
        // Parse Loop2010AC (Pay-to Plan Name) if present
        if remaining_content.contains("NM1*PE*") {
            let (loop2010ac, remaining) = parse_loop2010ac(&remaining_content);
            edi837p.loop2010ac = Some(loop2010ac);
            remaining_content = remaining;
        }
        
        // Parse Loop2000B (Subscriber Hierarchical Level)
        let mut loop2000b_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*22*") {
            let (loop2000b, remaining) = parse_loop2000b(&remaining_content);
            if loop2000b.hl.is_empty() {
                break;
            }
            loop2000b_vec.push(loop2000b);
            remaining_content = remaining;
        }
        edi837p.loop2000b = loop2000b_vec;
        
        // Parse Loop2000C (Patient Hierarchical Level)
        let mut loop2000c_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*23*") {
            let (loop2000c, remaining) = parse_loop2000c(&remaining_content);
            if loop2000c.hl.is_empty() {
                break;
            }
            loop2000c_vec.push(loop2000c);
            remaining_content = remaining;
        }
        edi837p.loop2000c = loop2000c_vec;
        
        // Parse Loop2300 (Claim Information)
        let mut loop2300_vec = Vec::new();
        while remaining_content.contains("CLM*") {
            let (loop2300, remaining) = parse_loop2300(&remaining_content);
            if loop2300.clm.is_empty() {
                break;
            }
            loop2300_vec.push(loop2300);
            remaining_content = remaining;
            
            // Parse Loop2400 (Service Line Information) for this claim
            let mut loop2400_vec = Vec::new();
            while remaining_content.contains("LX*") {
                let (loop2400, remaining) = parse_loop2400(&remaining_content);
                if loop2400.lx.is_empty() {
                    break;
                }
                loop2400_vec.push(loop2400);
                remaining_content = remaining;
            }
            
            // Add service lines to the claim
            if !loop2400_vec.is_empty() {
                let last_index = loop2300_vec.len() - 1;
                loop2300_vec[last_index].loop2400 = loop2400_vec;
            }
        }
        edi837p.loop2300 = loop2300_vec;
        
        // Parse interchange trailer
        if let Some(se_pos) = remaining_content.find("SE*") {
            let se_end = remaining_content[se_pos..].find('~').unwrap_or(remaining_content.len()) + se_pos;
            edi837p.se = remaining_content[se_pos..=se_end].to_string();
            remaining_content = remaining_content[se_end + 1..].to_string();
        }
        
        if let Some(ge_pos) = remaining_content.find("GE*") {
            let ge_end = remaining_content[ge_pos..].find('~').unwrap_or(remaining_content.len()) + ge_pos;
            edi837p.ge = remaining_content[ge_pos..=ge_end].to_string();
            remaining_content = remaining_content[ge_end + 1..].to_string();
        }
        
        if let Some(iea_pos) = remaining_content.find("IEA*") {
            let iea_end = remaining_content[iea_pos..].find('~').unwrap_or(remaining_content.len()) + iea_pos;
            edi837p.iea = remaining_content[iea_pos..=iea_end].to_string();
            remaining_content = remaining_content[iea_end + 1..].to_string();
        }
        
        Ok((edi837p, remaining_content))
    }
    
    fn to_edi(&self) -> String {
        info!("Generating EDI837P content");
        
        let mut result = String::new();
        
        // Write interchange header
        result.push_str(&self.isa);
        result.push_str("\n");
        result.push_str(&self.gs);
        result.push_str("\n");
        result.push_str(&self.st);
        result.push_str("\n");
        
        // Write table1
        result.push_str(&self.table1.table1.bht);
        result.push_str("\n");
        
        // Write loop2000a
        result.push_str(&write_loop2000a(&self.table1.loop2000a));
        
        // Write loop2010aa
        result.push_str(&write_loop2010aa(&self.loop2010aa));
        
        // Write loop2010ab if present
        if let Some(loop2010ab) = &self.loop2010ab {
            result.push_str(&write_loop2010ab(loop2010ab));
        }
        
        // Write loop2010ac if present
        if let Some(loop2010ac) = &self.loop2010ac {
            result.push_str(&write_loop2010ac(loop2010ac));
        }
        
        // Write loop2000b
        for loop2000b in &self.loop2000b {
            result.push_str(&write_loop2000b(loop2000b));
        }
        
        // Write loop2000c
        for loop2000c in &self.loop2000c {
            result.push_str(&write_loop2000c(loop2000c));
        }
        
        // Write loop2300
        for loop2300 in &self.loop2300 {
            result.push_str(&write_loop2300(loop2300));
            
            // Write loop2400 for each claim
            for loop2400 in &loop2300.loop2400 {
                result.push_str(&write_loop2400(loop2400));
            }
        }
        
        // Write interchange trailer
        result.push_str(&self.se);
        result.push_str("\n");
        result.push_str(&self.ge);
        result.push_str("\n");
        result.push_str(&self.iea);
        result.push_str("\n");
        
        result
    }
    
    fn get_transaction_type() -> &'static str {
        "837P"
    }
    
    fn detect(contents: &str) -> bool {
        contents.contains("ST*837*") && contents.contains("BHT*0019*00*")
    }
}

impl TransactionSet for Edi837I {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        info!("Parsing EDI837I content");
        
        let mut edi837i = Edi837I::default();
        let mut remaining_content = contents.clone();
        
        // Parse interchange header
        if let Some(isa_pos) = remaining_content.find("ISA*") {
            let isa_end = remaining_content[isa_pos..].find('~').unwrap_or(remaining_content.len()) + isa_pos;
            edi837i.isa = remaining_content[isa_pos..=isa_end].to_string();
            remaining_content = remaining_content[isa_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ISA segment not found".to_string()));
        }
        
        // Parse GS segment
        if let Some(gs_pos) = remaining_content.find("GS*") {
            let gs_end = remaining_content[gs_pos..].find('~').unwrap_or(remaining_content.len()) + gs_pos;
            edi837i.gs = remaining_content[gs_pos..=gs_end].to_string();
            remaining_content = remaining_content[gs_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("GS segment not found".to_string()));
        }
        
        // Parse ST segment
        if let Some(st_pos) = remaining_content.find("ST*") {
            let st_end = remaining_content[st_pos..].find('~').unwrap_or(remaining_content.len()) + st_pos;
            edi837i.st = remaining_content[st_pos..=st_end].to_string();
            remaining_content = remaining_content[st_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ST segment not found".to_string()));
        }
        
        // Parse BHT segment
        if let Some(bht_pos) = remaining_content.find("BHT*") {
            let bht_end = remaining_content[bht_pos..].find('~').unwrap_or(remaining_content.len()) + bht_pos;
            edi837i.table1.table1.bht = remaining_content[bht_pos..=bht_end].to_string();
            remaining_content = remaining_content[bht_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("BHT segment not found".to_string()));
        }
        
        // Parse Loop2000A (Billing Provider Hierarchical Level)
        let (loop2000a, remaining) = parse_loop2000a(&remaining_content);
        edi837i.table1.loop2000a = loop2000a;
        remaining_content = remaining;
        
        // Parse Loop2010AA (Billing Provider Name)
        let (loop2010aa, remaining) = parse_loop2010aa(&remaining_content);
        edi837i.loop2010aa = loop2010aa;
        remaining_content = remaining;
        
        // Parse Loop2010AB (Pay-to Address) if present
        if remaining_content.contains("NM1*87*") {
            let (loop2010ab, remaining) = parse_loop2010ab(&remaining_content);
            edi837i.loop2010ab = Some(loop2010ab);
            remaining_content = remaining;
        }
        
        // Parse Loop2010AC (Pay-to Plan Name) if present
        if remaining_content.contains("NM1*PE*") {
            let (loop2010ac, remaining) = parse_loop2010ac(&remaining_content);
            edi837i.loop2010ac = Some(loop2010ac);
            remaining_content = remaining;
        }
        
        // Parse Loop2000B (Subscriber Hierarchical Level)
        let mut loop2000b_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*22*") {
            let (loop2000b, remaining) = parse_loop2000b(&remaining_content);
            if loop2000b.hl.is_empty() {
                break;
            }
            loop2000b_vec.push(loop2000b);
            remaining_content = remaining;
        }
        edi837i.loop2000b = loop2000b_vec;
        
        // Parse Loop2000C (Patient Hierarchical Level)
        let mut loop2000c_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*23*") {
            let (loop2000c, remaining) = parse_loop2000c(&remaining_content);
            if loop2000c.hl.is_empty() {
                break;
            }
            loop2000c_vec.push(loop2000c);
            remaining_content = remaining;
        }
        edi837i.loop2000c = loop2000c_vec;
        
        // Parse Loop2300 (Claim Information)
        let mut loop2300_vec = Vec::new();
        while remaining_content.contains("CLM*") {
            let (mut loop2300, remaining) = parse_loop2300(&remaining_content);
            if loop2300.clm.is_empty() {
                break;
            }
            
            // Look for CL1 segments (specific to 837I)
            let mut cl1_segments = Vec::new();
            let mut temp_content = remaining.clone();
            let mut segment_end = 0;
            
            while let Some(segment_start) = temp_content[segment_end..].find("CL1*") {
                let start_pos = segment_end + segment_start;
                let end_pos = temp_content[start_pos..].find('~').unwrap_or(temp_content.len()) + start_pos;
                let segment = temp_content[start_pos..=end_pos].to_string();
                
                if let Some(cl1_segment) = parse_cl1_segment(&segment) {
                    cl1_segments.push(cl1_segment);
                }
                
                segment_end = end_pos + 1;
                
                // Break if we've reached the next LX segment
                if temp_content[segment_end..].contains("LX*") {
                    break;
                }
            }
            
            // Add CL1 segments to Loop2300
            loop2300.cl1_segments = cl1_segments;
            
            loop2300_vec.push(loop2300);
            remaining_content = remaining;
            
            // Parse Loop2400 (Service Line Information) for this claim
            let mut loop2400_vec = Vec::new();
            while remaining_content.contains("LX*") {
                let (loop2400, remaining) = parse_loop2400(&remaining_content);
                if loop2400.lx.is_empty() {
                    break;
                }
                loop2400_vec.push(loop2400);
                remaining_content = remaining;
            }
            
            // Add service lines to the claim
            if !loop2400_vec.is_empty() {
                let last_index = loop2300_vec.len() - 1;
                loop2300_vec[last_index].loop2400 = loop2400_vec;
            }
        }
        edi837i.loop2300 = loop2300_vec;
        
        // Parse interchange trailer
        if let Some(se_pos) = remaining_content.find("SE*") {
            let se_end = remaining_content[se_pos..].find('~').unwrap_or(remaining_content.len()) + se_pos;
            edi837i.se = remaining_content[se_pos..=se_end].to_string();
            remaining_content = remaining_content[se_end + 1..].to_string();
        }
        
        if let Some(ge_pos) = remaining_content.find("GE*") {
            let ge_end = remaining_content[ge_pos..].find('~').unwrap_or(remaining_content.len()) + ge_pos;
            edi837i.ge = remaining_content[ge_pos..=ge_end].to_string();
            remaining_content = remaining_content[ge_end + 1..].to_string();
        }
        
        if let Some(iea_pos) = remaining_content.find("IEA*") {
            let iea_end = remaining_content[iea_pos..].find('~').unwrap_or(remaining_content.len()) + iea_pos;
            edi837i.iea = remaining_content[iea_pos..=iea_end].to_string();
            remaining_content = remaining_content[iea_end + 1..].to_string();
        }
        
        Ok((edi837i, remaining_content))
    }
    
    fn to_edi(&self) -> String {
        info!("Generating EDI837I content");
        
        let mut result = String::new();
        
        // Write interchange header
        result.push_str(&self.isa);
        result.push_str("\n");
        result.push_str(&self.gs);
        result.push_str("\n");
        result.push_str(&self.st);
        result.push_str("\n");
        
        // Write table1
        result.push_str(&self.table1.table1.bht);
        result.push_str("\n");
        
        // Write loop2000a
        result.push_str(&write_loop2000a(&self.table1.loop2000a));
        
        // Write loop2010aa
        result.push_str(&write_loop2010aa(&self.loop2010aa));
        
        // Write loop2010ab if present
        if let Some(loop2010ab) = &self.loop2010ab {
            result.push_str(&write_loop2010ab(loop2010ab));
        }
        
        // Write loop2010ac if present
        if let Some(loop2010ac) = &self.loop2010ac {
            result.push_str(&write_loop2010ac(loop2010ac));
        }
        
        // Write loop2000b
        for loop2000b in &self.loop2000b {
            result.push_str(&write_loop2000b(loop2000b));
        }
        
        // Write loop2000c
        for loop2000c in &self.loop2000c {
            result.push_str(&write_loop2000c(loop2000c));
        }
        
        // Write loop2300
        for loop2300 in &self.loop2300 {
            result.push_str(&write_loop2300(loop2300));
            
            // Write loop2400 for each claim
            for loop2400 in &loop2300.loop2400 {
                result.push_str(&write_loop2400(loop2400));
            }
        }
        
        // Write interchange trailer
        result.push_str(&self.se);
        result.push_str("\n");
        result.push_str(&self.ge);
        result.push_str("\n");
        result.push_str(&self.iea);
        result.push_str("\n");
        
        result
    }
    
    fn get_transaction_type() -> &'static str {
        "837I"
    }
    
    fn detect(contents: &str) -> bool {
        contents.contains("ST*837*") && contents.contains("BHT*0019*00*")
    }
}

impl TransactionSet for Edi837D {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        info!("Parsing EDI837D content");
        
        let mut edi837d = Edi837D::default();
        let mut remaining_content = contents.clone();
        
        // Parse interchange header
        if let Some(isa_pos) = remaining_content.find("ISA*") {
            let isa_end = remaining_content[isa_pos..].find('~').unwrap_or(remaining_content.len()) + isa_pos;
            edi837d.isa = remaining_content[isa_pos..=isa_end].to_string();
            remaining_content = remaining_content[isa_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ISA segment not found".to_string()));
        }
        
        // Parse GS segment
        if let Some(gs_pos) = remaining_content.find("GS*") {
            let gs_end = remaining_content[gs_pos..].find('~').unwrap_or(remaining_content.len()) + gs_pos;
            edi837d.gs = remaining_content[gs_pos..=gs_end].to_string();
            remaining_content = remaining_content[gs_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("GS segment not found".to_string()));
        }
        
        // Parse ST segment
        if let Some(st_pos) = remaining_content.find("ST*") {
            let st_end = remaining_content[st_pos..].find('~').unwrap_or(remaining_content.len()) + st_pos;
            edi837d.st = remaining_content[st_pos..=st_end].to_string();
            remaining_content = remaining_content[st_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("ST segment not found".to_string()));
        }
        
        // Parse BHT segment
        if let Some(bht_pos) = remaining_content.find("BHT*") {
            let bht_end = remaining_content[bht_pos..].find('~').unwrap_or(remaining_content.len()) + bht_pos;
            edi837d.table1.table1.bht = remaining_content[bht_pos..=bht_end].to_string();
            remaining_content = remaining_content[bht_end + 1..].to_string();
        } else {
            return Err(EdiError::MissingSegment("BHT segment not found".to_string()));
        }
        
        // Parse Loop2000A (Billing Provider Hierarchical Level)
        let (loop2000a, remaining) = parse_loop2000a(&remaining_content);
        edi837d.table1.loop2000a = loop2000a;
        remaining_content = remaining;
        
        // Parse Loop2010AA (Billing Provider Name)
        let (loop2010aa, remaining) = parse_loop2010aa(&remaining_content);
        edi837d.loop2010aa = loop2010aa;
        remaining_content = remaining;
        
        // Parse Loop2010AB (Pay-to Address) if present
        if remaining_content.contains("NM1*87*") {
            let (loop2010ab, remaining) = parse_loop2010ab(&remaining_content);
            edi837d.loop2010ab = Some(loop2010ab);
            remaining_content = remaining;
        }
        
        // Parse Loop2010AC (Pay-to Plan Name) if present
        if remaining_content.contains("NM1*PE*") {
            let (loop2010ac, remaining) = parse_loop2010ac(&remaining_content);
            edi837d.loop2010ac = Some(loop2010ac);
            remaining_content = remaining;
        }
        
        // Parse Loop2000B (Subscriber Hierarchical Level)
        let mut loop2000b_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*22*") {
            let (loop2000b, remaining) = parse_loop2000b(&remaining_content);
            if loop2000b.hl.is_empty() {
                break;
            }
            loop2000b_vec.push(loop2000b);
            remaining_content = remaining;
        }
        edi837d.loop2000b = loop2000b_vec;
        
        // Parse Loop2000C (Patient Hierarchical Level)
        let mut loop2000c_vec = Vec::new();
        while remaining_content.contains("HL*") && remaining_content.contains("*23*") {
            let (loop2000c, remaining) = parse_loop2000c(&remaining_content);
            if loop2000c.hl.is_empty() {
                break;
            }
            loop2000c_vec.push(loop2000c);
            remaining_content = remaining;
        }
        edi837d.loop2000c = loop2000c_vec;
        
        // Parse Loop2300 (Claim Information)
        let mut loop2300_vec = Vec::new();
        while remaining_content.contains("CLM*") {
            let (mut loop2300, remaining) = parse_loop2300(&remaining_content);
            if loop2300.clm.is_empty() {
                break;
            }
            
            // Look for TOO segments (specific to 837D)
            let mut too_segments = Vec::new();
            let mut temp_content = remaining.clone();
            let mut segment_end = 0;
            
            while let Some(segment_start) = temp_content[segment_end..].find("TOO*") {
                let start_pos = segment_end + segment_start;
                let end_pos = temp_content[start_pos..].find('~').unwrap_or(temp_content.len()) + start_pos;
                let segment = temp_content[start_pos..=end_pos].to_string();
                
                if let Some(too_segment) = parse_too_segment(&segment) {
                    too_segments.push(too_segment);
                }
                
                segment_end = end_pos + 1;
                
                // Break if we've reached the next LX segment
                if temp_content[segment_end..].contains("LX*") {
                    break;
                }
            }
            
            // Add TOO segments to Loop2300
            loop2300.too_segments = too_segments;
            
            loop2300_vec.push(loop2300);
            remaining_content = remaining;
            
            // Parse Loop2400 (Service Line Information) for this claim
            let mut loop2400_vec = Vec::new();
            while remaining_content.contains("LX*") {
                let (loop2400, remaining) = parse_loop2400(&remaining_content);
                if loop2400.lx.is_empty() {
                    break;
                }
                loop2400_vec.push(loop2400);
                remaining_content = remaining;
            }
            
            // Add service lines to the claim
            if !loop2400_vec.is_empty() {
                let last_index = loop2300_vec.len() - 1;
                loop2300_vec[last_index].loop2400 = loop2400_vec;
            }
        }
        edi837d.loop2300 = loop2300_vec;
        
        // Parse interchange trailer
        if let Some(se_pos) = remaining_content.find("SE*") {
            let se_end = remaining_content[se_pos..].find('~').unwrap_or(remaining_content.len()) + se_pos;
            edi837d.se = remaining_content[se_pos..=se_end].to_string();
            remaining_content = remaining_content[se_end + 1..].to_string();
        }
        
        if let Some(ge_pos) = remaining_content.find("GE*") {
            let ge_end = remaining_content[ge_pos..].find('~').unwrap_or(remaining_content.len()) + ge_pos;
            edi837d.ge = remaining_content[ge_pos..=ge_end].to_string();
            remaining_content = remaining_content[ge_end + 1..].to_string();
        }
        
        if let Some(iea_pos) = remaining_content.find("IEA*") {
            let iea_end = remaining_content[iea_pos..].find('~').unwrap_or(remaining_content.len()) + iea_pos;
            edi837d.iea = remaining_content[iea_pos..=iea_end].to_string();
            remaining_content = remaining_content[iea_end + 1..].to_string();
        }
        
        Ok((edi837d, remaining_content))
    }
    
    fn to_edi(&self) -> String {
        info!("Generating EDI837D content");
        
        let mut result = String::new();
        
        // Write interchange header
        result.push_str(&self.isa);
        result.push_str("\n");
        result.push_str(&self.gs);
        result.push_str("\n");
        result.push_str(&self.st);
        result.push_str("\n");
        
        // Write table1
        result.push_str(&self.table1.table1.bht);
        result.push_str("\n");
        
        // Write loop2000a
        result.push_str(&write_loop2000a(&self.table1.loop2000a));
        
        // Write loop2010aa
        result.push_str(&write_loop2010aa(&self.loop2010aa));
        
        // Write loop2010ab if present
        if let Some(loop2010ab) = &self.loop2010ab {
            result.push_str(&write_loop2010ab(loop2010ab));
        }
        
        // Write loop2010ac if present
        if let Some(loop2010ac) = &self.loop2010ac {
            result.push_str(&write_loop2010ac(loop2010ac));
        }
        
        // Write loop2000b
        for loop2000b in &self.loop2000b {
            result.push_str(&write_loop2000b(loop2000b));
        }
        
        // Write loop2000c
        for loop2000c in &self.loop2000c {
            result.push_str(&write_loop2000c(loop2000c));
        }
        
        // Write loop2300
        for loop2300 in &self.loop2300 {
            result.push_str(&write_loop2300(loop2300));
            
            // Write loop2400 for each claim
            for loop2400 in &loop2300.loop2400 {
                result.push_str(&write_loop2400(loop2400));
            }
        }
        
        // Write interchange trailer
        result.push_str(&self.se);
        result.push_str("\n");
        result.push_str(&self.ge);
        result.push_str("\n");
        result.push_str(&self.iea);
        result.push_str("\n");
        
        result
    }
    
    fn get_transaction_type() -> &'static str {
        "837D"
    }
    
    fn detect(contents: &str) -> bool {
        contents.contains("ST*837*") && contents.contains("BHT*0019*00*")
    }
}

/// Parse EDI837P content
pub fn get_837p(content: &str) -> EdiResult<Edi837P> {
    info!("Parsing EDI837P content");
    
    match Edi837P::parse(content.to_string()) {
        Ok((edi837p, _)) => Ok(edi837p),
        Err(e) => Err(e)
    }
}

/// Generate EDI837P content
pub fn write_837p(edi837p: &Edi837P) -> EdiResult<String> {
    info!("Generating EDI837P content");
    
    let mut result = String::new();
    
    // Write ISA segment
    result.push_str(&edi837p.isa);
    result.push('~');
    
    // Write GS segment
    result.push_str(&edi837p.gs);
    result.push('~');
    
    // Write ST segment
    result.push_str(&edi837p.st);
    result.push('~');
    
    // Write BHT segment
    result.push_str(&edi837p.table1.table1.bht);
    result.push('~');
    
    // Write Loop2000A (Billing Provider Hierarchical Level)
    result.push_str(&write_loop2000a(&edi837p.table1.loop2000a));
    
    // Write Loop2010AA (Billing Provider Name)
    result.push_str(&write_loop2010aa(&edi837p.loop2010aa));
    
    // Write Loop2010AB (Pay-to Address) if present
    if let Some(loop2010ab) = &edi837p.loop2010ab {
        result.push_str(&write_loop2010ab(loop2010ab));
    }
    
    // Write Loop2010AC (Pay-to Plan Name) if present
    if let Some(loop2010ac) = &edi837p.loop2010ac {
        result.push_str(&write_loop2010ac(loop2010ac));
    }
    
    // Write Loop2000B (Subscriber Hierarchical Level)
    for loop2000b in &edi837p.loop2000b {
        result.push_str(&write_loop2000b(loop2000b));
    }
    
    // Write Loop2000C (Patient Hierarchical Level)
    for loop2000c in &edi837p.loop2000c {
        result.push_str(&write_loop2000c(loop2000c));
    }
    
    // Write Loop2300 (Claim Information)
    for loop2300 in &edi837p.loop2300 {
        result.push_str(&write_loop2300(loop2300));
    }
    
    // Write Loop2400 (Service Line)
    for loop2400 in &edi837p.loop2400 {
        result.push_str(&write_loop2400(loop2400));
    }
    
    // Write SE segment
    result.push_str(&edi837p.se);
    result.push('~');
    
    // Write GE segment
    result.push_str(&edi837p.ge);
    result.push('~');
    
    // Write IEA segment
    result.push_str(&edi837p.iea);
    result.push('~');
    
    Ok(result)
}

/// Parse EDI837I content
pub fn get_837i(content: &str) -> EdiResult<Edi837I> {
    info!("Parsing EDI837I content");
    
    match Edi837I::parse(content.to_string()) {
        Ok((edi837i, _)) => Ok(edi837i),
        Err(e) => Err(e)
    }
}

/// Generate EDI837I content
pub fn write_837i(edi837i: &Edi837I) -> EdiResult<String> {
    info!("Generating EDI837I content");
    
    let mut result = String::new();
    
    // Write ISA segment
    result.push_str(&edi837i.isa);
    result.push('~');
    
    // Write GS segment
    result.push_str(&edi837i.gs);
    result.push('~');
    
    // Write ST segment
    result.push_str(&edi837i.st);
    result.push('~');
    
    // Write BHT segment
    result.push_str(&edi837i.table1.table1.bht);
    result.push('~');
    
    // Write Loop2000A (Billing Provider Hierarchical Level)
    result.push_str(&write_loop2000a(&edi837i.table1.loop2000a));
    
    // Write Loop2010AA (Billing Provider Name)
    result.push_str(&write_loop2010aa(&edi837i.loop2010aa));
    
    // Write Loop2010AB (Pay-to Address) if present
    if let Some(loop2010ab) = &edi837i.loop2010ab {
        result.push_str(&write_loop2010ab(loop2010ab));
    }
    
    // Write Loop2010AC (Pay-to Plan Name) if present
    if let Some(loop2010ac) = &edi837i.loop2010ac {
        result.push_str(&write_loop2010ac(loop2010ac));
    }
    
    // Write Loop2000B (Subscriber Hierarchical Level)
    for loop2000b in &edi837i.loop2000b {
        result.push_str(&write_loop2000b(loop2000b));
    }
    
    // Write Loop2000C (Patient Hierarchical Level)
    for loop2000c in &edi837i.loop2000c {
        result.push_str(&write_loop2000c(loop2000c));
    }
    
    // Write Loop2300 (Claim Information)
    for loop2300 in &edi837i.loop2300 {
        let mut loop2300_str = write_loop2300(loop2300);
        
        // Check for CL1 segment in the raw segments
        for ref_seg in &loop2300.ref_segments {
            if ref_seg.starts_with("CL1*") {
                loop2300_str.push_str(ref_seg);
                loop2300_str.push('~');
            }
        }
        
        result.push_str(&loop2300_str);
    }
    
    // Write Loop2400 (Service Line)
    for loop2400 in &edi837i.loop2400 {
        result.push_str(&write_loop2400(loop2400));
    }
    
    // Write SE segment
    result.push_str(&edi837i.se);
    result.push('~');
    
    // Write GE segment
    result.push_str(&edi837i.ge);
    result.push('~');
    
    // Write IEA segment
    result.push_str(&edi837i.iea);
    result.push('~');
    
    Ok(result)
}

/// Parse EDI837D content
pub fn get_837d(content: &str) -> EdiResult<Edi837D> {
    info!("Parsing EDI837D content");
    
    match Edi837D::parse(content.to_string()) {
        Ok((edi837d, _)) => Ok(edi837d),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_837p_ambulance() {
        let content = include_str!("../../demo/005010X222 Health Care Claim Professional/X222-ambulance.edi");
        
        let result = get_837p(content);
        assert!(result.is_ok(), "Failed to parse 837P: {:?}", result.err());
        
        let edi837p = result.unwrap();
        
        // Verify basic structure
        assert!(!edi837p.isa.is_empty(), "ISA segment should not be empty");
        assert!(!edi837p.gs.is_empty(), "GS segment should not be empty");
        assert!(!edi837p.st.is_empty(), "ST segment should not be empty");
        assert!(!edi837p.table1.table1.bht.is_empty(), "BHT segment should not be empty");
        
        // Verify billing provider
        assert!(!edi837p.table1.loop2000a.hl.is_empty(), "HL segment for billing provider should not be empty");
        assert!(edi837p.table1.loop2000a.hl.contains("HL*1**20*1"), "HL segment should contain correct values");
        
        // Verify subscriber
        assert!(!edi837p.loop2000b.is_empty(), "Should have at least one subscriber");
        assert!(edi837p.loop2000b[0].hl.contains("HL*2*1*22*0"), "HL segment for subscriber should contain correct values");
        
        // Verify claims
        assert!(!edi837p.loop2300.is_empty(), "Should have at least one claim");
        assert!(edi837p.loop2300[0].clm.contains("CLM*051068*766.50"), "CLM segment should contain correct values");
        
        // Verify service lines
        assert!(!edi837p.loop2300[0].loop2400.is_empty(), "Should have at least one service line");
        assert!(edi837p.loop2300[0].loop2400[0].lx.contains("LX*1"), "LX segment should contain correct values");
        assert!(edi837p.loop2300[0].loop2400[0].sv1.as_ref().unwrap().contains("SV1*HC:A0427:RH*700*UN*1"), 
                "SV1 segment should contain correct values");
    }
    
    #[test]
    fn test_write_837p_ambulance() {
        let content = include_str!("../../demo/005010X222 Health Care Claim Professional/X222-ambulance.edi");
        
        let result = get_837p(content);
        assert!(result.is_ok(), "Failed to parse 837P: {:?}", result.err());
        
        let edi837p = result.unwrap();
        
        // Generate EDI from the parsed structure
        let write_result = write_837p(&edi837p);
        assert!(write_result.is_ok(), "Failed to write 837P: {:?}", write_result.err());
        
        let generated_edi = write_result.unwrap();
        
        // Verify the generated EDI contains key segments
        assert!(generated_edi.contains("ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*"), 
                "Generated EDI should contain ISA segment");
        assert!(generated_edi.contains("GS*HC*1234567890*9876543210*"), 
                "Generated EDI should contain GS segment");
        assert!(generated_edi.contains("ST*837*000017712*005010X222A1"), 
                "Generated EDI should contain ST segment");
        assert!(generated_edi.contains("BHT*0019*00*000017712*20050208*1112*CH"), 
                "Generated EDI should contain BHT segment");
        assert!(generated_edi.contains("HL*1**20*1"), 
                "Generated EDI should contain billing provider HL segment");
        assert!(generated_edi.contains("HL*2*1*22*0"), 
                "Generated EDI should contain subscriber HL segment");
        assert!(generated_edi.contains("CLM*051068*766.50"), 
                "Generated EDI should contain CLM segment");
        assert!(generated_edi.contains("SV1*HC:A0427:RH*700*UN*1"), 
                "Generated EDI should contain SV1 segment");
    }
    
    #[test]
    fn test_parse_837p_commercial_health_insurance() {
        let content = include_str!("../../demo/005010X222 Health Care Claim Professional/X222-commercial-health-insurance.edi");
        
        let result = get_837p(content);
        assert!(result.is_ok(), "Failed to parse 837P: {:?}", result.err());
        
        let edi837p = result.unwrap();
        
        // Verify basic structure
        assert!(!edi837p.isa.is_empty(), "ISA segment should not be empty");
        assert!(!edi837p.gs.is_empty(), "GS segment should not be empty");
        assert!(!edi837p.st.is_empty(), "ST segment should not be empty");
        assert!(!edi837p.table1.table1.bht.is_empty(), "BHT segment should not be empty");
        
        // Verify billing provider
        assert!(!edi837p.table1.loop2000a.hl.is_empty(), "HL segment for billing provider should not be empty");
        
        // Verify subscriber
        assert!(!edi837p.loop2000b.is_empty(), "Should have at least one subscriber");
        
        // Verify claims
        assert!(!edi837p.loop2300.is_empty(), "Should have at least one claim");
        
        // Verify service lines
        assert!(!edi837p.loop2300[0].loop2400.is_empty(), "Should have at least one service line");
    }
    
    #[test]
    fn test_parse_837i_institutional_claim() {
        let content = include_str!("../../demo/005010X223 Health Care Claim Institutional/X223-837-institutional-claim.edi");
        
        let result = get_837i(content);
        assert!(result.is_ok(), "Failed to parse 837I: {:?}", result.err());
        
        let edi837i = result.unwrap();
        
        // Verify basic structure
        assert!(!edi837i.isa.is_empty(), "ISA segment should not be empty");
        assert!(!edi837i.gs.is_empty(), "GS segment should not be empty");
        assert!(!edi837i.st.is_empty(), "ST segment should not be empty");
        assert!(!edi837i.table1.table1.bht.is_empty(), "BHT segment should not be empty");
        
        // Verify billing provider
        assert!(!edi837i.table1.loop2000a.hl.is_empty(), "HL segment for billing provider should not be empty");
        assert!(edi837i.table1.loop2000a.hl.contains("HL*1**20*1"), "HL segment should contain correct values");
        
        // Verify subscriber
        assert!(!edi837i.loop2000b.is_empty(), "Should have at least one subscriber");
        assert!(edi837i.loop2000b[0].hl.contains("HL*2*1*22*0"), "HL segment for subscriber should contain correct values");
        
        // Verify claims
        assert!(!edi837i.loop2300.is_empty(), "Should have at least one claim");
        assert!(edi837i.loop2300[0].clm.contains("CLM*756048Q*89.93"), "CLM segment should contain correct values");
        
        // Verify service lines
        assert!(!edi837i.loop2300[0].loop2400.is_empty(), "Should have at least one service line");
        assert!(edi837i.loop2300[0].loop2400[0].lx.contains("LX*1"), "LX segment should contain correct values");
        assert!(edi837i.loop2300[0].loop2400[0].sv2.as_ref().unwrap().contains("SV2*0305*HC:85025*13.39*UN*1"), 
                "SV2 segment should contain correct values");
    }
    
    #[test]
    fn test_write_837i_institutional_claim() {
        let content = include_str!("../../demo/005010X223 Health Care Claim Institutional/X223-837-institutional-claim.edi");
        
        let result = get_837i(content);
        assert!(result.is_ok(), "Failed to parse 837I: {:?}", result.err());
        
        let edi837i = result.unwrap();
        
        // Generate EDI from the parsed structure
        let write_result = write_837i(&edi837i);
        assert!(write_result.is_ok(), "Failed to write 837I: {:?}", write_result.err());
        
        let generated_edi = write_result.unwrap();
        
        // Verify the generated EDI contains key segments
        assert!(generated_edi.contains("ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*"), 
                "Generated EDI should contain ISA segment");
        assert!(generated_edi.contains("GS*HC*1234567890*9876543210*"), 
                "Generated EDI should contain GS segment");
        assert!(generated_edi.contains("ST*837*987654*005010X223A2"), 
                "Generated EDI should contain ST segment");
        assert!(generated_edi.contains("BHT*0019*00*0123*19960918*0932*CH"), 
                "Generated EDI should contain BHT segment");
        assert!(generated_edi.contains("HL*1**20*1"), 
                "Generated EDI should contain billing provider HL segment");
        assert!(generated_edi.contains("HL*2*1*22*0"), 
                "Generated EDI should contain subscriber HL segment");
        assert!(generated_edi.contains("CLM*756048Q*89.93"), 
                "Generated EDI should contain CLM segment");
        assert!(generated_edi.contains("SV2*0305*HC:85025*13.39*UN*1"), 
                "Generated EDI should contain SV2 segment");
    }
}
/// Generate EDI837D content
pub fn write_837d(edi837d: &Edi837D) -> EdiResult<String> {
    info!("Generating EDI837D content");
    
    let mut result = String::new();
    
    // Write ISA segment
    result.push_str(&edi837d.isa);
    result.push('~');
    
    // Write GS segment
    result.push_str(&edi837d.gs);
    result.push('~');
    
    // Write ST segment
    result.push_str(&edi837d.st);
    result.push('~');
    
    // Write BHT segment
    result.push_str(&edi837d.table1.table1.bht);
    result.push('~');
    
    // Write Loop2000A (Billing Provider Hierarchical Level)
    result.push_str(&write_loop2000a(&edi837d.table1.loop2000a));
    
    // Write Loop2010AA (Billing Provider Name)
    result.push_str(&write_loop2010aa(&edi837d.loop2010aa));
    
    // Write Loop2010AB (Pay-to Address) if present
    if let Some(loop2010ab) = &edi837d.loop2010ab {
        result.push_str(&write_loop2010ab(loop2010ab));
    }
    
    // Write Loop2010AC (Pay-to Plan Name) if present
    if let Some(loop2010ac) = &edi837d.loop2010ac {
        result.push_str(&write_loop2010ac(loop2010ac));
    }
    
    // Write Loop2000B (Subscriber Hierarchical Level)
    for loop2000b in &edi837d.loop2000b {
        result.push_str(&write_loop2000b(loop2000b));
    }
    
    // Write Loop2000C (Patient Hierarchical Level)
    for loop2000c in &edi837d.loop2000c {
        result.push_str(&write_loop2000c(loop2000c));
    }
    
    // Write Loop2300 (Claim Information)
    for loop2300 in &edi837d.loop2300 {
        let mut loop2300_str = write_loop2300(loop2300);
        
        // Add TOO segments if present (specific to 837D)
        for too in &loop2300.too_segments {
            loop2300_str.push_str(too);
            loop2300_str.push('~');
        }
        
        result.push_str(&loop2300_str);
    }
    
    // Write Loop2400 (Service Line)
    for loop2400 in &edi837d.loop2400 {
        result.push_str(&write_loop2400(loop2400));
    }
    
    // Write SE segment
    result.push_str(&edi837d.se);
    result.push('~');
    
    // Write GE segment
    result.push_str(&edi837d.ge);
    result.push('~');
    
    // Write IEA segment
    result.push_str(&edi837d.iea);
    result.push('~');
    
    Ok(result)
}
/// Implement specialized handling for TOO segment in 837D
fn parse_too_segment(segment: &str) -> Option<String> {
    if segment.starts_with("TOO*") {
        return Some(segment.to_string());
    }
    None
}

/// Implement specialized handling for CL1 segment in 837I
fn parse_cl1_segment(segment: &str) -> Option<String> {
    if segment.starts_with("CL1*") {
        return Some(segment.to_string());
    }
    None
}
