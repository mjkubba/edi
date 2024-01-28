use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::n1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::segments::r#ref::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop1000as {
    pub n1_segments: N1,
    pub n3_segments: N3,
    pub n4_segments: N4,
    pub ref_segments: REF,
    pub per_payer_business: PER,
    pub per_technical_contact: PER,
    pub per_web_site: PER,
}

pub fn get_loop_1000_a(mut contents:String) -> (N1, N3, N4, REF, PER, PER, PER, String) {
    
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
    let mut ref_segments = REF::default();
    let mut per_payer_business = PER::default();
    let mut per_technical_contact = PER::default();
    let mut per_web_site = PER::default();

    if contents.contains("N1") {
        info!("N1 segment found, ");
        n1_segments = get_n1(get_segment_contents("N1", &contents));
        info!("N1 segment parsed");
        contents = content_trim("N1",contents);
    } 

    if contents.contains("N3") {
        info!("N3 segment found, ");
        n3_segments = get_n3(get_segment_contents("N3", &contents));
        info!("N3 segment parsedm");
        contents = content_trim("N3",contents);
    }

    if contents.contains("N4") {
        info!("N4 segment found, ");
        n4_segments = get_n4(get_segment_contents("N4", &contents));
        info!("N4 segment parsed"); 
        contents = content_trim("N4",contents);
    }

    if contents.contains("REF") {
        info!("REF segment found, ");
        ref_segments = get_ref(get_segment_contents("REF", &contents));
        info!("REF segment parsed");
        contents = content_trim("REF",contents);
    }

    if contents.contains("PER") {
        info!("PER segment found, ");
        let per_segment = get_per(get_segment_contents("PER", &contents));
        match &per_technical_contact.per01_contact_function_code as &str{
            "CX" => {
                per_payer_business = per_segment.clone();
            },
            "BL" => {
                per_technical_contact = per_segment.clone();
            },
            "IC" => {
                per_web_site = per_segment.clone();
            },
            _ => {
                per_technical_contact = per_segment.clone();
            }
        }
        info!("PER segment parsed");
        contents = content_trim("PER",contents);
    }

    if contents.contains("PER") {
        info!("PER segment found, ");
        let per_segment = get_per(get_segment_contents("PER", &contents));
        match &per_technical_contact.per01_contact_function_code as &str{
            "CX" => {
                per_payer_business = per_segment.clone();
            },
            "BL" => {
                per_technical_contact = per_segment.clone();
            },
            "IC" => {
                per_web_site = per_segment.clone();
            },
            _ => {
                per_technical_contact = per_segment.clone();
            }
        }
        info!("PER segment parsed");
        contents = content_trim("PER",contents);
    }

    if contents.contains("PER") {
        info!("PER segment found, ");
        let per_segment = get_per(get_segment_contents("PER", &contents));
        match &per_technical_contact.per01_contact_function_code as &str{
            "CX" => {
                per_payer_business = per_segment.clone();
            },
            "BL" => {
                per_technical_contact = per_segment.clone();
            },
            "IC" => {
                per_web_site = per_segment.clone();
            },
            _ => {
                per_technical_contact = per_segment.clone();
            }
        }
        info!("PER segment parsed");
        contents = content_trim("PER",contents);
    }


    info!("Loop 1000A parsed\n");
    return (n1_segments, n3_segments, n4_segments, ref_segments, per_technical_contact, per_payer_business, per_web_site, contents)
}

pub fn get_1000as(contents:String) -> (Loop1000as, String) {
    let (n1_segments, n3_segments, n4_segments, ref_segments, per_technical_contact, per_payer_business, per_web_site, contents) = get_loop_1000_a(contents);
    let header  = Loop1000as {
        n1_segments,
        n3_segments,
        n4_segments,
        ref_segments,
        per_payer_business,
        per_technical_contact,
        per_web_site,
    };
    return (header,contents)
}

// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop1000a() {
        let contents = String::from("N1*PR*DELTA DENTAL OF ABC~N3*225 MAIN STREET~N4*CENTERVILLE*PA*17111~PER*BL*JANE DOE*TE*9005555555~");
        let (n1_segments, n3_segments, n4_segments,  ref_segments, per_technical_contact, per_payer_business, per_web_site, contents) = get_loop_1000_a(contents);
        assert_eq!(n1_segments.payer_id_code, "PR");
        assert_eq!(n1_segments.payee_name, "DELTA DENTAL OF ABC");
        assert_eq!(n3_segments.payee_address, "225 MAIN STREET");
        assert_eq!(n4_segments.payee_city, "CENTERVILLE");
        assert_eq!(n4_segments.payee_state, "PA");
        assert_eq!(n4_segments.payee_zip, "17111");
        assert_eq!(per_technical_contact.per01_contact_function_code, "BL");
        assert_eq!(per_technical_contact.per02_contact_name, "JANE DOE");
        assert_eq!(per_payer_business, PER::default());
        assert_eq!(per_web_site, PER::default());
        assert_eq!(ref_segments, REF::default());
        assert_eq!(contents, "");
    }


}