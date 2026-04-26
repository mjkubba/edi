use serde::{Deserialize, Serialize};

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
