use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::ik4::*;
use crate::segments::ctx::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Loop2110 {
    pub ik4_segments: IK4,
    pub ctx_element_context: CTX,
}


pub fn get_loop_2110(mut contents:String) -> (IK4, CTX, String) {

    let mut ik4_segments = IK4::default();
    let mut ctx_element_context = CTX::default();
  

    if contents.contains("IK4") {
        info!("IK4 segment found, ");
        ik4_segments = get_ik4(get_segment_contents("IK4", &contents));
        info!("IK4 segment parsed");
        contents = content_trim("IK4",contents);
    
    }

    if contents.contains("CTX") {
        info!("CTX segment found, ");
        ctx_element_context = get_ctx(get_segment_contents("CTX", &contents));
        info!("CTX segment parsed");
        contents = content_trim("CTX", contents);
    }
        

    info!("Loop 2110 parsed\n");
    return (ik4_segments, ctx_element_context, contents)
}

pub fn get_loop_2110s(mut contents: String) ->  (Vec<Loop2110>, String) {

    let ik4_count= contents.matches("IK4").count();
    let mut loop_2110_array = vec![];
    info!("Number of loops in loop 2110: {:?}",ik4_count);


    for _ in 0..ik4_count {
        let (ik4, ctx_element_context);
        (ik4, ctx_element_context, contents) = get_loop_2110(contents.clone());
        // (loop2100s, contents) = get_loop_2110s(contents.clone());

        let loop2110s = Loop2110 {
            ik4_segments: ik4,
            ctx_element_context,
        };

        loop_2110_array.push(loop2110s);
    }

    return (loop_2110_array, contents)
}

pub fn write_loop2110(loop2110:Vec<Loop2110>) -> String {
    let mut contents = String::new();
    for loop2110 in loop2110 {
        contents.push_str(&write_ik4(loop2110.ik4_segments));
        contents.push_str(&write_ctx(loop2110.ctx_element_context));
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
