use std::fs::File;
use std::io::Read;

mod edi835;
use edi835::{amt::*,bpr::*,cas::*,clp::*,cur::*,dtm::*,ge::*,gs::*,iea::*,isa::*,lx::*,n1::*,n3::*,n4::*,nm1::*,per::*,r#ref::*,rdm::*,st::*,se::*,trn::*};


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
        let _rdm_segments = get_rdm(get_segment_contents("RDM", &contents));
        println!("RDM segment parsed");
        contents = content_trim("RDM",contents);
    }
    return (n1_segments, n3_segments, n4_segments, ref_segments, rdm_segments, contents)
}

// fn get_loop_2000(mut contents:String) -> (LX, TS3, TS2, String) {
//     // Table 2 
//     // Loop 2000 Header Number (>1)
//     // LX Header Number S 1
//     // TS3 Provider Summary Information S 1
//     // TS2 Provider Supplemental Summary Information S 1
//     // Optional LX(1), TS3(1), TS2(1)

//     if contents.contains("LX") {
//         print!("LX segment found, ");
//         let _lx_segments = get_lx(get_segment_contents("LX", &contents));
//         println!("LX segment parsed");
//     }

// }

// make main smol again!
fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // 2 ideas if the assumption of order is correct then we can trim the contents as we go
    // the 2nd idea safer and need to "extract" the data from the overall string then merge the parts back to one string

    // Control Segments
    let (_isa, _gs, contents) = get_interchange_control(contents.clone());

    // Table 1
    let (_st, _bpr, _trn, _cur, _ref, _dtm, contents) = get_first_table_header(contents.clone());

    // Loop 1000A Payer Identification
    let (_n1, _n3, _n4, _per, contents) = get_loop_1000_a(contents.clone());

    // Loop 1000B Payee Identification
    let (_n1, _n3, _n4, _ref, _rdm, contents) = get_loop_1000_b(contents.clone());



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


    if contents.contains("CLP") {
        print!("CLP segment found");
        let clp_segments = get_clp(get_segment_contents("CLP", &contents));
        println!("{:?}", clp_segments);
        println!("CLP segment parsed");
    }

    
    if contents.contains("CAS") {
        print!("CAS segment found");
        let cas_segments = get_cas(get_segment_contents("CAS", &contents));
        println!("{:?}", cas_segments);
        println!("CAS segment parsed");
    }


    if contents.contains("NM1") {
        print!("NM1 segment found");
        let nm1_segments = get_nm1(get_segment_contents("NM1", &contents));
        println!("{:?}", nm1_segments);
        println!("NM1 segment parsed");
    }

    // if contents.contains("NM1") {
    //     // find how many nm1 segments are in the file
    //     let nm1_count= contents.matches("NM1").count();
    //     println!("Number of NM1 segments: {}", nm1_count);

    //     let mut next_segment =  &contents[contents.find("NM1").unwrap()..];
    //     let mut nm1_vec = Vec::new();

    //     for _ in 0..nm1_count {
    //         let nm1_start = next_segment;
    //         let nm1_end = nm1_start.find("~").unwrap();
    //         let nm1_content = &nm1_start[4..nm1_end];
    //         let nm1_segments = get_nm1(nm1_content);
    //         nm1_vec.push(nm1_segments);

    //         next_segment = &nm1_start[nm1_end+1..]
    //     }
    //     println!("{:?}", nm1_vec);
    //     println!("NM1 segment parsed");
    //     println!("\n");
    // }

    if contents.contains("AMT") {
        println!("AMT segment found");
        let amt_segments = get_amt(get_segment_contents("AMT", &contents));
        println!("{:?}", amt_segments);
        println!("AMT segment parsed");
        println!("\n");
    }

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


    // Table 3
    // PLB Provider Adjustment S >1
    // SE Transaction Set Trailer R 1

    if contents.contains("SE") {
        println!("SE segment found");
        let se_segments = get_se(get_segment_contents("SE", &contents));
        println!("{:?}", se_segments);
        println!("SE segment parsed");
        println!("\n");
    }

    // Control Segments

    if contents.contains("GE") {
        println!("GE segment found");
        println!("{:?}", get_segment_contents("GE", &contents));
        let ge_segments = get_ge(get_segment_contents("GE", &contents));
        println!("{:?}", ge_segments);
        println!("GE segment parsed");
        println!("\n");
    
    }

    if contents.contains("IEA") {
        println!("IEA segment found");
        let iea_segments = get_iea(get_segment_contents("IEA", &contents));
        println!("{:?}", iea_segments);
        println!("IEA segment parsed");
        println!("\n");
    }

}