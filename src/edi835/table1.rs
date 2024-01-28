use log::info;

use crate::segments::st::*;
use crate::segments::bpr::*;
use crate::segments::trn::*;
use crate::segments::cur::*;
use crate::segments::r#ref::*;
use crate::segments::dtm::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone)]
pub struct Table1s {
    pub st_segments: ST,
    pub bpr_segments: BPR,
    pub trn_segments: TRN,
    pub cur_segments: CUR,
    pub ref_receiver_segments: REF,
    pub ref_version_segments: REF,
    pub dtm_segments: DTM,    
}


pub fn get_first_table_header(mut contents:String) -> (ST, BPR, TRN, CUR, REF, REF, DTM, String) {

    // Table 1
    // Notes format: Code(x) Code is the segment name and x is the number if repeats
    // R: required
    // S: optional (situational)
    // Number at end is number of repeats

    // Start of header loop (1)
    // ST Transaction Set Header R 1
    // BPR Financial Information R 1
    // TRN Reassociation Trace Number R 1
    // CUR Foreign Currency Information S 1
    // REF Receiver Identification S 1
    // REF Version Identification S 1
    // DTM Production Date S 1
    
    // Required: ST(1), BPR(1), TRN(1)
    // Optional: CUR(1), REF(1), REF(1), DTM(1)
    let mut st_segments = ST::default();
    let mut bpr_segments = BPR::default();
    let mut trn_segments = TRN::default();
    let mut cur_segments = CUR::default();
    let mut ref_receiver_segments = REF::default();
    let mut ref_version_segments = REF::default();
    let mut dtm_segments = DTM::default();

    if contents.contains("ST") {
        info!("ST segment found, ");
        st_segments = get_st(get_segment_contents("ST", &contents));
        info!("ST segment parsed");
        contents = content_trim("ST",contents);
    }

    if contents.contains("BPR") {
        info!("BPR segment found, ");
        bpr_segments = get_bpr(get_segment_contents("BPR", &contents));
        info!("BPR segment parsed");
        contents = content_trim("BPR",contents);
    }

    if contents.contains("TRN") {
        info!("TRN segment found, ");
        trn_segments = get_trn(get_segment_contents("TRN", &contents));
        info!("TRN segment parsed");
        contents = content_trim("TRN",contents);
    }
    
    if contents.contains("CUR") {
        info!("CUR segment found, ");
        cur_segments = get_cur(get_segment_contents("CUR", &contents));
        info!("CUR segment parsed");
        contents = content_trim("CUR",contents);
    }

    if contents.contains("REF") {
        info!("REF segment found, ");
        ref_receiver_segments = get_ref(get_segment_contents("REF", &contents));
        info!("REF segment parsed");
        contents = content_trim("REF",contents);
    }

    if contents.contains("REF") {
        info!("REF segment found, ");
        ref_version_segments = get_ref(get_segment_contents("REF", &contents));
        info!("REF segment parsed");
        contents = content_trim("REF",contents);
    }

    if contents.contains("DTM") {
        info!("DTM segment found, ");
        dtm_segments = get_dtm(get_segment_contents("DTM", &contents));
        info!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }


    info!("Table 1 parsed\n");
    return (st_segments, bpr_segments, trn_segments, cur_segments, ref_receiver_segments, ref_version_segments, dtm_segments, contents)
}


pub fn get_table1s(contents:String) -> (Table1s, String) {
    let (st_segments, bpr_segments, trn_segments, cur_segments, ref_receiver_segments, ref_version_segments, dtm_segments, contents) = get_first_table_header(contents);
    let header  = Table1s {
        st_segments,
        bpr_segments,
        trn_segments,
        cur_segments,
        ref_receiver_segments,
        ref_version_segments,
        dtm_segments,
    };
    return (header,contents)
}


// unit tests

#[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_table1_header() {
            let contents = String::from("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~BPR*I*100.00*C*CHK************20190331~TRN*1*12345*1512345678~REF*EV*CLEARINGHOUSE~");
            let (st_segments, bpr_segments, trn_segments, cur_segments, ref_receiver_segments, ref_version_segments, dtm_segments, contents) = get_first_table_header(contents);
            assert_eq!(st_segments.transaction_set_id, "835");
            assert_eq!(trn_segments.reference_id, "12345");
            assert_eq!(bpr_segments.bpr01_transaction_handling_code, "I");
            assert_eq!(cur_segments, CUR::default());
            assert_eq!(ref_receiver_segments.reference_id_number_qualifier, "EV");
            assert_eq!(ref_version_segments, REF::default());
            assert_eq!(dtm_segments, DTM::default());
            assert_eq!(contents, "ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~");
        }
    }
