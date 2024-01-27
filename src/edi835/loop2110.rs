use crate::segments::svc::*;
use crate::segments::dtm::*;
use crate::segments::cas::*;
use crate::segments::r#ref::*;
use crate::segments::amt::*;
use crate::segments::qty::*;
use crate::segments::lq::*;
use crate::helper::helper::*;




#[derive(Debug, Default,PartialEq,Clone)]
pub struct Loop2110s {
    pub svc_segments: SVC,
    pub dtm_segments: DTM,
    pub cas_segments: CAS,
    pub ref_service_identification: REF,
    pub ref_line_item_control_number: REF,
    pub ref_rendering_provider_information: REF,
    pub ref_healthcare_policy_identification: REF,
    pub amt_segments: AMT,
    pub qty_segments: QTY,
    pub lq_segments: LQ,
}

pub fn get_loop_2110(mut contents: String) -> (SVC, DTM, CAS, REF, REF, REF, REF, AMT, QTY, LQ, String) {
    
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
    let mut dtm_segments = DTM::default();
    let mut cas_segments = CAS::default();
    let mut ref_service_identification = REF::default();
    let mut ref_line_item_control_number = REF::default();
    let mut ref_rendering_provider_information = REF::default();
    let mut ref_healthcare_policy_identification = REF::default();
    let mut amt_segments = AMT::default();
    let mut qty_segments = QTY::default();
    let mut lq_segments = LQ::default();

    if contents.contains("SVC") {
        print!("SVC segment found, ");
        svc_segments = get_svc(get_segment_contents("SVC", &contents));
        println!("SVC segment parsed");
        contents = content_trim("SVC",contents);
    }
    if contents.contains("DTM") {
        print!("DTM segment found, ");
        dtm_segments = get_dtm(get_segment_contents("DTM", &contents));
        println!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("CAS") {
        print!("CAS segment found, ");
        cas_segments = get_cas(get_segment_contents("CAS", &contents));
        println!("CAS segment parsed");
        contents = content_trim("CAS",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_service_identification = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_line_item_control_number = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_rendering_provider_information = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_healthcare_policy_identification = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("AMT") {
        print!("AMT segment found, ");
        amt_segments = get_amt(get_segment_contents("AMT", &contents));
        println!("AMT segment parsed");
        contents = content_trim("AMT",contents);
    }
    if contents.contains("QTY") {
        print!("QTY segment found, ");
        qty_segments = get_qty(get_segment_contents("QTY", &contents));
        println!("QTY segment parsed");
        contents = content_trim("QTY",contents);
    }
    if contents.contains("LQ") {
        print!("LQ segment found, ");
        lq_segments = get_lq(get_segment_contents("LQ", &contents));
        println!("LQ segment parsed");
        contents = content_trim("LQ",contents);
    }

    println!("Loop 2110 parsed\n");

    return (svc_segments, dtm_segments, cas_segments, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt_segments, 
            qty_segments, lq_segments, contents)
}

    // // Loop 2110 Service Payment Information
    // let svc_count= contents.matches("SVC").count();
    // let mut loop_2110_array = vec![];
    // println!("Number of loops in loop 2110: {:?}",svc_count);
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
    println!("Number of loops in loop 2110: {:?}",svc_count);
    
    for _ in 0..svc_count {
        let ( svc_segments,  dtm_segments,  cas_segments,  ref_service_identification,  ref_line_item_control_number,  ref_rendering_provider_information, 
             ref_healthcare_policy_identification,  amt_segments,  qty_segments,  lq_segments);
        (svc_segments, dtm_segments, cas_segments, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt_segments, 
         qty_segments, lq_segments, contents) = get_loop_2110(contents.clone());
    
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
        loop_2110_array.push(loop2110);
    }
    return (loop_2110_array, contents);
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
