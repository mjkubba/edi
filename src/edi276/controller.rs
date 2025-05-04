use log::info;

use serde::{Serialize, Deserialize};

use crate::edi276::interchangecontrol::*;
use crate::edi276::table1::*;
use crate::edi276::loop2000::*;
use crate::edi276::interchangecontroltrailer::*;
use crate::error::EdiResult;

/// Table1Combined structure for EDI 276
/// Contains the ST, BHT, and other header segments
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1Combined {
    pub table1: Table1s,
}

/// Main structure for EDI 276 (Health Care Claim Status Request)
/// Contains all segments and loops for the 276 transaction set
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi276 {
    pub interchange_header: InterchangeHeader,
    pub table1_combined: Table1Combined,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segment: String,
    pub interchange_trailer: InterchangeTrailer,
}

/// Parse an EDI 276 file into an Edi276 structure
/// 
/// # Arguments
/// * `contents` - The EDI 276 file contents as a string
/// 
/// # Returns
/// * `EdiResult<Edi276>` - The parsed EDI 276 structure or an error
pub fn get_276(mut contents: String) -> EdiResult<Edi276> {
    let interchange_header;
    let table1s;
    let loop2000a;
    let loop2000b_vec;
    let interchange_trailer;
    let table1_combined;
    let se_segment;
    
    // Remove BOM if present
    contents = contents.trim_start_matches("\u{feff}").to_string();
    
    // Remove carriage returns and line feeds
    contents = contents.replace("\r", "").replace("\n", "");

    // Control Segments
    (interchange_header, contents) = get_interchange_header(contents.clone());

    // Table 1
    (table1s, contents) = get_table1s(contents.clone());
    
    // Loop 2000A - Information Source
    (loop2000a, contents) = get_loop_2000a(contents.clone());
    
    // Loop 2000B - Information Receiver
    (loop2000b_vec, contents) = get_loop_2000b_vec(contents.clone());
    
    // Extract SE segment
    if let Some(se_segment_start) = contents.find("SE") {
        let se_segment_end = contents[se_segment_start..].find('~').unwrap_or(contents.len() - se_segment_start);
        se_segment = contents[se_segment_start..se_segment_start + se_segment_end].to_string();
        contents = contents[se_segment_start + se_segment_end + 1..].to_string();
    } else {
        se_segment = String::new();
    }

    // Control Trailer
    (interchange_trailer, contents) = get_interchange_trailer(contents.clone());

    // Combined Table 1
    table1_combined = Table1Combined {
        table1: table1s.clone(),
    };

    let edi276 = Edi276 {
        interchange_header,
        table1_combined,
        loop2000a,
        loop2000b: loop2000b_vec,
        se_segment,
        interchange_trailer,
    };
    
    info!("Unprocessed segments: {:?}", contents);
    Ok(edi276)
}

/// Generate an EDI 276 file from an Edi276 structure
/// 
/// # Arguments
/// * `edi276` - The Edi276 structure to convert to an EDI file
/// 
/// # Returns
/// * `String` - The generated EDI 276 file contents
pub fn write_276(edi276: &Edi276) -> String {
    let mut new_edi = String::new();
    
    // Write interchange header
    let new_ich = write_interchange_control(&edi276.interchange_header);
    new_edi.push_str(&new_ich);
    new_edi.push('\n');
    
    // Write Table 1
    let new_table1s = write_table1(&edi276.table1_combined.table1);
    new_edi.push_str(&new_table1s);
    new_edi.push('\n');
    
    // Write Loop 2000A
    let new_loop2000a = write_loop_2000a(&edi276.loop2000a);
    new_edi.push_str(&new_loop2000a);
    
    // Write Loop 2000B
    let new_loop2000b = write_loop_2000b_vec(&edi276.loop2000b);
    new_edi.push_str(&new_loop2000b);
    
    // Write SE segment
    new_edi.push_str(&edi276.se_segment);
    new_edi.push_str("~\n");
    
    // Write interchange trailer
    let new_ict = write_interchange_trailer(&edi276.interchange_trailer);
    new_edi.push_str(&new_ict);
    
    info!("Generated EDI 276: {}", new_edi);
    new_edi
}

