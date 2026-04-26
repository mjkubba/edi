use log::info;
use serde::{Deserialize, Serialize};

use crate::helper::edihelper::*;
use crate::segments::ak9::*;
use crate::segments::se::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Table1trailer {
    pub se_segments: SE,
    pub ak9_segments: AK9,
}

pub fn get_first_table_trailer(mut contents: String) -> (SE, AK9, String) {
    let mut se_segments = SE::default();
    let mut ak9_segments = AK9::default();

    // Process AK9 segment (required — comes before SE per spec)
    if contents.contains("AK9") {
        info!("AK9 segment found");
        let ak9_content = get_segment_contents("AK9", &contents);
        ak9_segments = get_ak9(ak9_content);
        info!("AK9 segment parsed");
        contents = content_trim("AK9", contents);
    } else {
        info!("Warning: Required AK9 segment not found");
    }

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

    // Write AK9 segment (comes before SE per spec)
    contents.push_str(&write_ak9(table1trailer.ak9_segments.clone()));

    // Write SE segment
    contents.push_str(&write_se(table1trailer.se_segments.clone()));

    contents
}
