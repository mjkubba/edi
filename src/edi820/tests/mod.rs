#[cfg(test)]
mod tests {
    use crate::edi820::loop2000::*;
    use crate::edi820::loop2100::*;
    use crate::edi820::controller::*;
    
    #[test]
    fn test_edi820_parsing() {
        // Sample EDI820 content
        let content = "ISA*00*          *00*          *ZZ*123456789012345*ZZ*123456789012346*130503*1705*>*00501*000010216*0*T*:~\
                      GS*RA*1234567890*9876543210*20130503*1705*20213*X*005010X306~\
                      ST*820*0001*005010X306~\
                      BPR*I*220*C*ACH*CCP*******01*199999999*DA*98765*20140604~\
                      TRN*3*78905~\
                      REF*38*123456~\
                      REF*TV*12565496~\
                      N1*PE*BATA INSURANCE CO.*FI*012222222~\
                      N1*RM*GOVERNMENT AGENCY*58*123ABC~\
                      ENT*1~\
                      NM1*IL*1*DOE*JOHN****C1*777222~\
                      REF*POL*1232456~\
                      REF*AZ*12565496~\
                      REF*0F*555666~\
                      RMR*ZZ*APTC**35~\
                      DTM*582****RD8*20120501-20140531~\
                      ENT*2~\
                      NM1*IL*1*FIRSTONE*EMILY****C1*777333~\
                      REF*POL*1232457~\
                      REF*AZ*12565497~\
                      REF*0F*555777~\
                      RMR*ZZ*APTC**35~\
                      DTM*582****RD8*20120501-20140531~\
                      SE*25*0001~\
                      GE*1*20213~\
                      IEA*1*000010216~";
        
        // Test parsing
        match Edi820::parse(content.to_string()) {
            Ok((edi820, remaining)) => {
                // Verify that parsing was successful
                assert_eq!(edi820.transaction_set_id, "820");
                
                // Check interchange header
                assert_eq!(edi820.interchange_header.isa_segments.sender_id, "123456789012345");
                assert_eq!(edi820.interchange_header.isa_segments.receiver_id, "123456789012346");
                
                // Check table1 (BPR segment)
                assert_eq!(edi820.table1_combined.table1.bpr_segments.bpr01_transaction_handling_code, "I");
                assert_eq!(edi820.table1_combined.table1.bpr_segments.bpr02_total_payment_amount, "220");
                
                // Check loop1000a (Payer)
                assert_eq!(edi820.table1_combined.loop1000as.n1_segments.entity_id, "PE");
                assert_eq!(edi820.table1_combined.loop1000as.n1_segments.entity_name, "BATA INSURANCE CO.");
                
                // Check loop1000b (Payee)
                assert_eq!(edi820.table1_combined.loop1000bs.n1_segments.entity_id, "RM");
                assert_eq!(edi820.table1_combined.loop1000bs.n1_segments.entity_name, "GOVERNMENT AGENCY");
                
                // Check table2s (ENT segments)
                assert!(!edi820.table2s.is_empty());
                assert_eq!(edi820.table2s[0].ent_segments.ent01_assigned_number, "1");
                
                // Check loop2100s (NM1 segments)
                assert!(!edi820.table2s[0].loop2100s.is_empty());
                assert_eq!(edi820.table2s[0].loop2100s[0].nm1_segments.entity_id, "IL");
                assert_eq!(edi820.table2s[0].loop2100s[0].nm1_segments.lastname, "DOE");
                assert_eq!(edi820.table2s[0].loop2100s[0].nm1_segments.firstname, "JOHN");
                
                // Check REF segments
                assert!(!edi820.table2s[0].loop2100s[0].ref_segments.is_empty());
                assert_eq!(edi820.table2s[0].loop2100s[0].ref_segments[0].reference_id_qualifier, "POL");
                
                // Check RMR segments
                assert!(!edi820.table2s[0].loop2100s[0].rmr_segments.is_empty());
                assert_eq!(edi820.table2s[0].loop2100s[0].rmr_segments[0].rmr01_reference_id_qualifier, "ZZ");
                assert_eq!(edi820.table2s[0].loop2100s[0].rmr_segments[0].rmr02_reference_id, "APTC");
                assert_eq!(edi820.table2s[0].loop2100s[0].rmr_segments[0].rmr04_monetary_amount, "35");
                
                // Check DTM segments
                assert!(!edi820.table2s[0].loop2100s[0].dtm_segments.is_empty());
                assert_eq!(edi820.table2s[0].loop2100s[0].dtm_segments[0].dtm01_date_time_qualifier, "582");
                assert_eq!(edi820.table2s[0].loop2100s[0].dtm_segments[0].dtm05_date_time_period_format_qualifier, "RD8");
                assert_eq!(edi820.table2s[0].loop2100s[0].dtm_segments[0].dtm06_date_time_period, "20120501-20140531");
                
                // Check that there's no remaining content
                assert!(remaining.trim().is_empty(), "Expected no remaining content, got: '{}'", remaining);
            },
            Err(e) => {
                panic!("Failed to parse EDI820: {:?}", e);
            }
        }
    }
    
