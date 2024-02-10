use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::lx::*;
use crate::segments::ts3::*;
use crate::segments::ts2::*;
use crate::edi835::loop2100::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2000s {
    pub lx_segments: LX,
    pub ts3_segments: TS3,
    pub ts2_segments: TS2,
    pub loop2100s: Vec<Loop2100s>,
}


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
        info!("LX segment found, ");
        lx_segments = get_lx(get_segment_contents("LX", &contents));
        info!("LX segment parsed");
        contents = content_trim("LX",contents);
    }
    if contents.contains("TS3") {
        info!("TS3 segment found, ");
        ts3_segments = get_ts3(get_segment_contents("TS3", &contents));
        info!("TS3 segment parsed");
        contents = content_trim("TS3",contents);
    }
    if contents.contains("TS2") {
        info!("TS2 segment found, ");
        ts2_segments = get_ts2(get_segment_contents("TS2", &contents));
        info!("TS2 segment parsed");
        contents = content_trim("TS2",contents);
    }

    info!("Loop 2000 parsed\n");
    return (lx_segments, ts3_segments, ts2_segments, contents)
}

pub fn get_loop_2000s(mut contents: String) ->  (Vec<Loop2000s>, String) {

    let lx_count= contents.matches("LX").count();

    /* 
    Check if there are multiple LX, TS3, TS2 segments then make that the loop,
    Get the content between LX and the other LX

    else
    Check for CLP and get the content between CLP and the other CLP
    */

    let mut loop_2000_array = vec![];
    info!("Number of loops in loop 2000: {:?}",lx_count);



    for _ in 0..lx_count {
        // let tmp_contents = get_loop_contents("LX", "CLP", contents.clone());
        let (lx, ts3, ts2, loop2100s);
        (lx, ts3, ts2, contents) = get_loop_2000(contents.clone());
        (loop2100s, contents) = get_loop_2100s(contents.clone());

        let loop2000s = Loop2000s {
            lx_segments: lx,
            ts3_segments: ts3,
            ts2_segments: ts2,
            loop2100s,
        };

        
        // contents = contents.replacen(&tmp_contents, "",1);
        // println!("contents after: {:?}",contents);
        // contents.push_str(&rem_contents);        
        loop_2000_array.push(loop2000s);
    }

    return (loop_2000_array, contents)
}

pub fn write_loop2000(loop2000:Vec<Loop2000s>) -> String {
    let mut contents = String::new();
    for loop2000 in loop2000 {
        contents.push_str(&write_lx(loop2000.lx_segments));
        contents.push_str(&write_ts3(loop2000.ts3_segments));
        contents.push_str(&write_ts2(loop2000.ts2_segments));
        contents.push_str(&write_loop2100(loop2000.loop2100s));
        // for loop2100 in loop2000.loop2100s {
        // }

    }
    return contents;
}



// unit tests

#[cfg(test)]

mod tests {
    use super::*;
    
    #[test]
    fn test_get_loop_2000() {
        let contents = String::from("LX*1~TS3*6543210903*11*20021231*1*211366.97********138018.4**73348.57~TS2*2178.45*1919.71**56.82*197.69*4.23~CLP*EXAMPLE3*2*500*100**12*05090256390*11*1~CAS*OA*23*600**94*-200~");
        let (lx_segments, ts3_segments, ts2_segments, contents) = get_loop_2000(contents);
        assert_eq!(lx_segments.lx01_claim_sequence_number, "1");
        assert_eq!(contents, "CLP*EXAMPLE3*2*500*100**12*05090256390*11*1~CAS*OA*23*600**94*-200~");
        assert_eq!(ts2_segments.ts201_total_drg_amount, "2178.45");
        assert_eq!(ts3_segments.ts301_provider_identifier, "6543210903");
    }
    
}
