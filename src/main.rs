use std::fs::File;
use std::io::Read;

mod edi835;
use edi835::{amt::*,bpr::*,cas::*,clp::*,cur::*,dtm::*,ge::*,gs::*,iea::*,isa::*,lq::*,lx::*,mia::*,moa::*,n1::*,n3::*,n4::*,nm1::*,per::*,plb::*,qty::*,r#ref::*,rdm::*,st::*,se::*,svc::*,ts2::*,ts3::*,trn::*};


fn get_segment_contents(key:&str, contents:  &str) -> String {
    let segment_content = get_full_segment_contents(key,contents);
    let start_skip = key.len() + 1;
    let content = &segment_content[start_skip..];
    content.to_string()
}

fn get_full_segment_contents(key:&str, contents: &str) -> String {
    let nkey = key.to_string() + "*";
    let index = contents.find(&nkey).unwrap();
    let start = &contents[index..];
    let end = start.find("~").unwrap();
    let content = &start[..end];
    content.to_string()
}

fn content_trim(key: &str, contents:String) -> String {
    contents.trim_start_matches(&get_full_segment_contents(&key, &contents)).trim_start_matches("~").to_string()
}


fn get_interchange_control(mut contents:String) -> (ISA, GS, String) {
    let mut isa_segments = ISA::default();
    let mut gs_segments = GS::default();
    if contents.contains("ISA") {
        print!("ISA segment found, ");
        isa_segments = get_isa(get_segment_contents("ISA", &contents));
        println!("ISA segment parsed");

        contents = content_trim("ISA", contents);
    }
        if contents.contains("GS") {
        print!("GS segment found, ");
        gs_segments = get_gs(get_segment_contents("GS", &contents));
        println!("GS segment parsed");
 
        contents = content_trim("GS",contents);
    }
    
    println!("Interchange Control parsed\n");
    return (isa_segments, gs_segments, contents)
}

fn get_first_table_header(mut contents:String) -> (ST, BPR, TRN, CUR, REF, DTM, String) {

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

fn get_loop_1000_a(mut contents:String) -> (N1, N3, N4, PER, String) {
    
    // Loop 1000A Payer Identification (1)
    // N1 Payer Identification R 1
    // N3 Payer Address R 1
    // N4 Payer City, State, ZIP Code R 1
    // REF Additional Payer Identification S 4
    // PER Payer Business Contact Information S 1
    // PER Payer Technical Contact Information R >1
    // PER Payer WEB Site S 1

    // Required: N1(1), N3(1), N4(1), PER(>1)
    // Optional: REF(4), PER(1)
    // PER Payer Business Contact Information: optional
    // PER Payer Technical Contact Information: required
    // PER Payer WEB Site: optional

    let mut n1_segments = N1::default();
    let mut n3_segments = N3::default();
    let mut n4_segments = N4::default();
    let mut per_segments = PER::default();

    if contents.contains("N1") {
        print!("N1 segment found, ");
        n1_segments = get_n1(get_segment_contents("N1", &contents));
        println!("N1 segment parsed");
        contents = content_trim("N1",contents);
    } 

    if contents.contains("N3") {
        print!("N3 segment found, ");
        n3_segments = get_n3(get_segment_contents("N3", &contents));
        println!("N3 segment parsedm");
        contents = content_trim("N3",contents);
    }

    if contents.contains("N4") {
        print!("N4 segment found, ");
        n4_segments = get_n4(get_segment_contents("N4", &contents));
        println!("N4 segment parsed"); 
        contents = content_trim("N4",contents);
    }

    if contents.contains("PER") {
        print!("PER segment found, ");
        per_segments = get_per(get_segment_contents("PER", &contents));
        println!("PER segment parsed");
        contents = content_trim("PER",contents);
    }

    println!("Loop 1000A parsed\n");
    return (n1_segments, n3_segments, n4_segments, per_segments, contents)
}

fn get_loop_1000_b(mut contents:String) -> (N1, N3, N4, REF, RDM, String) {
    // Loop 1000B Payee Identification (1)
    // N1 Payee Identification R 1
    // N3 Payee Address S 1
    // N4 Payee City, State, ZIP Code R 1
    // REF Payee Additional Identification S >1
    // RDM Remittance Delivery Method S 1

    // Required: N1(1), N4(1)
    // Optional: N3(1), REF(>1), RDM(1)

    let mut n1_segments = N1::default();
    let mut n3_segments = N3::default();
    let mut n4_segments = N4::default();
    let mut ref_segments = REF::default();
    let mut rdm_segments = RDM::default();

    if contents.contains("N1") {
        print!("N1 segment found, ");
        n1_segments = get_n1(get_segment_contents("N1", &contents));
        println!("N1 segment parsed");
        contents = content_trim("N1",contents);
    }
    if contents.contains("N3") {
        print!("N3 segment found, ");
        n3_segments = get_n3(get_segment_contents("N3", &contents));
        println!("N3 segment parsed");
        contents = content_trim("N3",contents);
    }
    if contents.contains("N4") {
        print!("N4 segment found, ");
        n4_segments = get_n4(get_segment_contents("N4", &contents));
        println!("N4 segment parsed");
        contents = content_trim("N4",contents);
    }
    if contents.contains("REF") {
        print!("REF segment found, ");
        ref_segments = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
        contents = content_trim("REF",contents);
    }
    if contents.contains("RDM") {
        print!("RDM segment found, ");
        rdm_segments = get_rdm(get_segment_contents("RDM", &contents));
        println!("RDM segment parsed");
        contents = content_trim("RDM",contents);
    }

    println!("Loop 1000B parsed\n");
    return (n1_segments, n3_segments, n4_segments, ref_segments, rdm_segments, contents)
}

fn get_loop_2000(mut contents:String) -> (LX, TS3, TS2, String) {
    // Table 2 
    // Loop 2000 Header Number (>1)
    // LX Header Number S 1
    // TS3 Provider Summary Information S 1
    // TS2 Provider Supplemental Summary Information S 1
    // Optional LX(1), TS3(1), TS2(1)

    let mut lx_segments = LX::default();
    let mut ts3_segments = TS3::default();
    let mut ts2_segments = TS2::default();

    if contents.contains("LX") {
        print!("LX segment found, ");
        lx_segments = get_lx(get_segment_contents("LX", &contents));
        println!("LX segment parsed");
        contents = content_trim("LX",contents);
    }
    if contents.contains("TS3") {
        print!("TS3 segment found, ");
        ts3_segments = get_ts3(get_segment_contents("TS3", &contents));
        println!("TS3 segment parsed");
        contents = content_trim("TS3",contents);
    }
    if contents.contains("TS2") {
        print!("TS2 segment found, ");
        ts2_segments = get_ts2(get_segment_contents("TS2", &contents));
        println!("TS2 segment parsed");
        contents = content_trim("TS2",contents);
    }

    println!("Loop 2000 parsed\n");
    return (lx_segments, ts3_segments, ts2_segments, contents)
}

fn get_loop_2100(mut contents:String) -> (CLP, CAS, NM1, NM1, NM1, NM1, NM1, NM1, NM1, MIA, MOA, REF, REF, DTM, DTM, DTM, PER, AMT, QTY, String) {
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

fn get_loop_2110(mut contents: String) -> (SVC, DTM, CAS, REF, REF, REF, REF, AMT, QTY, LQ, String) {
    
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

fn get_table_3(mut contents: String) -> (PLB, SE, String) {
    // Table 3
    // PLB Provider Adjustment S >1
    // SE Transaction Set Trailer R 1

    let mut plb_segments = PLB::default();
    let mut se_segments = SE::default();

    if contents.contains("PLB") {
        print!("PLB segment found, ");
        plb_segments = get_plb(get_segment_contents("PLB", &contents));
        println!("PLB segment parsed");
        contents = content_trim("PLB",contents);
    }
    if contents.contains("SE") {
        print!("SE segment found, ");
        se_segments = get_se(get_segment_contents("SE", &contents));
        println!("SE segment parsed");
        contents = content_trim("SE",contents);
    }

    println!("Table 3 parsed\n");

    return (plb_segments, se_segments, contents)
}

fn get_interchange_control_trailer(mut contents: String) -> (GE, IEA, String) {

    // Interchange Control Trailer
    // IEA Interchange Control Trailer R 1
    // GE FUNCTIONAL GROUP TRAILER R 1

    let mut iea_segments = IEA::default();
    let mut ge_segments = GE::default();

    if contents.contains("GE") {
        print!("GE segment found, ");
        ge_segments = get_ge(get_segment_contents("GE", &contents));
        println!("GE segment parsed");
        contents = content_trim("GE",contents);
    }

    if contents.contains("IEA") {
        print!("IEA segment found, ");
        iea_segments = get_iea(get_segment_contents("IEA", &contents));
        println!("IEA segment parsed");
        contents = content_trim("IEA",contents);
    }

    println!("Interchange Control Trailer parsed\n");

    return (ge_segments,iea_segments, contents)
}

fn main() {
    // Open File and read content
    let mut file = File::open("./data/X221-claim-specific-negotiated-discount.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    /*
    2 ideas if the assumption of order is correct then we can trim the contents as we go
    the 2nd idea safer and need to "extract" the data from the overall string then merge the parts back to one string
    yup idea 1 didn't work since there might be some loops I didn't count for so the trim start function is not finding
    the passed in string at the start
    */


    /*
    TODO:
        Figure out the looping of the segments,
        Will ask EDI SMEs what is the best way to figure this out, my first idea is to count the elements and see how many 
        of a specific unique segment in specific loop is coming required is perferred but some loops are all sitiuational, eg svc in loop 2110.
        loop 1000 A and B don't repeat
        loop 2000 have no required segments but LX is there in all my test files
        loop 2100 can use CLP
        loop 2110 have no required segments but can use SVC 
        if no SVC after MN1 in the previous loop '~' and this new loop have SE then it's table 3
        if there's something else not SE then we are in either another loop of 2100 (check for CLP) 
        if all this fails then SVC is not a good indicator
    */


    // Control Segments
    let (_isa, _gs, contents) = get_interchange_control(contents.clone());

    // Table 1
    let (_st, _bpr, _trn, _cur, _ref, _dtm, contents) = get_first_table_header(contents.clone());

    // Loop 1000A Payer Identification
    let (_n1, _n3, _n4, _per, contents) = get_loop_1000_a(contents.clone());

    // Loop 1000B Payee Identification
    let (_n1, _n3, _n4, _ref, _rdm, contents) = get_loop_1000_b(contents.clone());


    // Loop 2000 Header Number
    let (_lx, _ts3, _ts2, contents) = get_loop_2000(contents.clone());

    
    // Loop 2100 Claim Payment Information 
    let (_clp, _cas, _nm1_patient, _nm1_insured, _nm1_corrected_patient, _nm1_service_provider, _nm1_crossover_carrier, _nm1_corrected_priority_payer,
        _nm1_other_subscriber, _mia, _moa, _ref_other_claim, _ref_rendering_provider, _dtm_statement_from, _dtm_coverage_expiration, _dtm_claim_received,
        _per, _amt, _qty, contents) = get_loop_2100(contents.clone());
        
    // Loop 2110 Service Payment Information
    let (_svc, _dtm, _cas, _ref_service_identification, _ref_line_item_control_number, _ref_rendering_provider_information, _ref_healthcare_policy_identification, _amt, _qty, _lq, contents) =
    get_loop_2110(contents.clone());
    
        
    // Table 3
    let (_plb, _se, contents) = get_table_3(contents.clone());
    
    // Control Segments
    let (_ge, _iea, _contents) = get_interchange_control_trailer(contents.clone());
        
    println!("{:?}", _contents);
}