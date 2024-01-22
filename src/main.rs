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