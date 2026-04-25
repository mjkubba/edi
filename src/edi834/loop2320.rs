use log::info;
use serde::{Deserialize, Serialize};

// COB segment doesn't exist yet, so we'll create a placeholder
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct COB {
    pub cob01_payer_responsibility_sequence_number_code: String,
    pub cob02_reference_identification: String,
    pub cob03_coordination_of_benefits_code: String,
    pub cob04_service_type_code: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2320 {
    pub cob: COB,
}

pub fn get_loop2320(mut contents: String) -> (Loop2320, String) {
    let mut loop2320 = Loop2320::default();

    // For now, just skip COB segments since we don't have the implementation
    if let Some(cob_start) = contents.find("COB*") {
        if let Some(cob_end) = contents[cob_start..].find("~") {
            // Skip this segment for now
            contents = contents[cob_start + cob_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2320: {:?}", loop2320);
    (loop2320, contents)
}

pub fn write_loop2320(loop2320: Loop2320) -> String {
    let mut result = String::new();
    // Skip COB segment for now
    result
}
