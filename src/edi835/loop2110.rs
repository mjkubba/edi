use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::svc::*;
use crate::segments::dtm::*;
use crate::segments::cas::*;
use crate::segments::r#ref::*;
use crate::segments::amt::*;
use crate::segments::qty::*;
use crate::segments::lq::*;
use crate::helper::helper::*;




#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2110s {
    pub svc_segments: SVC,
    pub dtm_segments: Vec<DTM>,
    pub cas_segments: Vec<CAS>,
    pub ref_service_identification: Vec<REF>,
    pub ref_line_item_control_number: REF,
    pub ref_rendering_provider_information: Vec<REF>,
    pub ref_healthcare_policy_identification: Vec<REF>,
    pub amt_segments: Vec<AMT>,
    pub qty_segments: Vec<QTY>,
    pub lq_segments: Vec<LQ>,
}

pub fn get_loop_2110(mut contents: String) -> (SVC, Vec<DTM>, Vec<CAS>, Vec<REF>, REF, Vec<REF>, Vec<REF>, Vec<AMT>, Vec<QTY>, Vec<LQ>, String) {
    
    // Loop 2110 Service Payment Information (999)
    // SVC Service Payment Information S 1
    // DTM Service Date S 2
    // CAS Service Adjustment S 99
    // REF Service Identification S 8
    // REF Line Item Control Number S 1
    // REF Rendering Provider Information S 10
    // REF HealthCare Policy Identification S 5
    // AMT Service Supplemental Amount S 9
    // QTY Service Supplemental Quantity S 6
    // LQ Health Care Remark Codes S 99

    let mut svc_segments = SVC::default();
    let mut dtm_segments = vec![];
    let mut cas_segments = vec![];
    let mut ref_service_identification = vec![];
    let mut ref_line_item_control_number = REF::default();
    let mut ref_rendering_provider_information = vec![];
    let mut ref_healthcare_policy_identification = vec![];
    let mut amt_segments = vec![];
    let mut qty_segments = vec![];
    let mut lq_segments = vec![];

    if contents.contains("SVC") {
        info!("SVC segment found, ");
        svc_segments = get_svc(get_segment_contents("SVC", &contents));
        info!("SVC segment parsed");
        contents = content_trim("SVC",contents);
    }

    if contents.contains("DTM") {
        info!("DTM segment found, ");
        let dtm_count = contents.matches("DTM").count();
        for _ in 0..dtm_count {
            let dtm_tmp = get_dtm(get_segment_contents("DTM", &contents));
            if check_for_expected_codes("150,151,472", dtm_tmp.date_time_qualifier.clone()) {
            // if check_if_segement_in_loop("DTM", "SVC", contents.clone()) {
                info!("DTM segment found, in the correct loop");
                dtm_segments.push(dtm_tmp);
                info!("DTM segment parsed");
                contents = content_trim("DTM",contents);
            }
        }
    }
    // if contents.contains("DTM") {
    //     info!("DTM segment found, ");
    //     dtm_segments = get_dtm(get_segment_contents("DTM", &contents));
    //     info!("DTM segment parsed");
    //     contents = content_trim("DTM",contents);
    // }
    if contents.contains("CAS") {
        let cas_count = contents.matches("CAS").count();
        for _ in 0..cas_count {
            let cas_tmp = get_cas(get_segment_contents("CAS", &contents));
            info!("CAS segment found, ");
            cas_segments.push(cas_tmp);
            info!("CAS segment parsed");
            contents = content_trim("CAS",contents);
        }
    }
    // if contents.contains("CAS") {
    //     info!("CAS segment found, ");
    //     cas_segments = get_cas(get_segment_contents("CAS", &contents));
    //     info!("CAS segment parsed");
    //     contents = content_trim("CAS",contents);
    // }

    /*
    TODO: we need to get the expected codes in the REF segments
    */ 

    if contents.contains("REF") {
        info!("REF segment found, ");
        let ref_count = contents.matches("REF").count();
        for _ in 0..ref_count {
            let ref_tmp = get_ref(get_segment_contents("REF", &contents));
            if check_for_expected_codes("1S,APC,BB,E9,G1,G3,LU,RB", ref_tmp.reference_id_number_qualifier.clone()) {
                info!("REF segment found, in the right loop");
                ref_service_identification.push(get_ref(get_segment_contents("REF", &contents)));
                info!("REF segment parsed");
                contents = content_trim("REF",contents);
            }
        }
    }
    if contents.contains("REF") {
        info!("REF segment found, ");
        let ref_tmp = get_ref(get_segment_contents("REF", &contents));
        if check_for_expected_codes("6R", ref_tmp.reference_id_number_qualifier.clone()) {
            ref_line_item_control_number = ref_tmp;
            info!("REF segment parsed");
            contents = content_trim("REF",contents);
        }
    }

    if contents.contains("REF") {
        info!("REF segment found, ");
        let ref_count = contents.matches("REF").count();
        for _ in 0..ref_count {
            let ref_tmp = get_ref(get_segment_contents("REF", &contents));
            if check_for_expected_codes("0B,1A,1B,1C,1D,1G,1H,1J,D3,G2,HPI,SY,TJ", ref_tmp.reference_id_number_qualifier.clone()) {
                info!("REF segment found, in the right loop");
                ref_rendering_provider_information.push(get_ref(get_segment_contents("REF", &contents)));
                info!("REF segment parsed");
                contents = content_trim("REF",contents);
            }
        }
    }
    if contents.contains("REF") {
        info!("REF segment found, ");
        let ref_count = contents.matches("REF").count();
        for _ in 0..ref_count {
            let ref_tmp = get_ref(get_segment_contents("REF", &contents));
            if check_for_expected_codes("0K", ref_tmp.reference_id_number_qualifier.clone()) {
                info!("REF segment found, in the right loop");
                ref_healthcare_policy_identification.push(get_ref(get_segment_contents("REF", &contents)));
                info!("REF segment parsed");
                contents = content_trim("REF",contents);
            }
        }
    }
    // if contents.contains("REF") {
    //     info!("REF segment found, ");
    //     ref_line_item_control_number = get_ref(get_segment_contents("REF", &contents));
    //     info!("REF segment parsed");
    //     contents = content_trim("REF",contents);
    // }
    // if contents.contains("REF") {
    //     info!("REF segment found, ");
    //     ref_rendering_provider_information = get_ref(get_segment_contents("REF", &contents));
    //     info!("REF segment parsed");
    //     contents = content_trim("REF",contents);
    // }
    // if contents.contains("REF") {
    //     info!("REF segment found, ");
    //     ref_healthcare_policy_identification = get_ref(get_segment_contents("REF", &contents));
    //     info!("REF segment parsed");
    //     contents = content_trim("REF",contents);
    // }

    // if contents.contains("AMT") {
    //     info!("AMT segment found, ");
    //     amt_segments = get_amt(get_segment_contents("AMT", &contents));
    //     info!("AMT segment parsed");
    //     contents = content_trim("AMT",contents);
    // }
    if contents.contains("AMT") {
        let amt_count = contents.matches("AMT").count();
        for _ in 0..amt_count {
            info!("AMT segment found, ");
            amt_segments.push(get_amt(get_segment_contents("AMT", &contents)));
            info!("AMT segment parsed");
            contents = content_trim("AMT",contents);
        }
    }
    // if contents.contains("QTY") {
    //     info!("QTY segment found, ");
    //     qty_segments = get_qty(get_segment_contents("QTY", &contents));
    //     info!("QTY segment parsed");
    //     contents = content_trim("QTY",contents);
    // }
    if contents.contains("QTY") {
        let qty_count = contents.matches("QTY").count();
        for _ in 0..qty_count {
            info!("QTY segment found, ");
            qty_segments.push(get_qty(get_segment_contents("QTY", &contents)));
            info!("QTY segment parsed");
            contents = content_trim("QTY",contents);
        }
    }
    // if contents.contains("LQ") {
    //     info!("LQ segment found, ");
    //     lq_segments = get_lq(get_segment_contents("LQ", &contents));
    //     info!("LQ segment parsed");
    //     contents = content_trim("LQ",contents);
    // }
    if contents.contains("LQ") {
        let lq_count = contents.matches("LQ").count();
        for _ in 0..lq_count {
            info!("LQ segment found, ");
            lq_segments.push(get_lq(get_segment_contents("LQ", &contents)));
            info!("LQ segment parsed");
            contents = content_trim("LQ",contents);
        }
    }

    info!("Loop 2110 parsed\n");

    return (svc_segments, dtm_segments, cas_segments, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt_segments, 
            qty_segments, lq_segments, contents)
}

    // // Loop 2110 Service Payment Information
    // let svc_count= contents.matches("SVC").count();
    // let mut loop_2110_array = vec![];
    // info!("Number of loops in loop 2110: {:?}",svc_count);
    // let (mut svc, mut dtm, mut cas, mut ref_service_identification, mut ref_line_item_control_number, 
    //      mut ref_rendering_provider_information, mut ref_healthcare_policy_identification, mut amt, mut qty, mut lq);

    // for _ in 0..svc_count {
    //     (svc, dtm, cas, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt, qty, lq, contents) =
    //     get_loop_2110(contents.clone());
    //     let loop2110 = get_loop_2110s(svc,dtm,cas,ref_service_identification,ref_line_item_control_number,ref_rendering_provider_information,ref_healthcare_policy_identification,amt,qty,lq);
    //     loop_2110_array.push(loop2110);
    // }

