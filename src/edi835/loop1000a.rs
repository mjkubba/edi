use crate::segments::n1::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::per::*;
use crate::helper::helper::*;

pub fn get_loop_1000_a(mut contents:String) -> (N1, N3, N4, PER, String) {
    
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



// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop1000a() {
        let contents = String::from("N1*PR*DELTA DENTAL OF ABC~N3*225 MAIN STREET~N4*CENTERVILLE*PA*17111~PER*BL*JANE DOE*TE*9005555555~");
        let (n1_segments, n3_segments, n4_segments, per_segments, contents) = get_loop_1000_a(contents);
        assert_eq!(n1_segments.payer_id_code, "PR");
        assert_eq!(n1_segments.payee_name, "DELTA DENTAL OF ABC");
        assert_eq!(n3_segments.payee_address, "225 MAIN STREET");
        assert_eq!(n4_segments.payee_city, "CENTERVILLE");
        assert_eq!(n4_segments.payee_state, "PA");
        assert_eq!(n4_segments.payee_zip, "17111");
        assert_eq!(per_segments.per01_contact_function_code, "BL");
        assert_eq!(per_segments.per02_contact_name, "JANE DOE");
        assert_eq!(contents, "");
    }


}