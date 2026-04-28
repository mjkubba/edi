use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TA1 {
    pub ta01_interchange_control_number: String,
    pub ta02_interchange_date: String,
    pub ta03_interchange_time: String,
    pub ta04_interchange_ack_code: String,
    pub ta05_interchange_note_code: String,
}
