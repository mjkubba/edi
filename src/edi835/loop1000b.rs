use crate::segments::n1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::r#ref::*;
use crate::segments::rdm::*;
use crate::helper::helper::*;

pub fn get_loop_1000_b(mut contents:String) -> (N1, N3, N4, REF, RDM, String) {
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
