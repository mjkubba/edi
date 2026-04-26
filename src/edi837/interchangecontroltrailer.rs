use serde::{Deserialize, Serialize};

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