/// Function to detect if JSON contains 276 format data
/// 
/// # Arguments
/// * `contents` - The JSON string to check
/// 
/// # Returns
/// * `bool` - True if the JSON contains 276 format data, false otherwise
pub fn is_276_json(contents: &str) -> bool {
    // Check if the JSON contains key indicators of 276 format
    contents.contains("\"st01_transaction_set_identifier_code\":\"276\"") || 
    contents.contains("\"bht06_transaction_type_code\":\"13\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::segments::hl::HL;
    use crate::segments::nm1::NM1;
    use crate::segments::trn::TRN;
    use crate::segments::r#ref::REF;
    use crate::segments::dmg::DMG;
    
    #[test]
    fn test_parse_and_generate_276() {
        // Create a sample EDI 276 file
        let sample_edi = "ISA*00*          *00*          *ZZ*SENDER         *ZZ*RECEIVER       *230501*1200*^*00501*000000001*0*P*:~\
                          GS*HR*SENDER*RECEIVER*20230501*1200*1*X*005010X212~\
                          ST*276*0001*005010X212~\
                          BHT*0010*13*12345*20230501*1200~\
                          HL*1**20*1~\
                          NM1*PR*2*INSURANCE COMPANY*****PI*12345~\
                          HL*2*1*21*1~\
                          NM1*41*2*CLEARINGHOUSE*****46*67890~\
                          HL*3*2*19*1~\
                          NM1*1P*2*PROVIDER NAME*****XX*1234567890~\
                          HL*4*3*22*0~\
                          NM1*IL*1*DOE*JOHN****MI*12345678901~\
                          TRN*1*CLAIM123*9PROVIDER~\
                          REF*BLT*12345~\
                          DMG*D8*19800101*M~\
                          SE*14*0001~\
                          GE*1*1~\
                          IEA*1*000000001~";
        
        // Parse the EDI file
        let edi276_result = get_276(sample_edi.to_string());
        assert!(edi276_result.is_ok(), "Failed to parse EDI 276 file");
        
        let edi276 = edi276_result.unwrap();
        
        // Verify key components
        assert_eq!(edi276.interchange_header.isa01_authorization_info_qualifier, "00");
        assert_eq!(edi276.table1_combined.table1.st01_transaction_set_identifier_code, "276");
        assert_eq!(edi276.table1_combined.table1.bht01_hierarchical_structure_code, "0010");
        assert_eq!(edi276.table1_combined.table1.bht06_transaction_type_code, "13"); // 13 is for Request
        
        // Verify Loop 2000A (Information Source)
        assert_eq!(edi276.loop2000a.hl.hl03_hierarchical_level_code, "20");
        assert_eq!(edi276.loop2000a.nm1.nm101_entity_identifier_code, "PR");
        assert_eq!(edi276.loop2000a.nm1.nm102_entity_type_qualifier, "2");
        assert_eq!(edi276.loop2000a.nm1.nm103_name_last_or_organization_name, "INSURANCE COMPANY");
        
        // Generate EDI from the parsed object
        let generated_edi = write_276(&edi276);
        
        // Verify that the generated EDI contains key segments
        assert!(generated_edi.contains("ISA*00*"), "Missing ISA segment");
        assert!(generated_edi.contains("ST*276*"), "Missing ST segment");
        assert!(generated_edi.contains("BHT*0010*13*"), "Missing BHT segment");
        assert!(generated_edi.contains("HL*1**20*1"), "Missing HL segment for Information Source");
        assert!(generated_edi.contains("NM1*PR*2*INSURANCE COMPANY"), "Missing NM1 segment for Information Source");
        assert!(generated_edi.contains("HL*2*1*21*1"), "Missing HL segment for Information Receiver");
        assert!(generated_edi.contains("NM1*41*2*CLEARINGHOUSE"), "Missing NM1 segment for Information Receiver");
        assert!(generated_edi.contains("TRN*1*CLAIM123*9PROVIDER"), "Missing TRN segment");
        assert!(generated_edi.contains("REF*BLT*12345"), "Missing REF segment");
        assert!(generated_edi.contains("DMG*D8*19800101*M"), "Missing DMG segment");
    }
    
    #[test]
    fn test_loop_structure_276() {
        // Create a sample Loop 2000A
        let mut loop_2000a = Loop2000A::default();
        loop_2000a.hl = HL {
            segment_id: "HL".to_string(),
            hl01_hierarchical_id_number: "1".to_string(),
            hl02_hierarchical_parent_id_number: "".to_string(),
            hl03_hierarchical_level_code: "20".to_string(),
            hl04_hierarchical_child_code: "1".to_string(),
        };
        loop_2000a.nm1 = NM1 {
            segment_id: "NM1".to_string(),
            nm101_entity_identifier_code: "PR".to_string(),
            nm102_entity_type_qualifier: "2".to_string(),
            nm103_name_last_or_organization_name: "INSURANCE COMPANY".to_string(),
            nm104_name_first: "".to_string(),
            nm105_name_middle: "".to_string(),
            nm106_name_prefix: "".to_string(),
            nm107_name_suffix: "".to_string(),
            nm108_identification_code_qualifier: "PI".to_string(),
            nm109_identification_code: "12345".to_string(),
            nm110_entity_relationship_code: "".to_string(),
            nm111_entity_identifier_code: "".to_string(),
            nm112_name_last_or_organization_name: "".to_string(),
        };
        
        // Create a sample Loop 2000D
        let mut loop_2000d = Loop2000D::default();
        loop_2000d.hl = HL {
            segment_id: "HL".to_string(),
            hl01_hierarchical_id_number: "4".to_string(),
            hl02_hierarchical_parent_id_number: "3".to_string(),
            hl03_hierarchical_level_code: "22".to_string(),
            hl04_hierarchical_child_code: "0".to_string(),
        };
        loop_2000d.nm1 = NM1 {
            segment_id: "NM1".to_string(),
            nm101_entity_identifier_code: "IL".to_string(),
            nm102_entity_type_qualifier: "1".to_string(),
            nm103_name_last_or_organization_name: "DOE".to_string(),
            nm104_name_first: "JOHN".to_string(),
            nm105_name_middle: "".to_string(),
            nm106_name_prefix: "".to_string(),
            nm107_name_suffix: "".to_string(),
            nm108_identification_code_qualifier: "MI".to_string(),
            nm109_identification_code: "12345678901".to_string(),
            nm110_entity_relationship_code: "".to_string(),
            nm111_entity_identifier_code: "".to_string(),
            nm112_name_last_or_organization_name: "".to_string(),
        };
        loop_2000d.trn = TRN {
            segment_id: "TRN".to_string(),
            trn01_trace_type_code: "1".to_string(),
            trn02_reference_identification: "CLAIM123".to_string(),
            trn03_originating_company_identifier: "9PROVIDER".to_string(),
            trn04_reference_identification: "".to_string(),
        };
        loop_2000d.ref_segments = vec![REF {
            segment_id: "REF".to_string(),
            ref01_reference_identification_qualifier: "BLT".to_string(),
            ref02_reference_identification: "12345".to_string(),
            ref03_description: "".to_string(),
            ref04_reference_identifier: "".to_string(),
        }];
        
        // Generate EDI from the loop structures
        let loop_2000a_edi = crate::edi276::loop2000::write_loop_2000a(&loop_2000a);
        
        // Verify the generated EDI
        assert!(loop_2000a_edi.contains("HL*1**20*1~"), "Incorrect HL segment for Information Source");
        assert!(loop_2000a_edi.contains("NM1*PR*2*INSURANCE COMPANY"), "Incorrect NM1 segment for Information Source");
    }
    
    #[test]
    fn test_is_276_json() {
        // Test with valid 276 JSON
        let valid_json = r#"{"table1_combined":{"table1":{"st01_transaction_set_identifier_code":"276","bht06_transaction_type_code":"13"}}}"#;
        assert!(is_276_json(valid_json), "Failed to identify valid 276 JSON");
        
        // Test with invalid JSON
        let invalid_json = r#"{"table1_combined":{"table1":{"st01_transaction_set_identifier_code":"277","bht06_transaction_type_code":"28"}}}"#;
        assert!(!is_276_json(invalid_json), "Incorrectly identified invalid JSON as 276");
    }
}
