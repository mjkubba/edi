use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ik3::*;
// use crate::edi999::loop2110::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2100 {
    pub ik3_segments: IK3,
}


pub fn get_loop_2100(mut contents:String) -> (IK3, String) {

    let mut ik3_segments = IK3::default();
  

    if contents.contains("IK3") {
        info!("IK3 segment found, ");
        ik3_segments = get_ik3(get_segment_contents("IK3", &contents));
        info!("IK3 segment parsed");
        contents = content_trim("IK3",contents);
    }


    info!("Loop 2100 parsed\n");
    return (ik3_segments, contents)
}

pub fn get_loop_2100s(mut contents: String) ->  (Vec<Loop2100>, String) {

    let ik3_count= contents.matches("IK3").count();
    let mut loop_2100_array = vec![];
    info!("Number of loops in loop 2100: {:?}",ik3_count);



    for _ in 0..ik3_count {
        let ik3;
        (ik3, contents) = get_loop_2100(contents.clone());
        // (loop2100s, contents) = get_loop_2110s(contents.clone());

        let loop2100s = Loop2100 {
            ik3_segments: ik3,
        };

        loop_2100_array.push(loop2100s);
    }

    return (loop_2100_array, contents)
}

pub fn write_loop2100(loop2100:Vec<Loop2100>) -> String {
    let mut contents = String::new();
    for loop2100 in loop2100 {
        contents.push_str(&write_ik3(loop2100.ik3_segments));
    }
    return contents;
}



// unit tests

// #[cfg(test)]

// mod tests {
//     use super::*;
    
//     #[test]
//     fn test_get_loop_2000() {
//         let contents = String::from("LX*1~TS3*6543210903*11*20021231*1*211366.97********138018.4**73348.57~TS2*2178.45*1919.71**56.82*197.69*4.23~CLP*EXAMPLE3*2*500*100**12*05090256390*11*1~CAS*OA*23*600**94*-200~");
//         let (lx_segments, ts3_segments, ts2_segments, contents) = get_loop_2000(contents);
//         assert_eq!(lx_segments.lx01_claim_sequence_number, "1");
//         assert_eq!(contents, "CLP*EXAMPLE3*2*500*100**12*05090256390*11*1~CAS*OA*23*600**94*-200~");
//         assert_eq!(ts2_segments.ts201_total_drg_amount, "2178.45");
//         assert_eq!(ts3_segments.ts301_provider_identifier, "6543210903");
//     }
    
// }
