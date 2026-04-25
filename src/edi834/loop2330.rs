use log::info;
use serde::{Deserialize, Serialize};

use crate::segments::dsb::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2330 {
    pub dsb: DSB,
}

pub fn get_loop2330(mut contents: String) -> (Loop2330, String) {
    let mut loop2330 = Loop2330::default();

    // Parse DSB segment (required)
    if let Some(dsb_start) = contents.find("DSB*") {
        if let Some(dsb_end) = contents[dsb_start..].find("~") {
            let dsb_content = &contents[dsb_start + 4..dsb_start + dsb_end];
            loop2330.dsb = get_dsb(dsb_content.to_string());
            contents = contents[dsb_start + dsb_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2330: {:?}", loop2330);
    (loop2330, contents)
}

pub fn write_loop2330(loop2330: Loop2330) -> String {
    let mut result = String::new();

    result.push_str(&write_dsb(loop2330.dsb));
    result.push_str("\n");

    result
}