pub fn get_loop_2110s(mut contents: String) -> (Vec<Loop2110s>, String) {

    let svc_count= contents.matches("SVC").count();
    let mut loop_2110_array = vec![];
    info!("Number of loops in loop 2110: {:?}",svc_count);
    for _ in 0..svc_count {
        let tmp_contents = get_loop_contents("SVC", "SVC", contents.clone());
        
        let ( svc_segments,  dtm_segments,  cas_segments,  ref_service_identification,  ref_line_item_control_number,  ref_rendering_provider_information, 
             ref_healthcare_policy_identification,  amt_segments,  qty_segments,  lq_segments, rem_contents);
        (svc_segments, dtm_segments, cas_segments, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt_segments, 
         qty_segments, lq_segments, rem_contents) = get_loop_2110(tmp_contents.clone());
    
        let loop2110 = Loop2110s {
            svc_segments,
            dtm_segments,
            cas_segments,
            ref_service_identification,
            ref_line_item_control_number,
            ref_rendering_provider_information,
            ref_healthcare_policy_identification,
            amt_segments,
            qty_segments,
            lq_segments,
        };
        contents = contents.replacen(&tmp_contents, "",1);
        contents.push_str(&rem_contents);
        loop_2110_array.push(loop2110);
    }
    return (loop_2110_array, contents);
}

