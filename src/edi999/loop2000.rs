use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ak2::*;
use crate::segments::ik5::*;
use crate::edi999::loop2100::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2000 {
    pub ak2_segments: AK2,
    pub ik5_segments: IK5,
    pub loop2100: Vec<Loop2100>,
}


pub fn get_loop_2000(mut contents:String) -> (AK2, IK5, String) {

    let mut ak2_segments = AK2::default();
    let mut ik5_segments = IK5::default();
  

    if contents.contains("AK2") {
        info!("AK2 segment found, ");
        ak2_segments = get_ak2(get_segment_contents("AK2", &contents));
        info!("AK2 segment parsed");
        contents = content_trim("AK2",contents);
    }
    if contents.contains("IK5") {
        info!("IK5 segment found, ");
        ik5_segments = get_ik5(get_segment_contents("IK5", &contents));
        info!("IK5 segment parsed");
        contents = content_trim("IK5",contents);
    }


    info!("Loop 2000 parsed\n");
    return (ak2_segments, ik5_segments, contents)
}

pub fn get_loop_2000s(mut contents: String) ->  (Vec<Loop2000>, String) {

    let ak2_count= contents.matches("AK2").count();
    let mut loop_2000_array = vec![];
    info!("Number of loops in loop 2000: {:?}",ak2_count);



    for _ in 0..ak2_count {
        let tmp_contents = get_999_2000(contents.clone());
        let (ak2, ik5, loop2100, rem_contents, inner_rem_contents);

        (ak2, ik5, rem_contents) = get_loop_2000(tmp_contents.clone());
        (loop2100, inner_rem_contents) = get_loop_2100s(rem_contents.clone());

        let loop2000s = Loop2000 {
            ak2_segments: ak2,
            loop2100,
            ik5_segments: ik5,
        };

        contents = contents.replacen(&tmp_contents, "",1);
        contents.push_str(&inner_rem_contents);

        loop_2000_array.push(loop2000s);
    }

    return (loop_2000_array, contents)
}

pub fn write_loop2000(loop2000:Vec<Loop2000>) -> String {
    let mut contents = String::new();
    for loop2000 in loop2000 {
        contents.push_str(&write_ak2(loop2000.ak2_segments));
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
