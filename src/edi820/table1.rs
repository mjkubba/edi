use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1s {
    pub st_segments: ST,
    pub bpr_segments: BPR,
    pub trn_segments: Option<TRN>,
    pub ref_segments: Vec<REF>,
    pub dtm_segments: Vec<DTM>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ST {
    pub transaction_set_id: String,
    pub transaction_set_control_number: String,
    pub implementation_conven_ref: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BPR {
    pub bpr01_transaction_handling_code: String,
    pub bpr02_total_payment_amount: String,
    pub bpr03_credit_debit_flag_code: String,
    pub bpr04_payment_method_code: String,
    pub bpr05_payment_format_code: String,
    pub bpr06_dfi_id_number_qualifier: String,
    pub bpr07_dfi_id_number: String,
    pub bpr08_account_number_qualifier: String,
    pub bpr09_account_number: String,
    pub bpr10_originating_company_id: String,
    pub bpr11_originating_company_supplemental_code: String,
    pub bpr12_dfi_id_number_qualifier: String,
    pub bpr13_dfi_id_number: String,
    pub bpr14_account_number_qualifier: String,
    pub bpr15_account_number: String,
    pub bpr16_payment_effective_date: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TRN {
    pub trn01_trace_type_code: String,
    pub trn02_reference_id: String,
    pub trn03_originating_company_id: String,
    pub trn04_reference_id: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct REF {
    pub ref01_reference_id_qualifier: String,
    pub ref02_reference_id: String,
    pub ref03_description: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTM {
    pub dtm01_date_time_qualifier: String,
    pub dtm02_date: String,
    pub dtm03_time: String,
    pub dtm04_time_code: String,
    pub dtm05_date_time_period_format_qualifier: String,
    pub dtm06_date_time_period: String,
}

pub fn get_table1s(mut contents: String) -> (Table1s, String) {
    let mut table1s = Table1s::default();
    
    // Parse ST segment
    if contents.contains("ST") {
        info!("ST segment found, ");
        let st_content = get_segment_contents("ST", &contents);
        info!("segment_content: {}", st_content);
        
        let st_parts: Vec<&str> = st_content.split('*').collect();
        
        if st_parts.len() >= 3 {
            table1s.st_segments = ST {
                transaction_set_id: st_parts[1].to_string(),
                transaction_set_control_number: st_parts[2].to_string(),
                implementation_conven_ref: if st_parts.len() > 3 { st_parts[3].to_string() } else { String::new() },
            };
        }
        
        info!("ST segment parsed");
        contents = content_trim("ST", contents);
    }
    
    // Parse BPR segment
    if contents.contains("BPR") {
        info!("BPR segment found, ");
        let bpr_content = get_segment_contents("BPR", &contents);
        info!("segment_content: {}", bpr_content);
        
        let bpr_parts: Vec<&str> = bpr_content.split('*').collect();
        
        if bpr_parts.len() >= 1 {
            table1s.bpr_segments = BPR {
                bpr01_transaction_handling_code: if bpr_parts.len() > 1 { bpr_parts[1].to_string() } else { String::new() },
                bpr02_total_payment_amount: if bpr_parts.len() > 2 { bpr_parts[2].to_string() } else { String::new() },
                bpr03_credit_debit_flag_code: if bpr_parts.len() > 3 { bpr_parts[3].to_string() } else { String::new() },
                bpr04_payment_method_code: if bpr_parts.len() > 4 { bpr_parts[4].to_string() } else { String::new() },
                bpr05_payment_format_code: if bpr_parts.len() > 5 { bpr_parts[5].to_string() } else { String::new() },
                bpr06_dfi_id_number_qualifier: if bpr_parts.len() > 6 { bpr_parts[6].to_string() } else { String::new() },
                bpr07_dfi_id_number: if bpr_parts.len() > 7 { bpr_parts[7].to_string() } else { String::new() },
                bpr08_account_number_qualifier: if bpr_parts.len() > 8 { bpr_parts[8].to_string() } else { String::new() },
                bpr09_account_number: if bpr_parts.len() > 9 { bpr_parts[9].to_string() } else { String::new() },
                bpr10_originating_company_id: if bpr_parts.len() > 10 { bpr_parts[10].to_string() } else { String::new() },
                bpr11_originating_company_supplemental_code: if bpr_parts.len() > 11 { bpr_parts[11].to_string() } else { String::new() },
                bpr12_dfi_id_number_qualifier: if bpr_parts.len() > 12 { bpr_parts[12].to_string() } else { String::new() },
                bpr13_dfi_id_number: if bpr_parts.len() > 13 { bpr_parts[13].to_string() } else { String::new() },
                bpr14_account_number_qualifier: if bpr_parts.len() > 14 { bpr_parts[14].to_string() } else { String::new() },
                bpr15_account_number: if bpr_parts.len() > 15 { bpr_parts[15].to_string() } else { String::new() },
                bpr16_payment_effective_date: if bpr_parts.len() > 16 { bpr_parts[16].to_string() } else { String::new() },
            };
        }
        
        info!("BPR segment parsed");
        contents = content_trim("BPR", contents);
    }
    
    // Parse TRN segment
    if contents.contains("TRN") {
        info!("TRN segment found, ");
        let trn_content = get_segment_contents("TRN", &contents);
        info!("segment_content: {}", trn_content);
        
        let trn_parts: Vec<&str> = trn_content.split('*').collect();
        
        if trn_parts.len() >= 3 {
            table1s.trn_segments = Some(TRN {
                trn01_trace_type_code: trn_parts[1].to_string(),
                trn02_reference_id: trn_parts[2].to_string(),
                trn03_originating_company_id: if trn_parts.len() > 3 { trn_parts[3].to_string() } else { String::new() },
                trn04_reference_id: if trn_parts.len() > 4 { trn_parts[4].to_string() } else { String::new() },
            });
        }
        
        info!("TRN segment parsed");
        contents = content_trim("TRN", contents);
    }
    
    // Parse REF segments
    while contents.contains("REF") && check_if_segement_in_loop("REF", "DTM", contents.clone()) && 
          check_if_segement_in_loop("REF", "N1", contents.clone()) {
        info!("REF segment found, ");
        let ref_content = get_segment_contents("REF", &contents);
        
        let ref_parts: Vec<&str> = ref_content.split('*').collect();
        
        if ref_parts.len() >= 2 {
            let ref_segment = REF {
                ref01_reference_id_qualifier: ref_parts[1].to_string(),
                ref02_reference_id: if ref_parts.len() > 2 { ref_parts[2].to_string() } else { String::new() },
                ref03_description: if ref_parts.len() > 3 { ref_parts[3].to_string() } else { String::new() },
            };
            
            table1s.ref_segments.push(ref_segment);
        }
        
        info!("REF segment parsed");
        contents = content_trim("REF", contents);
    }
    
    // Parse DTM segments
    while contents.contains("DTM") && check_if_segement_in_loop("DTM", "N1", contents.clone()) {
        info!("DTM segment found, ");
        let dtm_content = get_segment_contents("DTM", &contents);
        
        let dtm_parts: Vec<&str> = dtm_content.split('*').collect();
        
        if dtm_parts.len() >= 2 {
            let dtm_segment = DTM {
                dtm01_date_time_qualifier: dtm_parts[1].to_string(),
                dtm02_date: if dtm_parts.len() > 2 { dtm_parts[2].to_string() } else { String::new() },
                dtm03_time: if dtm_parts.len() > 3 { dtm_parts[3].to_string() } else { String::new() },
                dtm04_time_code: if dtm_parts.len() > 4 { dtm_parts[4].to_string() } else { String::new() },
                dtm05_date_time_period_format_qualifier: if dtm_parts.len() > 5 { dtm_parts[5].to_string() } else { String::new() },
                dtm06_date_time_period: if dtm_parts.len() > 6 { dtm_parts[6].to_string() } else { String::new() },
            };
            
            table1s.dtm_segments.push(dtm_segment);
        }
        
        info!("DTM segment parsed");
        contents = content_trim("DTM", contents);
    }
    
    info!("Table 1 parsed\n");
    
    return (table1s, contents);
}

pub fn write_table1(table1s: Table1s) -> String {
    let mut result = String::new();
    
    // Write ST segment
    result.push_str("ST*");
    result.push_str(&table1s.st_segments.transaction_set_id);
    result.push_str("*");
    result.push_str(&table1s.st_segments.transaction_set_control_number);
    if !table1s.st_segments.implementation_conven_ref.is_empty() {
        result.push_str("*");
        result.push_str(&table1s.st_segments.implementation_conven_ref);
    }
    result.push_str("~\n");
    
    // Write BPR segment
    result.push_str("BPR*");
    result.push_str(&table1s.bpr_segments.bpr01_transaction_handling_code);
    result.push_str("*");
    result.push_str(&table1s.bpr_segments.bpr02_total_payment_amount);
    result.push_str("*");
    result.push_str(&table1s.bpr_segments.bpr03_credit_debit_flag_code);
    result.push_str("*");
    result.push_str(&table1s.bpr_segments.bpr04_payment_method_code);
    result.push_str("*");
    result.push_str(&table1s.bpr_segments.bpr05_payment_format_code);
    
    // Add remaining BPR fields if they exist
    if !table1s.bpr_segments.bpr06_dfi_id_number_qualifier.is_empty() ||
       !table1s.bpr_segments.bpr07_dfi_id_number.is_empty() ||
       !table1s.bpr_segments.bpr08_account_number_qualifier.is_empty() ||
       !table1s.bpr_segments.bpr09_account_number.is_empty() ||
       !table1s.bpr_segments.bpr10_originating_company_id.is_empty() ||
       !table1s.bpr_segments.bpr11_originating_company_supplemental_code.is_empty() ||
       !table1s.bpr_segments.bpr12_dfi_id_number_qualifier.is_empty() ||
       !table1s.bpr_segments.bpr13_dfi_id_number.is_empty() ||
       !table1s.bpr_segments.bpr14_account_number_qualifier.is_empty() ||
       !table1s.bpr_segments.bpr15_account_number.is_empty() ||
       !table1s.bpr_segments.bpr16_payment_effective_date.is_empty() {
        
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr06_dfi_id_number_qualifier);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr07_dfi_id_number);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr08_account_number_qualifier);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr09_account_number);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr10_originating_company_id);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr11_originating_company_supplemental_code);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr12_dfi_id_number_qualifier);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr13_dfi_id_number);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr14_account_number_qualifier);
        result.push_str("*");
        result.push_str(&table1s.bpr_segments.bpr15_account_number);
        
        if !table1s.bpr_segments.bpr16_payment_effective_date.is_empty() {
            result.push_str("*");
            result.push_str(&table1s.bpr_segments.bpr16_payment_effective_date);
        }
    }
    
    result.push_str("~\n");
    
    // Write TRN segment if it exists
    if let Some(trn) = &table1s.trn_segments {
        result.push_str("TRN*");
        result.push_str(&trn.trn01_trace_type_code);
        result.push_str("*");
        result.push_str(&trn.trn02_reference_id);
        
        if !trn.trn03_originating_company_id.is_empty() {
            result.push_str("*");
            result.push_str(&trn.trn03_originating_company_id);
            
            if !trn.trn04_reference_id.is_empty() {
                result.push_str("*");
                result.push_str(&trn.trn04_reference_id);
            }
        }
        
        result.push_str("~\n");
    }
    
    // Write REF segments
    for ref_segment in &table1s.ref_segments {
        result.push_str("REF*");
        result.push_str(&ref_segment.ref01_reference_id_qualifier);
        result.push_str("*");
        result.push_str(&ref_segment.ref02_reference_id);
        
        if !ref_segment.ref03_description.is_empty() {
            result.push_str("*");
            result.push_str(&ref_segment.ref03_description);
        }
        
        result.push_str("~\n");
    }
    
    // Write DTM segments
    for dtm_segment in &table1s.dtm_segments {
        result.push_str("DTM*");
        result.push_str(&dtm_segment.dtm01_date_time_qualifier);
        
        if !dtm_segment.dtm02_date.is_empty() {
            result.push_str("*");
            result.push_str(&dtm_segment.dtm02_date);
            
            if !dtm_segment.dtm03_time.is_empty() {
                result.push_str("*");
                result.push_str(&dtm_segment.dtm03_time);
                
                if !dtm_segment.dtm04_time_code.is_empty() {
                    result.push_str("*");
                    result.push_str(&dtm_segment.dtm04_time_code);
                    
                    if !dtm_segment.dtm05_date_time_period_format_qualifier.is_empty() {
                        result.push_str("*");
                        result.push_str(&dtm_segment.dtm05_date_time_period_format_qualifier);
                        
                        if !dtm_segment.dtm06_date_time_period.is_empty() {
                            result.push_str("*");
                            result.push_str(&dtm_segment.dtm06_date_time_period);
                        }
                    }
                }
            }
        }
        
        result.push_str("~\n");
    }
    
    return result;
}
