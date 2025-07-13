use log::info;
use serde::{Serialize, Deserialize};
use crate::segments::nm1::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100B {
    pub nm1: NM1,
}

pub fn get_loop2100b(mut contents: String) -> (Loop2100B, String) {
    let mut loop2100b = Loop2100B::default();
    
    if let Some(nm1_start) = contents.find("NM1*70*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 3..nm1_start + nm1_end];
            loop2100b.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }
    
    info!("Parsed Loop2100B: {:?}", loop2100b);
    (loop2100b, contents)
}

pub fn write_loop2100b(loop2100b: Loop2100B) -> String {
    let mut result = String::new();
    result.push_str(&write_nm1(loop2100b.nm1));
    result.push_str("\n");
    result
}