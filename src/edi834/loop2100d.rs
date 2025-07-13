use log::info;
use serde::{Serialize, Deserialize};
use crate::segments::nm1::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100D {
    pub nm1: NM1,
}

pub fn get_loop2100d(mut contents: String) -> (Loop2100D, String) {
    let mut loop2100d = Loop2100D::default();
    
    if let Some(nm1_start) = contents.find("NM1*36*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 3..nm1_start + nm1_end];
            loop2100d.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }
    
    info!("Parsed Loop2100D: {:?}", loop2100d);
    (loop2100d, contents)
}

pub fn write_loop2100d(loop2100d: Loop2100D) -> String {
    let mut result = String::new();
    result.push_str(&write_nm1(loop2100d.nm1));
    result.push_str("\n");
    result
}