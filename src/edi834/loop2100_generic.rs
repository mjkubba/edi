use crate::segments::nm1::*;
use log::info;
use serde::{Deserialize, Serialize};

/// Generic NM1-only loop used by 834 Loop 2100B through 2100H.
/// Each variant differs only by the NM1 entity identifier code.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100Generic {
    pub nm1: NM1,
}

/// Parse a generic Loop 2100 by looking for an NM1 segment with the given entity code.
pub fn get_loop2100_generic(contents: &str, entity_code: &str) -> (Loop2100Generic, String) {
    let mut contents = contents.to_string();
    let mut loop_data = Loop2100Generic::default();
    let pattern = format!("NM1*{}*", entity_code);

    if let Some(nm1_start) = contents.find(&pattern) {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 4..nm1_start + nm1_end];
            loop_data.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    info!("Parsed Loop2100 (NM1*{}): {:?}", entity_code, loop_data);
    (loop_data, contents)
}

pub fn write_loop2100_generic(loop_data: &Loop2100Generic) -> String {
    write_nm1(loop_data.nm1.clone())
}

// Type aliases to preserve JSON field naming compatibility
pub type Loop2100B = Loop2100Generic;
pub type Loop2100C = Loop2100Generic;
pub type Loop2100D = Loop2100Generic;
pub type Loop2100E = Loop2100Generic;
pub type Loop2100F = Loop2100Generic;
pub type Loop2100G = Loop2100Generic;
pub type Loop2100H = Loop2100Generic;

// Convenience functions to preserve existing call signatures
pub fn get_loop2100b(contents: &str) -> (Loop2100B, String) {
    get_loop2100_generic(contents, "70")
}
pub fn get_loop2100c(contents: &str) -> (Loop2100C, String) {
    get_loop2100_generic(contents, "31")
}
pub fn get_loop2100d(contents: &str) -> (Loop2100D, String) {
    get_loop2100_generic(contents, "36")
}
pub fn get_loop2100e(contents: &str) -> (Loop2100E, String) {
    get_loop2100_generic(contents, "M8")
}
pub fn get_loop2100f(contents: &str) -> (Loop2100F, String) {
    get_loop2100_generic(contents, "S1")
}
pub fn get_loop2100g(contents: &str) -> (Loop2100G, String) {
    get_loop2100_generic(contents, "6Y")
}
pub fn get_loop2100h(contents: &str) -> (Loop2100H, String) {
    get_loop2100_generic(contents, "9K")
}

pub fn write_loop2100b(l: Loop2100B) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100c(l: Loop2100C) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100d(l: Loop2100D) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100e(l: Loop2100E) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100f(l: Loop2100F) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100g(l: Loop2100G) -> String {
    write_loop2100_generic(&l)
}
pub fn write_loop2100h(l: Loop2100H) -> String {
    write_loop2100_generic(&l)
}
