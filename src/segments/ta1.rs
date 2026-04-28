use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct TA1 {
    pub ta01_interchange_control_number: String,
    pub ta02_interchange_date: String,
    pub ta03_interchange_time: String,
    pub ta04_interchange_ack_code: String,
    pub ta05_interchange_note_code: String,
}

pub fn get_ta1(ta1_content: String) -> TA1 {
    let ta1_parts: Vec<&str> = ta1_content.split("*").collect();
    TA1 {
        ta01_interchange_control_number: get_element(&ta1_parts, 0),
        ta02_interchange_date: get_element(&ta1_parts, 1),
        ta03_interchange_time: get_element(&ta1_parts, 2),
        ta04_interchange_ack_code: get_element(&ta1_parts, 3),
        ta05_interchange_note_code: get_element(&ta1_parts, 4),
    }
}

pub fn write_ta1(ta1: &TA1) -> String {
    if ta1.ta01_interchange_control_number.is_empty() {
        return String::new();
    }
    format!(
        "TA1*{}*{}*{}*{}*{}~",
        ta1.ta01_interchange_control_number,
        ta1.ta02_interchange_date,
        ta1.ta03_interchange_time,
        ta1.ta04_interchange_ack_code,
        ta1.ta05_interchange_note_code,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ta1() {
        let ta1 = get_ta1("000000905*030101*1253*A*000".to_string());
        assert_eq!(ta1.ta01_interchange_control_number, "000000905");
        assert_eq!(ta1.ta02_interchange_date, "030101");
        assert_eq!(ta1.ta03_interchange_time, "1253");
        assert_eq!(ta1.ta04_interchange_ack_code, "A");
        assert_eq!(ta1.ta05_interchange_note_code, "000");
    }

    #[test]
    fn test_write_ta1() {
        let ta1 = TA1 {
            ta01_interchange_control_number: "000000905".to_string(),
            ta02_interchange_date: "030101".to_string(),
            ta03_interchange_time: "1253".to_string(),
            ta04_interchange_ack_code: "A".to_string(),
            ta05_interchange_note_code: "000".to_string(),
        };
        assert_eq!(write_ta1(&ta1), "TA1*000000905*030101*1253*A*000~");
    }
}
