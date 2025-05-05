use serde::{Serialize, Deserialize};

/// ISA - Interchange Control Header
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub segment_id: String,
    pub authorization_information_qualifier: String,
    pub authorization_information: String,
    pub security_information_qualifier: String,
    pub security_information: String,
    pub interchange_id_qualifier: String,
    pub interchange_sender_id: String,
    pub interchange_id_qualifier_2: String,
    pub interchange_receiver_id: String,
    pub interchange_date: String,
    pub interchange_time: String,
    pub repetition_separator: String,
    pub interchange_control_version_number: String,
    pub interchange_control_number: String,
    pub acknowledgment_requested: String,
    pub usage_indicator: String,
    pub component_element_separator: String,
}

/// GS - Functional Group Header
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FunctionalGroupHeader {
    pub segment_id: String,
    pub functional_identifier_code: String,
    pub application_sender_code: String,
    pub application_receiver_code: String,
    pub date: String,
    pub time: String,
    pub group_control_number: String,
    pub responsible_agency_code: String,
    pub version_release_industry_identifier_code: String,
}

/// ST - Transaction Set Header
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionSetHeader {
    pub segment_id: String,
    pub transaction_set_identifier_code: String,
    pub transaction_set_control_number: String,
    pub implementation_convention_reference: Option<String>,
}

/// BHT - Beginning of Hierarchical Transaction
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BHT {
    pub segment_id: String,
    pub hierarchical_structure_code: String,
    pub transaction_set_purpose_code: String,
    pub reference_identification: String,
    pub date: String,
    pub time: Option<String>,
    pub transaction_type_code: String,
}

/// Parse ISA segment
pub fn parse_isa(segment: &str) -> Result<InterchangeHeader, String> {
    let fields: Vec<&str> = segment.split('*').collect();
    
    if fields.len() < 17 {
        return Err(format!("Invalid ISA segment: {}", segment));
    }
    
    Ok(InterchangeHeader {
        segment_id: fields[0].to_string(),
        authorization_information_qualifier: fields[1].to_string(),
        authorization_information: fields[2].to_string(),
        security_information_qualifier: fields[3].to_string(),
        security_information: fields[4].to_string(),
        interchange_id_qualifier: fields[5].to_string(),
        interchange_sender_id: fields[6].to_string(),
        interchange_id_qualifier_2: fields[7].to_string(),
        interchange_receiver_id: fields[8].to_string(),
        interchange_date: fields[9].to_string(),
        interchange_time: fields[10].to_string(),
        repetition_separator: fields[11].to_string(),
        interchange_control_version_number: fields[12].to_string(),
        interchange_control_number: fields[13].to_string(),
        acknowledgment_requested: fields[14].to_string(),
        usage_indicator: fields[15].to_string(),
        component_element_separator: fields[16].to_string(),
    })
}

/// Generate ISA segment
pub fn write_isa(isa: &InterchangeHeader) -> String {
    format!(
        "{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}",
        isa.segment_id,
        isa.authorization_information_qualifier,
        isa.authorization_information,
        isa.security_information_qualifier,
        isa.security_information,
        isa.interchange_id_qualifier,
        isa.interchange_sender_id,
        isa.interchange_id_qualifier_2,
        isa.interchange_receiver_id,
        isa.interchange_date,
        isa.interchange_time,
        isa.repetition_separator,
        isa.interchange_control_version_number,
        isa.interchange_control_number,
        isa.acknowledgment_requested,
        isa.usage_indicator,
        isa.component_element_separator
    )
}
