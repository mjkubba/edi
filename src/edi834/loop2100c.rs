use crate::segments::nm1::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100C {
    pub nm1: NM1,
}

pub fn get_loop2100c(mut contents: String) -> (Loop2100C, String) {
    let mut loop2100c = Loop2100C::default();

    if let Some(nm1_start) = contents.find("NM1*31*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 4..nm1_start + nm1_end];
            loop2100c.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2100C: {:?}", loop2100c);
    (loop2100c, contents)
}

pub fn write_loop2100c(loop2100c: Loop2100C) -> String {
    let mut result = String::new();
    result.push_str(&write_nm1(loop2100c.nm1));
    result.push_str("\n");
    result
}