pub fn write_loop2110(loop2110:Vec<Loop2110s>) -> String {
    let mut contents = String::new();
    for loop2110 in loop2110 {
        contents.push_str(&write_svc(loop2110.svc_segments));
        for n in 0..loop2110.dtm_segments.len() {
            contents.push_str(&write_dtm(loop2110.dtm_segments[n].clone()));
        }
        // contents.push_str(&write_dtm(loop2110.dtm_segments));
        for n in 0..loop2110.cas_segments.len() {
            contents.push_str(&write_cas(loop2110.cas_segments[n].clone()));
        }
        // contents.push_str(&write_cas(loop2110.cas_segments));
        for n in 0..loop2110.ref_service_identification.len() {
            contents.push_str(&write_ref(loop2110.ref_service_identification[n].clone()));
        }
        // contents.push_str(&write_ref(loop2110.ref_service_identification));
        contents.push_str(&write_ref(loop2110.ref_line_item_control_number));

        for n in 0..loop2110.ref_rendering_provider_information.len() {
            contents.push_str(&write_ref(loop2110.ref_rendering_provider_information[n].clone()));
        }
        for n in 0..loop2110.ref_healthcare_policy_identification.len() {
            contents.push_str(&write_ref(loop2110.ref_healthcare_policy_identification[n].clone()));
        }
        // contents.push_str(&write_ref(loop2110.ref_rendering_provider_information));
        // contents.push_str(&write_ref(loop2110.ref_healthcare_policy_identification));
        for n in 0..loop2110.amt_segments.len() {
            contents.push_str(&write_amt(loop2110.amt_segments[n].clone()));
        }
        for n in 0..loop2110.qty_segments.len() {
            contents.push_str(&write_qty(loop2110.qty_segments[n].clone()));
        }
        for n in 0..loop2110.lq_segments.len() {
            contents.push_str(&write_lq(loop2110.lq_segments[n].clone()));
        }
        // contents.push_str(&write_amt(loop2110.amt_segments));
        // contents.push_str(&write_qty(loop2110.qty_segments));
        // contents.push_str(&write_lq(loop2110.lq_segments));
    }
    return contents;
}

// unit tests

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2110() {
        let contents = String::from("~SVC*HC|99213*500*100**1~DTM*472*20191001~CAS*OA*23*400~REF*6R*1~AMT*B6*450~SE*22*35681~GE*1*1~IEA*1*000000905~");
        let (svc, dtm, cas, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt, qty, lq, contents) = get_loop_2110(contents
            );
        assert_eq!(contents, "SE*22*35681~GE*1*1~IEA*1*000000905~");
        assert_eq!(svc.svc01_1_product_or_service_is_qualifier, "HC|99213");
        assert_eq!(dtm.date_time_qualifier, "472");
        assert_eq!(cas.cas01_claim_adjustsment_group_code, "OA");
        assert_eq!(ref_service_identification.reference_id_number_qualifier, "6R");
        assert_eq!(ref_line_item_control_number, REF::default());
        assert_eq!(ref_rendering_provider_information, REF::default());
        assert_eq!(ref_healthcare_policy_identification, REF::default());
        assert_eq!(amt.amt01_amount_qualifier_code, "B6");
        assert_eq!(qty, QTY::default());
        assert_eq!(lq, LQ::default());

    }
    // #[test]
    // fn test_get_loop_2110s() {
    //     let svc_segments = SVC::default();
    //     let dtm_segments = DTM::default();
    //     let cas_segments = CAS::default();
    //     let ref_service_identification = REF::default();
    //     let ref_line_item_control_number = REF::default();
    //     let ref_rendering_provider_information = REF::default();
    //     let ref_healthcare_policy_identification = REF::default();
    //     let amt_segments = AMT::default();
    //     let qty_segments = QTY::default();
    //     let lq_segments = LQ::default();
    //     let loop2110 = get_loop_2110s(svc_segments, dtm_segments, cas_segments, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, 
    //         ref_healthcare_policy_identification, amt_segments, qty_segments, lq_segments);
    //     assert_eq!(loop2110.svc_segments, SVC::default());
    //     assert_eq!(loop2110.dtm_segments, DTM::default());
    //     assert_eq!(loop2110.cas_segments, CAS::default());
    //     assert_eq!(loop2110.ref_service_identification, REF::default());
    // }
}
