use serde::{Serialize, Deserialize};

use crate::edi837::interchangecontrol::*;

/// Table1s structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1s {
    pub functional_group_header: FunctionalGroupHeader,
    pub transaction_set_header: TransactionSetHeader,
    pub bht: BHT,
}

/// Parse GS segment
pub fn parse_gs(segment: &str) -> Result<FunctionalGroupHeader, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 9 {
        return Err(format!("Invalid GS segment: {}", segment));
    }
    
    Ok(FunctionalGroupHeader {
        segment_id: fields[0].to_string(),
        functional_identifier_code: fields[1].to_string(),
        application_sender_code: fields[2].to_string(),
        application_receiver_code: fields[3].to_string(),
        date: fields[4].to_string(),
        time: fields[5].to_string(),
        group_control_number: fields[6].to_string(),
        responsible_agency_code: fields[7].to_string(),
        version_release_industry_identifier_code: fields[8].to_string(),
    })
}

/// Generate GS segment
pub fn write_gs(gs: &FunctionalGroupHeader) -> String {
    format!(
        "{}*{}*{}*{}*{}*{}*{}*{}*{}",
        gs.segment_id,
        gs.functional_identifier_code,
        gs.application_sender_code,
        gs.application_receiver_code,
        gs.date,
        gs.time,
        gs.group_control_number,
        gs.responsible_agency_code,
        gs.version_release_industry_identifier_code
    )
}

/// Parse ST segment
pub fn parse_st(segment: &str) -> Result<TransactionSetHeader, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 3 {
        return Err(format!("Invalid ST segment: {}", segment));
    }
    
    let implementation_convention_reference = if fields.len() > 3 {
        Some(fields[3].to_string())
    } else {
        None
    };
    
    Ok(TransactionSetHeader {
        segment_id: fields[0].to_string(),
        transaction_set_identifier_code: fields[1].to_string(),
        transaction_set_control_number: fields[2].to_string(),
        implementation_convention_reference,
    })
}

/// Generate ST segment
pub fn write_st(st: &TransactionSetHeader) -> String {
    if let Some(ref implementation_convention_reference) = st.implementation_convention_reference {
        format!(
            "{}*{}*{}*{}",
            st.segment_id,
            st.transaction_set_identifier_code,
            st.transaction_set_control_number,
            implementation_convention_reference
        )
    } else {
        format!(
            "{}*{}*{}",
            st.segment_id,
            st.transaction_set_identifier_code,
            st.transaction_set_control_number
        )
    }
}

/// Parse BHT segment
pub fn parse_bht(segment: &str) -> Result<BHT, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 7 {
        return Err(format!("Invalid BHT segment: {}", segment));
    }
    
    let time = if !fields[5].is_empty() {
        Some(fields[5].to_string())
    } else {
        None
    };
    
    Ok(BHT {
        segment_id: fields[0].to_string(),
        hierarchical_structure_code: fields[1].to_string(),
        transaction_set_purpose_code: fields[2].to_string(),
        reference_identification: fields[3].to_string(),
        date: fields[4].to_string(),
        time,
        transaction_type_code: fields[6].to_string(),
    })
}

/// Generate BHT segment
pub fn write_bht(bht: &BHT) -> String {
    if let Some(ref time) = bht.time {
        format!(
            "{}*{}*{}*{}*{}*{}*{}",
            bht.segment_id,
            bht.hierarchical_structure_code,
            bht.transaction_set_purpose_code,
            bht.reference_identification,
            bht.date,
            time,
            bht.transaction_type_code
        )
    } else {
        format!(
            "{}*{}*{}*{}*{}**{}",
            bht.segment_id,
            bht.hierarchical_structure_code,
            bht.transaction_set_purpose_code,
            bht.reference_identification,
            bht.date,
            bht.transaction_type_code
        )
    }
}
