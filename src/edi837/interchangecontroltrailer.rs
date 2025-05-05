use serde::{Serialize, Deserialize};

/// IEA - Interchange Control Trailer
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub segment_id: String,
    pub number_of_included_functional_groups: String,
    pub interchange_control_number: String,
}

/// GE - Functional Group Trailer
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionalGroupTrailer {
    pub segment_id: String,
    pub number_of_transaction_sets_included: String,
    pub group_control_number: String,
}

/// SE - Transaction Set Trailer
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionSetTrailer {
    pub segment_id: String,
    pub number_of_included_segments: String,
    pub transaction_set_control_number: String,
}

/// Parse IEA segment
pub fn parse_iea(segment: &str) -> Result<InterchangeTrailer, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 3 {
        return Err(format!("Invalid IEA segment: {}", segment));
    }
    
    Ok(InterchangeTrailer {
        segment_id: fields[0].to_string(),
        number_of_included_functional_groups: fields[1].to_string(),
        interchange_control_number: fields[2].to_string(),
    })
}

/// Generate IEA segment
pub fn write_iea(iea: &InterchangeTrailer) -> String {
    format!(
        "{}*{}*{}",
        iea.segment_id,
        iea.number_of_included_functional_groups,
        iea.interchange_control_number
    )
}

/// Parse GE segment
pub fn parse_ge(segment: &str) -> Result<FunctionalGroupTrailer, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 3 {
        return Err(format!("Invalid GE segment: {}", segment));
    }
    
    Ok(FunctionalGroupTrailer {
        segment_id: fields[0].to_string(),
        number_of_transaction_sets_included: fields[1].to_string(),
        group_control_number: fields[2].to_string(),
    })
}

/// Generate GE segment
pub fn write_ge(ge: &FunctionalGroupTrailer) -> String {
    format!(
        "{}*{}*{}",
        ge.segment_id,
        ge.number_of_transaction_sets_included,
        ge.group_control_number
    )
}

/// Parse SE segment
pub fn parse_se(segment: &str) -> Result<TransactionSetTrailer, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 3 {
        return Err(format!("Invalid SE segment: {}", segment));
    }
    
    Ok(TransactionSetTrailer {
        segment_id: fields[0].to_string(),
        number_of_included_segments: fields[1].to_string(),
        transaction_set_control_number: fields[2].to_string(),
    })
}

/// Generate SE segment
pub fn write_se(se: &TransactionSetTrailer) -> String {
    format!(
        "{}*{}*{}",
        se.segment_id,
        se.number_of_included_segments,
        se.transaction_set_control_number
    )
}
