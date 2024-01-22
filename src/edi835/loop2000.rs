use crate::segments::lx::*;
use crate::segments::ts3::*;
use crate::segments::ts2::*;
use crate::helper::helper::*;


pub fn get_loop_2000(mut contents:String) -> (LX, TS3, TS2, String) {
    // Table 2 
    // Loop 2000 Header Number (>1)
    // LX Header Number S 1
    // TS3 Provider Summary Information S 1
    // TS2 Provider Supplemental Summary Information S 1
    // Optional LX(1), TS3(1), TS2(1)

    let mut lx_segments = LX::default();
    let mut ts3_segments = TS3::default();
    let mut ts2_segments = TS2::default();

    if contents.contains("LX") {
        print!("LX segment found, ");
        lx_segments = get_lx(get_segment_contents("LX", &contents));
        println!("LX segment parsed");
        contents = content_trim("LX",contents);
    }
    if contents.contains("TS3") {
        print!("TS3 segment found, ");
        ts3_segments = get_ts3(get_segment_contents("TS3", &contents));
        println!("TS3 segment parsed");
        contents = content_trim("TS3",contents);
    }
    if contents.contains("TS2") {
        print!("TS2 segment found, ");
        ts2_segments = get_ts2(get_segment_contents("TS2", &contents));
        println!("TS2 segment parsed");
        contents = content_trim("TS2",contents);
    }

    println!("Loop 2000 parsed\n");
    return (lx_segments, ts3_segments, ts2_segments, contents)
}