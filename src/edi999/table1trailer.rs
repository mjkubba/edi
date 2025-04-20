use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::se::*;
use crate::segments::ak9::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1trailer {
    pub se_segments: SE,
    pub ak9_segments: AK9,
}

pub fn get_first_table_trailer(mut contents:String) -> (SE, AK9, String) {

    let mut se_segments = SE::default();
    let mut ak9_segments = AK9::default();
   

    if contents.contains("SE") {
        info!("SE segment found, ");
        se_segments = get_se(get_segment_contents("SE", &contents));
        info!("SE segment parsed");
        contents = content_trim("SE",contents);
    }


    if contents.contains("AK9") {
        info!("AK9 segment found, ");
        ak9_segments = get_ak9(get_segment_contents("AK9", &contents));
        info!("AK9 segment parsed");
        contents = content_trim("AK9", contents);
    }




    info!("Table 1 parsed\n");
    return (se_segments, ak9_segments, contents)
}


pub fn get_table1trailer(contents:String) -> (Table1trailer, String) {
    let (se_segments, ak9_segments, contents) = get_first_table_trailer(contents);
    let header  = Table1trailer {
        ak9_segments,
        se_segments,

    };
    return (header,contents)
}

pub fn write_table1trailer(table1:Table1trailer) -> String {
    let mut contents = String::new();
    contents.push_str(&write_se(table1.se_segments));
    contents.push_str(&write_ak9(table1.ak9_segments));

    return contents;
}


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