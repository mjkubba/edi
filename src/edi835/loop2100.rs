use log::info;
use serde::{Serialize, Deserialize};

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


#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2100s {
    pub clp_segments: CLP,
    pub cas_segments: CAS,
    pub nm1_patint_segments: NM1,
    pub nm1_insured_segments: NM1,
    pub nm1_corrected_patient_segments: NM1,
    pub nm1_service_provider_segments: NM1,
    pub nm1_crossover_carrier_segments: NM1,
    pub nm1_corrected_priority_payer_segments: NM1,
    pub nm1_other_subscriber_segments: NM1,
    pub mia_segments: MIA,
    pub moa_segments: MOA,
    pub ref_other_claim_segments: REF,
    pub ref_rendering_provider_segments: REF,
    pub dtm_statement_from_segments: DTM,
    pub dtm_coverage_expiration_segments: DTM,
    pub dtm_claim_received_segments: DTM,
    pub per_segments: PER,
    pub amt_segments: AMT,
    pub qty_segments: QTY,
}




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
        info!("CLP segment found, ");
        clp_segments = get_clp(get_segment_contents("CLP", &contents));
        info!("CLP segment parsed");
        contents = content_trim("CLP",contents);
    }
    if contents.contains("CAS") {
        info!("CAS segment found, ");
        cas_segments = get_cas(get_segment_contents("CAS", &contents));
        info!("CAS segment parsed");
        contents = content_trim("CAS",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_patint_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_insured_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_corrected_patient_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    } 
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_service_provider_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_crossover_carrier_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_corrected_priority_payer_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("NM1") {
        info!("NM1 segment found, ");
        nm1_other_subscriber_segments = get_nm1(get_segment_contents("NM1", &contents));
        info!("NM1 segment parsed");
        contents = content_trim("NM1",contents);
    }
    if contents.contains("MIA") {
        info!("MIA segment found, ");
        mia_segments = get_mia(get_segment_contents("MIA", &contents));
        info!("MIA segment parsed");
        contents = content_trim("MIA",contents);
    }
    if contents.contains("MOA") {
        info!("MOA segment found, ");
        moa_segments = get_moa(get_segment_contents("MOA", &contents));
        info!("MOA segment parsed");
        contents = content_trim("MOA",contents);
    }
    if contents.contains("REF") {
        info!("REF segment found, ");
        ref_other_claim_segments = get_ref(get_segment_contents("REF", &contents));
        info!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("REF") {
        info!("REF segment found, ");
        ref_rendering_provider_segments = get_ref(get_segment_contents("REF", &contents));
        info!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("DTM") {
        info!("DTM segment found, ");
        dtm_statement_from_segments = get_dtm(get_segment_contents("DTM", &contents));
        info!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("DTM") {
        info!("DTM segment found, ");
        dtm_coverage_expiration_segments = get_dtm(get_segment_contents("DTM", &contents));
        info!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("DTM") {
        info!("DTM segment found, ");
        dtm_claim_received_segments = get_dtm(get_segment_contents("DTM", &contents));
        info!("DTM segment parsed");
        contents = content_trim("DTM",contents);
    }
    if contents.contains("PER") {
        info!("PER segment found, ");
        per_segments = get_per(get_segment_contents("PER", &contents));
        info!("PER segment parsed");
        contents = content_trim("PER",contents);
    }
    if contents.contains("AMT") {
        info!("AMT segment found, ");
        amt_segments = get_amt(get_segment_contents("AMT", &contents));
        info!("AMT segment parsed");
        contents = content_trim("AMT",contents);
    }
    if contents.contains("QTY") {
        info!("QTY segment found, ");
        qty_segments = get_qty(get_segment_contents("QTY", &contents));
        info!("QTY segment parsed");
        contents = content_trim("QTY",contents);
    }

    info!("Loop 2100 parsed\n");

    return (clp_segments, cas_segments, nm1_patint_segments, nm1_insured_segments, nm1_corrected_patient_segments, nm1_service_provider_segments, nm1_crossover_carrier_segments, nm1_corrected_priority_payer_segments,
            nm1_other_subscriber_segments, mia_segments, moa_segments, ref_other_claim_segments, ref_rendering_provider_segments, dtm_statement_from_segments, dtm_coverage_expiration_segments, dtm_claim_received_segments, 
            per_segments, amt_segments, qty_segments, contents)

}

pub fn get_loop_2100s(mut contents:String) -> (Vec<Loop2100s>, String) {
    let clp_count= contents.matches("CLP").count();
    let mut loop_2100_array = vec![];
    info!("Number of loops in loop 2100: {:?}",clp_count);
    for _ in 0..clp_count {
        let (clp_segments, cas_segments, nm1_patint_segments, nm1_insured_segments, nm1_corrected_patient_segments, nm1_service_provider_segments, 
            nm1_crossover_carrier_segments, nm1_corrected_priority_payer_segments, nm1_other_subscriber_segments, mia_segments, moa_segments, ref_other_claim_segments, 
            ref_rendering_provider_segments, dtm_statement_from_segments, dtm_coverage_expiration_segments, dtm_claim_received_segments, per_segments, amt_segments, qty_segments);
        (clp_segments, cas_segments, nm1_patint_segments, nm1_insured_segments, nm1_corrected_patient_segments, nm1_service_provider_segments, nm1_crossover_carrier_segments, nm1_corrected_priority_payer_segments,
        nm1_other_subscriber_segments, mia_segments, moa_segments, ref_other_claim_segments, ref_rendering_provider_segments, dtm_statement_from_segments, dtm_coverage_expiration_segments, dtm_claim_received_segments, 
        per_segments, amt_segments, qty_segments, contents) = get_loop_2100(contents.clone());
        

        let loop2100 = Loop2100s {
            clp_segments,
            cas_segments,
            nm1_patint_segments,
            nm1_insured_segments,
            nm1_corrected_patient_segments,
            nm1_service_provider_segments,
            nm1_crossover_carrier_segments,
            nm1_corrected_priority_payer_segments,
            nm1_other_subscriber_segments,
            mia_segments,
            moa_segments,
            ref_other_claim_segments,
            ref_rendering_provider_segments,
            dtm_statement_from_segments,
            dtm_coverage_expiration_segments,
            dtm_claim_received_segments,
            per_segments,
            amt_segments,
            qty_segments,
        };
        loop_2100_array.push(loop2100);
    }

    (loop_2100_array, contents)
}

pub fn write_loop2100(loop2100:Vec<Loop2100s>) -> String {
    let mut contents = String::new();
    for loop2100 in loop2100 {
        contents.push_str(&write_clp(loop2100.clp_segments));
        contents.push_str(&write_cas(loop2100.cas_segments));
        contents.push_str(&write_nm1(loop2100.nm1_patint_segments));
        contents.push_str(&write_nm1(loop2100.nm1_insured_segments));
        contents.push_str(&write_nm1(loop2100.nm1_corrected_patient_segments));
        contents.push_str(&write_nm1(loop2100.nm1_service_provider_segments));
        contents.push_str(&write_nm1(loop2100.nm1_crossover_carrier_segments));
        contents.push_str(&write_nm1(loop2100.nm1_corrected_priority_payer_segments));
        contents.push_str(&write_nm1(loop2100.nm1_other_subscriber_segments));
        contents.push_str(&write_mia(loop2100.mia_segments));
        contents.push_str(&write_moa(loop2100.moa_segments));
        contents.push_str(&write_ref(loop2100.ref_other_claim_segments));
        contents.push_str(&write_ref(loop2100.ref_rendering_provider_segments));
        contents.push_str(&write_dtm(loop2100.dtm_statement_from_segments));
        contents.push_str(&write_dtm(loop2100.dtm_coverage_expiration_segments));
        contents.push_str(&write_dtm(loop2100.dtm_claim_received_segments));
        contents.push_str(&write_per(loop2100.per_segments));
        contents.push_str(&write_amt(loop2100.amt_segments));
        contents.push_str(&write_qty(loop2100.qty_segments));
    }
    return contents;
}


// unit tests

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2100() {
        let contents = String::from("CLP*EXAMPLE9*3*500*100**12*05090256390*11*1~NM1*QC*1*TOWNSEND*WILLIAM*P***MI*XXX123456789~NM1*82*2*ACME MEDICAL CENTER*****XX*98765432111~NM1**3*ACME*****XX*98765432111~NM1*11*3*ACME*****XX*91~NM1*21*31*ACME*John****XX*91~DTM*232*20190303~DTM*233*20190304~AMT*AU*500~");
        let (clp, cas, nm1_patient, nm1_insured, nm1_corrected_patient, nm1_service_provider, nm1_crossover_carrier, nm1_corrected_priority_payer,
        nm1_other_subscriber, mia, moa, ref_other_claim, ref_rendering_provider, dtm_statement_from, dtm_coverage_expiration, dtm_claim_received,
        per, amt, qty, contents) = get_loop_2100(contents);
        assert_eq!(clp.clp03_total_claim_charge_amount, "500");
        assert_eq!(cas, CAS::default());
        assert_eq!(nm1_patient.lastname, "TOWNSEND");
        assert_eq!(nm1_insured.entity_id, "82");
        assert_eq!(nm1_corrected_patient.member_number, "98765432111");
        assert_eq!(nm1_service_provider.entity_type, "3");
        assert_eq!(nm1_crossover_carrier.firstname, "John");
        assert_eq!(nm1_corrected_priority_payer, NM1::default());
        assert_eq!(nm1_other_subscriber, NM1::default());
        assert_eq!(mia, MIA::default());
        assert_eq!(moa, MOA::default());
        assert_eq!(ref_other_claim, REF::default());
        assert_eq!(ref_rendering_provider, REF::default());
        assert_eq!(dtm_statement_from.date_time_qualifier, "232");
        assert_eq!(dtm_coverage_expiration.date_time, "20190304");
        assert_eq!(dtm_claim_received, DTM::default());
        assert_eq!(per, PER::default());
        assert_eq!(amt.amt01_amount_qualifier_code, "AU");
        assert_eq!(qty, QTY::default());
        assert_eq!(contents, "");

    }
    // test get_loop_2100s
    // #[test]
    // fn test_get_loop_2100s() {
    //     let contents = String::from("CLP*EXAMPLE9*3*500*100**12*05090256390*11*1~NM1*QC*1*TOWNSEND*WILLIAM*P***MI*XXX123456789~NM1*82*2*ACME MEDICAL CENTER*****XX*98765432111~DTM*232*20190303~DTM*233*20190304~AMT*AU*500~");
    //     let (clp, cas, nm1_patient, nm1_insured, nm1_corrected_patient, nm1_service_provider, nm1_crossover_carrier, nm1_corrected_priority_payer,
    //     nm1_other_subscriber, mia, moa, ref_other_claim, ref_rendering_provider, dtm_statement_from, dtm_coverage_expiration, dtm_claim_received,
    //     per, amt, qty, _contents) = get_loop_2100(contents);
    //     let loop2100 = get_loop_2100s(clp, cas, nm1_patient, nm1_insured, nm1_corrected_patient, nm1_service_provider, nm1_crossover_carrier, nm1_corrected_priority_payer,
    //         nm1_other_subscriber, mia, moa, ref_other_claim, ref_rendering_provider, dtm_statement_from, dtm_coverage_expiration, dtm_claim_received,
    //         per, amt, qty);
    //         assert_eq!(loop2100.clp_segments.clp03_total_claim_charge_amount, "500");
    //         assert_eq!(loop2100.cas_segments, CAS::default());
    //         assert_eq!(loop2100.nm1_patint_segments.lastname, "TOWNSEND");
    //     }
}
