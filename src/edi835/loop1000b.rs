use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::n1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::r#ref::*;
use crate::segments::rdm::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop1000bs {
    pub n1_segments: N1,
    pub n3_segments: N3,
    pub n4_segments: N4,
    pub ref_segments: Vec<REF>,
    pub rdm_segments: RDM,
}


pub fn get_loop_1000_b(mut contents:String) -> (N1, N3, N4, Vec<REF>, RDM, String) {
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
    let mut ref_segments = vec![];
    let mut rdm_segments = RDM::default();

    if contents.contains("N1") {
        info!("N1 segment found, ");
        n1_segments = get_n1(get_segment_contents("N1", &contents));
        info!("N1 segment parsed");
        contents = content_trim("N1",contents);
    }
    if contents.contains("N3") {
        info!("N3 segment found, ");
        n3_segments = get_n3(get_segment_contents("N3", &contents));
        info!("N3 segment parsed");
        contents = content_trim("N3",contents);
    }
    if contents.contains("N4") {
        info!("N4 segment found, ");
        n4_segments = get_n4(get_segment_contents("N4", &contents));
        info!("N4 segment parsed");
        contents = content_trim("N4",contents);
    }

    if contents.contains("REF") {
        let ref_count = contents.matches("REF").count();
        for _ in 0..ref_count {
            if check_if_segement_in_loop("REF", "CLP", contents.clone()) {
                info!("REF segment found, ");
                ref_segments.push(get_ref(get_segment_contents("REF", &contents)));
                info!("REF segment parsed");
                contents = content_trim("REF",contents);
            }
        }
    }
    if contents.contains("RDM") {
        info!("RDM segment found, ");
        rdm_segments = get_rdm(get_segment_contents("RDM", &contents));
        info!("RDM segment parsed");
        contents = content_trim("RDM",contents);
    }

    info!("Loop 1000B parsed\n");
    return (n1_segments, n3_segments, n4_segments, ref_segments, rdm_segments, contents)
}


pub fn get_1000bs(contents:String) -> (Loop1000bs, String) {
    let (n1_segments, n3_segments, n4_segments, ref_segments, rdm_segments, contents) = get_loop_1000_b(contents);
    let header  = Loop1000bs {
        n1_segments,
        n3_segments,
        n4_segments,
        ref_segments,
        rdm_segments,
    };
    return (header,contents)
}

pub fn write_loop1000b(loop1000b:Loop1000bs) -> String {
    let mut contents = String::new();
    contents.push_str(&write_n1(loop1000b.n1_segments));
    contents.push_str(&write_n3(loop1000b.n3_segments));
    contents.push_str(&write_n4(loop1000b.n4_segments));
    for n in 0..loop1000b.ref_segments.len() {
        contents.push_str(&write_ref(loop1000b.ref_segments[n].clone()));
    }
    contents.push_str(&write_rdm(loop1000b.rdm_segments));
    return contents;
}



// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop1000b() {
        let contents = String::from("N1*PE*PROVIDER*XX*1123454567~N3*2255 ANY ROAD~N4*ANY CITY*CA*12211~REF*TJ*123456789~");
        let (n1_segments, n3_segments, n4_segments, ref_segments, rdm_segments, contents) = get_loop_1000_b(contents.to_string());
        assert_eq!(n1_segments.payer_id_code, "PE");
        assert_eq!(n1_segments.payee_name, "PROVIDER");
        assert_eq!(n1_segments.payee_identification_code_qualifier, "XX");
        assert_eq!(n1_segments.payee_identification_code, "1123454567");
        assert_eq!(n3_segments.payee_address, "2255 ANY ROAD");
        assert_eq!(n4_segments.payee_city, "ANY CITY");
        assert_eq!(n4_segments.payee_state, "CA");
        assert_eq!(n4_segments.payee_zip, "12211");
        assert_eq!(ref_segments.reference_id_number_qualifier, "TJ");
        assert_eq!(ref_segments.reference_id_number, "123456789");
        assert_eq!(rdm_segments, RDM::default());
        assert_eq!(contents, "");
    }

    #[test]
    fn test_loop1000b2() {
        let contents = String::from("N1*PE*PROVIDER*XX*1123454567~N3*2255 ANY ROAD*unit 500~N4*ANY CITY*CA*12211~REF*TJ*123456789~RDM*BM~");
        let (_n1_segments, n3_segments, _n4_segments, _ref_segments, rdm_segments, contents) = get_loop_1000_b(contents.to_string());
        assert_eq!(n3_segments.payee_address, "2255 ANY ROAD");
        assert_eq!(n3_segments.payee_address2, "unit 500");
        assert_eq!(rdm_segments.rdm01_report_transmission_code, "BM");
        assert_eq!(contents, "");
    }
}