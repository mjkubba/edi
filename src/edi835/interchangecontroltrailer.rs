use crate::segments::iea::*;
use crate::segments::ge::*;
use crate::helper::helper::*;



pub fn get_interchange_control_trailer(mut contents: String) -> (GE, IEA, String) {

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