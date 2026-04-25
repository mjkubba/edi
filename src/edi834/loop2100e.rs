use crate::segments::nm1::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100E {
    pub nm1: NM1,
}

pub fn get_loop2100e(mut contents: String) -> (Loop2100E, String) {
    let mut loop2100e = Loop2100E::default();

    if let Some(nm1_start) = contents.find("NM1*M8*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 3..nm1_start + nm1_end];
            loop2100e.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2100E: {:?}", loop2100e);
    (loop2100e, contents)
}

pub fn write_loop2100e(loop2100e: Loop2100E) -> String {
    let mut result = String::new();
    result.push_str(&write_nm1(loop2100e.nm1));
    result.push_str("\n");
    result
}
