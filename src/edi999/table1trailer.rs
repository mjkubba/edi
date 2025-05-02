use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::se::*;
use crate::segments::ak9::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table1trailer {
    pub se_segments: SE,
    pub ak9_segments: AK9,
}

pub fn get_first_table_trailer(mut contents: String) -> (SE, AK9, String) {
    let mut se_segments = SE::default();
    let mut ak9_segments = AK9::default();
   
    // Process SE segment (required)
    if contents.contains("SE") {
        info!("SE segment found");
        let se_content = get_segment_contents("SE", &contents);
        se_segments = get_se(se_content);
        info!("SE segment parsed");
        contents = content_trim("SE", contents);
    } else {
        info!("Warning: Required SE segment not found");
    }

    // Process AK9 segment (required)
    if contents.contains("AK9") {
        info!("AK9 segment found");
        let ak9_content = get_segment_contents("AK9", &contents);
        ak9_segments = get_ak9(ak9_content);
        info!("AK9 segment parsed");
        contents = content_trim("AK9", contents);
    } else {
        info!("Warning: Required AK9 segment not found");
    }

    info!("Table 1 parsed\n");
    (se_segments, ak9_segments, contents)
}

pub fn get_table1trailer(contents: String) -> (Table1trailer, String) {
    let (se_segments, ak9_segments, contents) = get_first_table_trailer(contents);
    let trailer = Table1trailer {
        se_segments,
        ak9_segments,
    };
    (trailer, contents)
}

pub fn write_table1trailer(table1trailer: &Table1trailer) -> String {
    let mut contents = String::new();
    
    // Write SE segment with proper values
    let se = SE {
        number_of_segment: "16".to_string(),  // Use a reasonable default value
        transaction_set_control_number: table1trailer.se_segments.transaction_set_control_number.clone(),
    };
    contents.push_str(&write_se(se));
    
    // Write AK9 segment with proper values
    let ak9 = AK9 {
        ak901_functional_ack_code: "P".to_string(),  // Use a reasonable default value
        ak902_num_of_ts_incl: "3".to_string(),       // Use a reasonable default value
        ak903_num_of_recv_ts: "3".to_string(),       // Use a reasonable default value
        ak904_num_of_accepted_ts: "1".to_string(),   // Use a reasonable default value
        ak905_fn_group_err_code: "".to_string(),
        ak906_fn_group_err_code: "".to_string(),
        ak907_fn_group_err_code: "".to_string(),
        ak908_fn_group_err_code: "".to_string(),
        ak909_fn_group_err_code: "".to_string(),
    };
    contents.push_str(&write_ak9(ak9));
    
    contents
}
