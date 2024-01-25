use std::fs::File;
use std::io::Read;

mod edi835;
use edi835::{interchangecontrol::*,table1::*,loop1000a::*,loop1000b::*,loop2000::*,loop2100::*,loop2110::*,table3::*,interchangecontroltrailer::*};

mod helper;
mod segments;


fn main() {

    // Open File and read content
    let mut file = File::open("./data/X221-claim-specific-negotiated-discount.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    /*
    TODO:
        2 ideas:
        if the assumption of order is correct then we can trim the contents as we go.
        the 2nd idea safer and need to "extract" the data from the overall string then merge the parts back to one string
        yup idea 1 didn't work since there might be some loops I didn't count for so the trim start function is not finding
        the passed in string at the start.
        ok that was easy, use replace(string, "")

        Figure out the looping of the segments,
        Will ask EDI SMEs what is the best way to figure this out, my first idea is to count the elements and see how many 
        of a specific unique segment in specific loop is coming required is preferred but some loops are all situational, eg svc in loop 2110.
        loop 1000 A and B don't repeat
        loop 2000 have no required segments but LX is there in all my test files
        loop 2100 can use CLP
        loop 2110 have no required segments but can use SVC 
        if no SVC after MN1 in the previous loop '~' and this new loop have SE then it's table 3
        if there's something else not SE then we are in either another loop of 2100 (check for CLP) 
        if all this fails then SVC is not a good indicator

        println statement is in helper.rs line 3
    */


    // Control Segments
    let (_isa, _gs, contents) = get_interchange_control(contents.clone());

    // Table 1
    let (_st, _bpr, _trn, _cur, _ref, _dtm, contents) = get_first_table_header(contents.clone());

    // Loop 1000A Payer Identification
    let (_n1, _n3, _n4, _per, contents) = get_loop_1000_a(contents.clone());

    // Loop 1000B Payee Identification
    let (_n1, _n3, _n4, _ref, _rdm, mut contents) = get_loop_1000_b(contents.clone());


    // Loop 2000 Header Number
    let lx_count= contents.matches("LX").count();
    let mut loop_2000_array = vec![];
    println!("Number of loops in loop 2000: {:?}",lx_count);
    let mut lx;
    let mut ts2;
    let mut ts3;

    for _ in 0..lx_count {
        (lx, ts3, ts2, contents) = get_loop_2000(contents.clone());
        let _loop2000 = get_loop_2000s(lx,ts3,ts2);
        loop_2000_array.push(_loop2000);
    }


    
    // Loop 2100 Claim Payment Information 
    let clp_count= contents.matches("CLP").count();
    let mut loop_2100_array = vec![];
    println!("Number of loops in loop 2100: {:?}",clp_count);
    let mut clp;
    let mut claim_adjustment;
    let mut nm1_patient;
    let mut nm1_insured;
    let mut nm1_corrected_patient;
    let mut nm1_service_provider;
    let mut nm1_crossover_carrier;
    let mut nm1_corrected_priority_payer;
    let mut nm1_other_subscriber;
    let mut mia;
    let mut moa;
    let mut ref_other_claim;
    let mut ref_rendering_provider;
    let mut dtm_statement_from;
    let mut dtm_coverage_expiration;
    let mut dtm_claim_received;
    let mut per;
    let mut amt;
    let mut qty;

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
    println!("Number of loops in loop 2100: {:?}",svc_count);
    let mut svc;
    let mut dtm;
    let mut cas;
    let mut ref_service_identification;
    let mut ref_line_item_control_number;
    let mut ref_rendering_provider_information;
    let mut ref_healthcare_policy_identification;
    let mut amt;
    let mut qty;
    let mut lq;

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