use crate::segments::nm1::*;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100G {
    pub nm1: NM1,
}

pub fn get_loop2100g(contents: &str) -> (Loop2100G, String) {
    let mut contents = contents.to_string();
    let mut loop2100g = Loop2100G::default();

    if let Some(nm1_start) = contents.find("NM1*6Y*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 4..nm1_start + nm1_end];
            loop2100g.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2100G: {:?}", loop2100g);
    (loop2100g, contents)
}

pub fn write_loop2100g(loop2100g: Loop2100G) -> String {
    let mut result = String::new();
    result.push_str(&write_nm1(loop2100g.nm1));
    result.push_str("\n");
    result
}
