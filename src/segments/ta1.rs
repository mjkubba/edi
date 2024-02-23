use serde::{Serialize, Deserialize};


#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
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
        ta01_interchange_control_number: ta1_parts[0].to_string(),
        ta02_interchange_date: ta1_parts[1].to_string(),
        ta03_interchange_time: ta1_parts[2].to_string(),
        ta04_interchange_ack_code: ta1_parts[3].to_string(),
        ta05_interchange_note_code: ta1_parts[4].to_string(),
    }
}

pub fn write_ta1(ta1: TA1) -> String {
    let mut ta1_string = String::new();
    ta1_string.push_str("TA1*");
    ta1_string.push_str(&ta1.ta01_interchange_control_number);
    ta1_string.push_str("*");
    ta1_string.push_str(&ta1.ta02_interchange_date);
    ta1_string.push_str("*");
    ta1_string.push_str(&ta1.ta03_interchange_time);
    ta1_string.push_str("*");
    ta1_string.push_str(&ta1.ta04_interchange_ack_code);
    ta1_string.push_str("*");
    ta1_string.push_str(&ta1.ta05_interchange_note_code);
    ta1_string.push_str("~");
    ta1_string
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_ta1() {
        let ta1_content = "123456789*20210302*A*1*2";
        let ta1 = get_ta1(ta1_content.to_string());
        assert_eq!(ta1.ta01_interchange_control_number, "123456789");
        assert_eq!(ta1.ta02_interchange_date, "20210302");
        assert_eq!(ta1.ta03_interchange_time, "A");
        assert_eq!(ta1.ta04_interchange_ack_code, "1");
        assert_eq!(ta1.ta05_interchange_note_code, "2");
    }
}
