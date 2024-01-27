use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;


mod edi835;
use edi835::{interchangecontrol::*,table1::*,loop1000a::*,loop1000b::*,loop2000::*,loop2100::*,loop2110::*,table3::*,interchangecontroltrailer::*};

mod helper;
mod segments;


fn main() {
    let mut file_path;
    // Open File and read content
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_some() {
        file_path = Path::new(&args[1]);
    } else {
        file_path = Path::new("./demo/edi835-1.edi");
    }

    if file_path.exists() {
        println!("File exists");
    } else {
        println!("File does not exist");
        println!("Loading default demo file edi835-1.edi");
        file_path = Path::new("./demo/edi835-1.edi");
    }
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    /*
    TODO:
        implement logger
        make it safer when something does not exist
        println statement is in helper.rs line 3


        Check against the guide how many of each segment is in each loop, 
        finding some mismatches between the standard and the implementation of EDI835!!!

        Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle
    */


    // Control Segments
    let (_isa, _gs, contents) = get_interchange_control(contents.clone());

    // Table 1
    let (_st, _bpr, _trn, _cur, _ref, _ref2, _dtm, contents) = get_first_table_header(contents.clone());

    // Loop 1000A Payer Identification
    let (_n1, _n3, _n4, _ref, _per, _per2, _per3, contents) = get_loop_1000_a(contents.clone());

    // Loop 1000B Payee Identification
    let (_n1, _n3, _n4, _ref, _rdm, mut contents) = get_loop_1000_b(contents.clone());


    // Loop 2000 Header Number
    let lx_count= contents.matches("LX").count();
    let mut loop_2000_array = vec![];
    println!("Number of loops in loop 2000: {:?}",lx_count);
    let (mut lx, mut ts3, mut ts2);

    for _ in 0..lx_count {
        (lx, ts3, ts2, contents) = get_loop_2000(contents.clone());
        let _loop2000 = get_loop_2000s(lx,ts3,ts2);
        loop_2000_array.push(_loop2000);
    }


    
    // Loop 2100 Claim Payment Information 
    let clp_count= contents.matches("CLP").count();
    let mut loop_2100_array = vec![];
    println!("Number of loops in loop 2100: {:?}",clp_count);
    let (mut clp, mut claim_adjustment, mut nm1_patient, mut nm1_insured, mut nm1_corrected_patient, mut nm1_service_provider, 
         mut nm1_crossover_carrier, mut nm1_corrected_priority_payer, mut nm1_other_subscriber, mut mia, mut moa, mut ref_other_claim, 
         mut ref_rendering_provider, mut dtm_statement_from, mut dtm_coverage_expiration, mut dtm_claim_received, mut per, mut amt, mut qty);

    for _ in 0..clp_count {
        (clp, claim_adjustment, nm1_patient, nm1_insured, nm1_corrected_patient, nm1_service_provider, nm1_crossover_carrier, nm1_corrected_priority_payer,
            nm1_other_subscriber, mia, moa, ref_other_claim, ref_rendering_provider, dtm_statement_from, dtm_coverage_expiration, dtm_claim_received,
            per, amt, qty, contents) = get_loop_2100(contents.clone());
        let _loop2100 = get_loop_2100s(clp,claim_adjustment,nm1_patient,nm1_insured,nm1_corrected_patient,nm1_service_provider,nm1_crossover_carrier,nm1_corrected_priority_payer,
            nm1_other_subscriber,mia,moa,ref_other_claim,ref_rendering_provider,dtm_statement_from,dtm_coverage_expiration,dtm_claim_received,
            per,amt,qty);
            loop_2100_array.push(_loop2100);
    }


    // Loop 2110 Service Payment Information
    let svc_count= contents.matches("SVC").count();
    let mut loop_2110_array = vec![];
    println!("Number of loops in loop 2110: {:?}",svc_count);
    let (mut svc, mut dtm, mut cas, mut ref_service_identification, mut ref_line_item_control_number, 
         mut ref_rendering_provider_information, mut ref_healthcare_policy_identification, mut amt, mut qty, mut lq);

    for _ in 0..svc_count {
        (svc, dtm, cas, ref_service_identification, ref_line_item_control_number, ref_rendering_provider_information, ref_healthcare_policy_identification, amt, qty, lq, contents) =
        get_loop_2110(contents.clone());
        let loop2110 = get_loop_2110s(svc,dtm,cas,ref_service_identification,ref_line_item_control_number,ref_rendering_provider_information,ref_healthcare_policy_identification,amt,qty,lq);
        loop_2110_array.push(loop2110);
    }

        
    // Table 3
    let (_plb, _se, contents) = get_table_3(contents.clone());
    
    // Control Segments
    let (_ge, _iea, _contents) = get_interchange_control_trailer(contents.clone());
        
    println!("{:?}", _contents);
}


// unit test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}