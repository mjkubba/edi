use crate::segments::svc::*;
use crate::segments::dtm::*;
use crate::segments::cas::*;
use crate::segments::r#ref::*;
use crate::segments::amt::*;
use crate::segments::qty::*;
use crate::segments::lq::*;
use crate::helper::helper::*;

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