    #[test]
    fn test_edi820_generation() {
        // Create a minimal EDI820 structure
        let mut edi820 = Edi820::default();
        edi820.transaction_set_id = "820".to_string();
        
        // Set up interchange header
        edi820.interchange_header.isa_segments.sender_id = "123456789012345".to_string();
        edi820.interchange_header.isa_segments.receiver_id = "123456789012346".to_string();
        edi820.interchange_header.isa_segments.date = "130503".to_string();
        edi820.interchange_header.isa_segments.time = "1705".to_string();
        edi820.interchange_header.gs_segments.functional_id_code = "RA".to_string();
        
        // Set up table1
        edi820.table1_combined.table1.st_segments.transaction_set_id = "820".to_string();
        edi820.table1_combined.table1.st_segments.transaction_set_control_number = "0001".to_string();
        edi820.table1_combined.table1.bpr_segments.bpr01_transaction_handling_code = "I".to_string();
        edi820.table1_combined.table1.bpr_segments.bpr02_total_payment_amount = "220".to_string();
        
        // Set up loop1000a (Payer)
        edi820.table1_combined.loop1000as.n1_segments.entity_id = "PE".to_string();
        edi820.table1_combined.loop1000as.n1_segments.entity_name = "BATA INSURANCE CO.".to_string();
        
        // Set up loop1000b (Payee)
        edi820.table1_combined.loop1000bs.n1_segments.entity_id = "RM".to_string();
        edi820.table1_combined.loop1000bs.n1_segments.entity_name = "GOVERNMENT AGENCY".to_string();
        
        // Set up table2 (ENT segment)
        let mut table2 = Table2::default();
        table2.ent_segments.ent01_assigned_number = "1".to_string();
        
        // Set up loop2100 (NM1 segment)
        let mut loop2100 = Loop2100::default();
        loop2100.nm1_segments.entity_id = "IL".to_string();
        loop2100.nm1_segments.entity_type = "1".to_string();
        loop2100.nm1_segments.lastname = "DOE".to_string();
        loop2100.nm1_segments.firstname = "JOHN".to_string();
        
        // Add REF segment
        let ref_segment = REF {
            reference_id_qualifier: "POL".to_string(),
            reference_id: "1232456".to_string(),
            description: "".to_string(),
        };
        loop2100.ref_segments.push(ref_segment);
        
        // Add RMR segment
        let rmr_segment = RMR {
            rmr01_reference_id_qualifier: "ZZ".to_string(),
            rmr02_reference_id: "APTC".to_string(),
            rmr03_payment_action_code: "".to_string(),
            rmr04_monetary_amount: "35".to_string(),
            rmr05_credit_debit_flag_code: "".to_string(),
        };
        loop2100.rmr_segments.push(rmr_segment);
        
        // Add DTM segment
        let dtm_segment = DTM {
            dtm01_date_time_qualifier: "582".to_string(),
            dtm02_date: "".to_string(),
            dtm03_time: "".to_string(),
            dtm04_time_code: "".to_string(),
            dtm05_date_time_period_format_qualifier: "RD8".to_string(),
            dtm06_date_time_period: "20120501-20140531".to_string(),
        };
        loop2100.dtm_segments.push(dtm_segment);
        
        // Add loop2100 to table2
        table2.loop2100s.push(loop2100);
        
        // Add table2 to edi820
        edi820.table2s.push(table2);
        
        // Set up interchange trailer
        edi820.interchange_trailer.se_segments.number_of_segment = "25".to_string();
        edi820.interchange_trailer.se_segments.transaction_set_control_number = "0001".to_string();
        edi820.interchange_trailer.ge_segments.number_of_transitions = "1".to_string();
        edi820.interchange_trailer.ge_segments.group_control_number = "20213".to_string();
        edi820.interchange_trailer.iea_segments.number_of_included_group = "1".to_string();
        edi820.interchange_trailer.iea_segments.interchange_control_number = "000010216".to_string();
        
        // Generate EDI
        let edi_output = edi820.to_edi();
        
        // Verify output contains expected segments
        assert!(edi_output.contains("ISA*"), "Missing ISA segment");
        assert!(edi_output.contains("GS*RA*"), "Missing GS segment");
        assert!(edi_output.contains("ST*820*0001"), "Missing ST segment");
        assert!(edi_output.contains("BPR*I*220"), "Missing BPR segment");
        assert!(edi_output.contains("N1*PE*BATA INSURANCE CO."), "Missing N1 segment for payer");
        assert!(edi_output.contains("N1*RM*GOVERNMENT AGENCY"), "Missing N1 segment for payee");
        assert!(edi_output.contains("ENT*1"), "Missing ENT segment");
        assert!(edi_output.contains("NM1*IL*1*DOE*JOHN"), "Missing NM1 segment");
        assert!(edi_output.contains("REF*POL*1232456"), "Missing REF segment");
        assert!(edi_output.contains("RMR*ZZ*APTC**35"), "Missing RMR segment");
        assert!(edi_output.contains("DTM*582****RD8*20120501-20140531"), "Missing DTM segment");
        assert!(edi_output.contains("SE*25*0001"), "Missing SE segment");
        assert!(edi_output.contains("GE*1*20213"), "Missing GE segment");
        assert!(edi_output.contains("IEA*1*000010216"), "Missing IEA segment");
    }
}
