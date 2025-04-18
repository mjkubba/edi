use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::st::*;
use crate::segments::ak1::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1s {
    pub st_segments: ST,
    pub ak1_segments: AK1,
}

pub fn get_first_table_header(mut contents:String) -> (ST, AK1, String) {

    let mut st_segments = ST::default();
    let mut ak1_segments = AK1::default();
   

    if contents.contains("ST") {
        info!("ST segment found, ");
        st_segments = get_st(get_segment_contents("ST", &contents));
        info!("ST segment parsed");
        contents = content_trim("ST",contents);
    }


    if contents.contains("AK1") {
        info!("AK1 segment found, ");
        ak1_segments = get_ak1(get_segment_contents("AK1", &contents));
        info!("AK1 segment parsed");
        contents = content_trim("AK1", contents);
    }




    info!("Table 1 parsed\n");
    return (st_segments, ak1_segments, contents)
}


pub fn get_table1s(contents:String) -> (Table1s, String) {
    let (st_segments, ak1_segments, contents) = get_first_table_header(contents);
    let header  = Table1s {
        st_segments,
        ak1_segments,

    };
    return (header,contents)
}

// pub fn write_table1(table1:Table1s) -> String {
//     let mut contents = String::new();
//     contents.push_str(&write_st(table1.st_segments));
//     contents.push_str(&write_ak1(table1.ak1_segments));

//     return contents;
// }


// unit tests

// #[cfg(test)]
//     mod tests {
//         use super::*;
//         #[test]
//         fn test_table1_header() {
//             let contents = String::from("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~BPR*I*100.00*C*CHK************20190331~TRN*1*12345*1512345678~REF*EV*CLEARINGHOUSE~N1*");
//             let (st_segments, bpr_segments, trn_segments, cur_segments, ref_receiver_segments, ref_version_segments, dtm_segments, contents) = get_first_table_header(contents);
//             assert_eq!(st_segments.transaction_set_id, "835");
//             assert_eq!(trn_segments.reference_id, "12345");
//             assert_eq!(bpr_segments.bpr01_transaction_handling_code, "I");
//             assert_eq!(cur_segments, CUR::default());
//             assert_eq!(ref_receiver_segments.reference_id_number_qualifier, "EV");
//             assert_eq!(ref_version_segments, REF::default());
//             assert_eq!(dtm_segments, DTM::default());
//             assert_eq!(contents, "ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~N1*");
//         }
//     }