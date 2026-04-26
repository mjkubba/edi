use serde::{Deserialize, Serialize};

use crate::segments::n4::*;
use crate::segments::nm1::*;

/// Loop 2330 - Coordination of Benefits Other Insurance Company
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2330 {
    pub nm1: NM1,
    pub n3: Option<String>,
    pub n4: Option<N4>,
}

pub fn get_loop2330(mut contents: String) -> (Loop2330, String) {
    let mut loop2330 = Loop2330::default();

    // Parse NM1 segment
    if let Some(nm1_start) = contents.find("NM1*") {
        if let Some(nm1_end) = contents[nm1_start..].find("~") {
            let nm1_content = &contents[nm1_start + 4..nm1_start + nm1_end];
            loop2330.nm1 = get_nm1(nm1_content.to_string());
            contents = contents[nm1_start + nm1_end + 1..].to_string();
        }
    }

    // Parse N3 segment (optional, raw string)
    if let Some(n3_start) = contents.find("N3*") {
        let boundary = ["NM1*", "N1*", "COB*", "HD*", "INS*", "SE*"]
            .iter()
            .filter_map(|s| contents.find(s))
            .min()
            .unwrap_or(contents.len());
        if n3_start < boundary {
            if let Some(n3_end) = contents[n3_start..].find("~") {
                loop2330.n3 = Some(contents[n3_start..n3_start + n3_end].to_string());
                contents = contents[n3_start + n3_end + 1..].to_string();
            }
        }
    }

    // Parse N4 segment (optional)
    if let Some(n4_start) = contents.find("N4*") {
        let boundary = ["NM1*", "N1*", "COB*", "HD*", "INS*", "SE*", "PER*"]
            .iter()
            .filter_map(|s| contents.find(s))
            .min()
            .unwrap_or(contents.len());
        if n4_start < boundary {
            if let Some(n4_end) = contents[n4_start..].find("~") {
                let n4_content = &contents[n4_start + 3..n4_start + n4_end];
                loop2330.n4 = Some(get_n4(n4_content.to_string()));
                contents = contents[n4_start + n4_end + 1..].to_string();
            }
        }
    }

    (loop2330, contents)
}

pub fn write_loop2330(loop2330: Loop2330) -> String {
    let mut result = String::new();
    if loop2330.nm1.entity_id.is_empty() {
        return result;
    }
    result.push_str(&write_nm1(loop2330.nm1));
    result.push('\n');
    if let Some(n3) = &loop2330.n3 {
        result.push_str(n3);
        result.push_str("~\n");
    }
    if let Some(n4) = &loop2330.n4 {
        result.push_str(&write_n4(n4.clone()));
        result.push('\n');
    }
    result
}
