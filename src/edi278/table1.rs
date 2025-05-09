use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::st::*;
use crate::segments::bht::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1s {
    pub st_segments: ST,
    pub bht_segments: BHT,
}

pub fn get_table1s(mut contents: String) -> (Table1s, String) {
    let mut st_segments = ST::default();
    let mut bht_segments = BHT::default();
    
    if contents.contains("ST") {
        info!("ST segment found, ");
        st_segments = get_st(get_segment_contents("ST", &contents));
        info!("ST segment parsed");

        contents = content_trim("ST", contents);
    }
    
    if contents.contains("BHT") {
        info!("BHT segment found, ");
        bht_segments = get_bht(get_segment_contents("BHT", &contents));
        info!("BHT segment parsed");

        contents = content_trim("BHT", contents);
    }
    
    info!("Table 1 parsed\n");
    
    let table1s = Table1s {
        st_segments,
        bht_segments,
    };
    
    return (table1s, contents)
}

pub fn write_table1(table1s: Table1s) -> String {
    let mut contents = String::new();
    contents.push_str(&write_st(table1s.st_segments));
    contents.push_str(&write_bht(table1s.bht_segments));
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_table1s() {
        let contents = String::from("ST*278*0001*005010X217~BHT*0007*11*123456*20200101*1200~");
        let (table1s, contents) = get_table1s(contents);
        assert_eq!(table1s.st_segments.transaction_set_id, "278");
        assert_eq!(table1s.st_segments.transaction_set_control_number, "0001");
        assert_eq!(table1s.st_segments.implementation_conven_ref, "005010X217");
        assert_eq!(table1s.bht_segments.bht01_hierarchical_structure_code, "0007");
        assert_eq!(table1s.bht_segments.bht02_transaction_set_purpose_code, "11");
        assert_eq!(table1s.bht_segments.bht03_reference_identification, "123456");
        assert_eq!(contents, "");
    }
}
