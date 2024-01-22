use crate::segments::clp::*;
use crate::segments::cas::*;
use crate::segments::nm1::*;
use crate::segments::mia::*;
use crate::segments::moa::*;
use crate::segments::r#ref::*;
use crate::segments::dtm::*;
use crate::segments::per::*;
use crate::segments::amt::*;
use crate::segments::qty::*;
use crate::helper::helper::*;






pub fn get_loop_2100(mut contents:String) -> (CLP, CAS, NM1, NM1, NM1, NM1, NM1, NM1, NM1, MIA, MOA, REF, REF, DTM, DTM, DTM, PER, AMT, QTY, String) {
    // Loop 2100 Claim Payment Information (>1)
    // R: required
    // S: optional (situational)
    // Number at end is number of repeats

    // CLP Claim Payment Information R 1
    // CAS Claim Adjustment S 99
    // NM1 Patient Name R 1
    // NM1 Insured Name S 1
    // NM1 Corrected Patient/Insured Name S 1
    // NM1 Service Provider Name S 1
    // NM1 Crossover Carrier Name S 1
    // NM1 Corrected Priority Payer Name S 1
    // NM1 Other Subscriber Name S 1
    // MIA Inpatient Adjudication Information S 1
    // MOA Outpatient Adjudication Information S 1
    // REF Other Claim Related Identification S 5
    // REF Rendering Provider Identification S 10
    // DTM Statement From or To Date S 2
    // DTM Coverage Expiration Date S 1
    // DTM Claim Received Date S 1
    // PER Claim Contact Information S 2
    // AMT Claim Supplemental Information S 13
    // QTY Claim Supplemental Information Quantity S 14

    let mut clp_segments = CLP::default();
    let mut cas_segments = CAS::default();
    let mut nm1_patint_segments = NM1::default();
    let mut nm1_insured_segments = NM1::default();
    let mut nm1_corrected_patient_segments = NM1::default();
    let mut nm1_service_provider_segments = NM1::default();
    let mut nm1_crossover_carrier_segments = NM1::default();
    let mut nm1_corrected_priority_payer_segments = NM1::default();
    let mut nm1_other_subscriber_segments = NM1::default();
    let mut mia_segments = MIA::default();
    let mut moa_segments = MOA::default();
    let mut ref_other_claim_segments = REF::default();
    let mut ref_rendering_provider_segments = REF::default();
    let mut dtm_statement_from_segments = DTM::default();
    let mut dtm_coverage_expiration_segments = DTM::default();
    let mut dtm_claim_received_segments = DTM::default();
    let mut per_segments = PER::default();
    let mut amt_segments = AMT::default();
    let mut qty_segments = QTY::default();

    if contents.contains("CLP") {
        print!("CLP segment found, ");
        clp_segments = get_clp(get_segment_contents("CLP", &contents));
        println!("CLP segment parsed");
        contents = content_trim("CLP",contents);
    }
    if contents.contains("CAS") {
        print!("CAS segment found, ");
        cas_segments = get_cas(get_segment_contents("CAS", &contents));
        println!("CAS segment parsed");
        contents = content_trim("CAS",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_patint_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_insured_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_corrected_patient_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    } 
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_service_provider_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_crossover_carrier_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_corrected_priority_payer_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        print!("NM1 segment found, ");
        nm1_other_subscriber_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("MIA") {
        print!("MIA segment found, ");
        mia_segments = get_mia(get_segment_contents("MIA", &contents));
        println!("MIA segment parsed");
        contents = content_trim("MIA",contents);
    }
    if contents.contains("MOA") {
        print!("MOA segment found, ");
        moa_segments = get_moa(get_segment_contents("MOA", &contents));
        println!("MOA segment parsed");
        contents = content_trim("MOA",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_other_claim_segments = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_rendering_provider_segments = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("DTM") {
        print!("DTM segment found, ");
        dtm_statement_from_segments = get_dtm(get_segment_contents("DTM", &contents));
        println!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("DTM") {
        print!("DTM segment found, ");
        dtm_coverage_expiration_segments = get_dtm(get_segment_contents("DTM", &contents));
        println!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("DTM") {
        print!("DTM segment found, ");
        dtm_claim_received_segments = get_dtm(get_segment_contents("DTM", &contents));
        println!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("PER") {
        print!("PER segment found, ");
        per_segments = get_per(get_segment_contents("PER", &contents));
        println!("PER segment parsed");
        contents = content_trim("PER",contents);
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

    println!("Loop 2100 parsed\n");

    return (clp_segments, cas_segments, nm1_patint_segments, nm1_insured_segments, nm1_corrected_patient_segments, nm1_service_provider_segments, nm1_crossover_carrier_segments, nm1_corrected_priority_payer_segments,
            nm1_other_subscriber_segments, mia_segments, moa_segments, ref_other_claim_segments, ref_rendering_provider_segments, dtm_statement_from_segments, dtm_coverage_expiration_segments, dtm_claim_received_segments, 
            per_segments, amt_segments, qty_segments, contents)

}