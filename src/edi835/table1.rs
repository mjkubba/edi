use crate::segments::st::*;
use crate::segments::bpr::*;
use crate::segments::trn::*;
use crate::segments::cur::*;
use crate::segments::r#ref::*;
use crate::segments::dtm::*;
use crate::helper::helper::*;


pub fn get_first_table_header(mut contents:String) -> (ST, BPR, TRN, CUR, REF, DTM, String) {

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
    let mut ref_segments = REF::default();
    let mut dtm_segments = DTM::default();

    if contents.contains("ST") {
        print!("ST segment found, ");
        st_segments = get_st(get_segment_contents("ST", &contents));
        println!("ST segment parsed");
        contents = content_trim("ST",contents);
    }

    if contents.contains("BPR") {
        print!("BPR segment found, ");
        bpr_segments = get_bpr(get_segment_contents("BPR", &contents));
        println!("BPR segment parsed");
        contents = content_trim("BPR",contents);
    }

    if contents.contains("TRN") {
        print!("TRN segment found, ");
        trn_segments = get_trn(get_segment_contents("TRN", &contents));
        println!("TRN segment parsed");
        contents = content_trim("TRN",contents);
    }
    
    if contents.contains("CUR") {
        print!("CUR segment found, ");
        cur_segments = get_cur(get_segment_contents("CUR", &contents));
        println!("CUR segment parsed");
        contents = content_trim("CUR",contents);
    }

    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_segments = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }

    if contents.contains("DTM") {
        print!("DTM segment found, ");
        dtm_segments = get_dtm(get_segment_contents("DTM", &contents));
        println!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }

    // if contents.contains("DTM") {
    //     let dtm_count= contents.matches("DTM").count();
    //     print!("Number of DTM segments: {}, ", dtm_count);

    //     let mut next_segment =  &contents[contents.find("DTM").unwrap()..];
    //     let mut _dtm_vec = Vec::new();

    //     for _ in 0..dtm_count {
    //         let dtm_start = next_segment;
    //         let dtm_end = dtm_start.find("~").unwrap();
    //         let dtm_content = &dtm_start[4..dtm_end];
    //         let dtm_segments = get_dtm(dtm_content);
    //         _dtm_vec.push(dtm_segments);
    //         next_segment = &dtm_start[dtm_end+1..]
    //     }
    //     println!("DTM segment parsed");
    // }
    
    println!("Table 1 parsed\n");
    return (st_segments, bpr_segments, trn_segments, cur_segments, ref_segments, dtm_segments, contents)
}