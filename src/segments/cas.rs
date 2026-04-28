use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct CAS {
    pub cas01_claim_adjustment_group_code: String,
    pub cas02_adjustment_reason_code: String,
    pub cas03_adjustment_amt: String,
    pub cas04_adjustment_qty: String,
    pub cas05_adjustment_reason_code: String,
    pub cas06_adjustment_amt: String,
    pub cas07_adjustment_qty: String,
    pub cas08_adjustment_reason_code: String,
    pub cas09_adjustment_amt: String,
    pub cas10_adjustment_qty: String,
    pub cas11_adjustment_reason_code: String,
    pub cas12_adjustment_amt: String,
    pub cas13_adjustment_qty: String,
    pub cas14_adjustment_reason_code: String,
    pub cas15_adjustment_amt: String,
    pub cas16_adjustment_qty: String,
    pub cas17_adjustment_reason_code: String,
    pub cas18_adjustment_amt: String,
    pub cas19_adjustment_qty: String,
}

pub fn get_cas(cas_content: String) -> CAS {
    let cas_parts: Vec<&str> = cas_content.split("*").collect();
    let mut cas04_adjustment_qty: String = "".to_string();
    let mut cas05_adjustment_reason_code: String = "".to_string();
    let mut cas06_adjustment_amt: String = "".to_string();
    let mut cas07_adjustment_qty: String = "".to_string();
    let mut cas08_adjustment_reason_code: String = "".to_string();
    let mut cas09_adjustment_amt: String = "".to_string();
    let mut cas10_adjustment_qty: String = "".to_string();
    let mut cas11_adjustment_reason_code: String = "".to_string();
    let mut cas12_adjustment_amt: String = "".to_string();
    let mut cas13_adjustment_qty: String = "".to_string();
    let mut cas14_adjustment_reason_code: String = "".to_string();
    let mut cas15_adjustment_amt: String = "".to_string();
    let mut cas16_adjustment_qty: String = "".to_string();
    let mut cas17_adjustment_reason_code: String = "".to_string();
    let mut cas18_adjustment_amt: String = "".to_string();
    let mut cas19_adjustment_qty: String = "".to_string();

    if cas_parts.get(3).is_some() {
        cas04_adjustment_qty = get_element(&cas_parts, 3);
    }
    if cas_parts.get(4).is_some() {
        cas05_adjustment_reason_code = get_element(&cas_parts, 4);
    }
    if cas_parts.get(5).is_some() {
        cas06_adjustment_amt = get_element(&cas_parts, 5);
    }
    if cas_parts.get(6).is_some() {
        cas07_adjustment_qty = get_element(&cas_parts, 6);
    }
    if cas_parts.get(7).is_some() {
        cas08_adjustment_reason_code = get_element(&cas_parts, 7);
    }
    if cas_parts.get(8).is_some() {
        cas09_adjustment_amt = get_element(&cas_parts, 8);
    }
    if cas_parts.get(9).is_some() {
        cas10_adjustment_qty = get_element(&cas_parts, 9);
    }
    if cas_parts.get(10).is_some() {
        cas11_adjustment_reason_code = get_element(&cas_parts, 10);
    }
    if cas_parts.get(11).is_some() {
        cas12_adjustment_amt = get_element(&cas_parts, 11);
    }
    if cas_parts.get(12).is_some() {
        cas13_adjustment_qty = get_element(&cas_parts, 12);
    }
    if cas_parts.get(13).is_some() {
        cas14_adjustment_reason_code = get_element(&cas_parts, 13);
    }
    if cas_parts.get(14).is_some() {
        cas15_adjustment_amt = get_element(&cas_parts, 14);
    }
    if cas_parts.get(15).is_some() {
        cas16_adjustment_qty = get_element(&cas_parts, 15);
    }
    if cas_parts.get(16).is_some() {
        cas17_adjustment_reason_code = get_element(&cas_parts, 16);
    }
    if cas_parts.get(17).is_some() {
        cas18_adjustment_amt = get_element(&cas_parts, 17);
    }
    if cas_parts.get(18).is_some() {
        cas19_adjustment_qty = get_element(&cas_parts, 18);
    }
    CAS {
        cas01_claim_adjustment_group_code: get_element(&cas_parts, 0),
        cas02_adjustment_reason_code: get_element(&cas_parts, 1),
        cas03_adjustment_amt: get_element(&cas_parts, 2),
        cas04_adjustment_qty,
        cas05_adjustment_reason_code,
        cas06_adjustment_amt,
        cas07_adjustment_qty,
        cas08_adjustment_reason_code,
        cas09_adjustment_amt,
        cas10_adjustment_qty,
        cas11_adjustment_reason_code,
        cas12_adjustment_amt,
        cas13_adjustment_qty,
        cas14_adjustment_reason_code,
        cas15_adjustment_amt,
        cas16_adjustment_qty,
        cas17_adjustment_reason_code,
        cas18_adjustment_amt,
        cas19_adjustment_qty,
    }
}

pub fn write_cas(cas: CAS) -> String {
    if cas.cas01_claim_adjustment_group_code.is_empty() {
        return String::new();
    }
    build_segment(&[
        "CAS",
        &cas.cas01_claim_adjustment_group_code,
        &cas.cas02_adjustment_reason_code,
        &cas.cas03_adjustment_amt,
        &cas.cas04_adjustment_qty,
        &cas.cas05_adjustment_reason_code,
        &cas.cas06_adjustment_amt,
        &cas.cas07_adjustment_qty,
        &cas.cas08_adjustment_reason_code,
        &cas.cas09_adjustment_amt,
        &cas.cas10_adjustment_qty,
        &cas.cas11_adjustment_reason_code,
        &cas.cas12_adjustment_amt,
        &cas.cas13_adjustment_qty,
        &cas.cas14_adjustment_reason_code,
        &cas.cas15_adjustment_amt,
        &cas.cas16_adjustment_qty,
        &cas.cas17_adjustment_reason_code,
        &cas.cas18_adjustment_amt,
        &cas.cas19_adjustment_qty,
    ])
}

// unit test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_get_cas() {
        let cas_content = "CAS*P*1*10.00*1*20.00*2*30.00*3*40.00*4*50.00*5*60.00*6*70.00*7*80.00*8*90.00*9*100.00*10*110.00*11*120.00*12*130.00*13*140.00*14*150.00*15*160.00*16*170.00*17*180.00*18*190.00*19*200.00".to_string();
        let cas = get_cas(cas_content);
        assert_eq!(cas.cas01_claim_adjustment_group_code, "CAS".to_string());
        assert_eq!(cas.cas02_adjustment_reason_code, "P".to_string());
        assert_eq!(cas.cas03_adjustment_amt, "1".to_string());
        assert_eq!(cas.cas04_adjustment_qty, "10.00".to_string());
        assert_eq!(cas.cas05_adjustment_reason_code, "1".to_string());
    }
}
