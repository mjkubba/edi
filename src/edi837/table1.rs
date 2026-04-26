use serde::{Deserialize, Serialize};

use crate::edi837::interchangecontrol::*;

/// Table1s structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1s {
    pub functional_group_header: FunctionalGroupHeader,
    pub transaction_set_header: TransactionSetHeader,
    pub bht: String,
}
