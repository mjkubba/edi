use crate::segments::plb::*;
use crate::segments::se::*;
use crate::helper::helper::*;

pub fn get_table_3(mut contents: String) -> (PLB, SE, String) {
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
