use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::plb::*;
use crate::segments::se::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct Table3s {
    pub plb_segments: Vec<PLB>,
    pub se_segments: SE,
}


pub fn get_table_3(mut contents: String) -> (Vec<PLB>, SE, String) {
    // Table 3
    // PLB Provider Adjustment S >1
    // SE Transaction Set Trailer R 1

    let mut plb_segments = vec![];
    let mut se_segments = SE::default();

    if contents.contains("PLB") {
        let plb_count = contents.matches("PLB").count();
        for _ in 0..plb_count {
            info!("PLB segment found, ");
            plb_segments.push(get_plb(get_segment_contents("PLB", &contents)));
            info!("PLB segment parsed");
            contents = content_trim("PLB",contents);

        }
    }

    if contents.contains("SE") {
        info!("SE segment found, ");
        se_segments = get_se(get_segment_contents("SE", &contents));
        info!("SE segment parsed");
        contents = content_trim("SE",contents);
    }

    info!("Table 3 parsed\n");

    return (plb_segments, se_segments, contents)
}

pub fn get_table3s(contents:String) -> (Table3s, String) {
    let (plb_segments, se_segments, contents) = get_table_3(contents);
    let header = Table3s {
        plb_segments,
        se_segments,
    };
    return (header, contents)
}

pub fn write_table3(table3:Table3s) -> String {
    let mut contents = String::new();
    for n in 0..table3.plb_segments.len() {
        contents.push_str(&write_plb(table3.plb_segments[n].clone()));
    }
    contents.push_str(&write_se(table3.se_segments));
    return contents
}



// unit tests

#[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_table3() {
            let contents = String::from("~SE*22*35681~GE*1*1~IEA*1*000000905~");
            let (plb_segments, se_segments, contents) = get_table_3(contents);
            assert_eq!(contents, "GE*1*1~IEA*1*000000905~");
            assert_eq!(plb_segments, vec![]);
            assert_eq!(se_segments.number_of_segment, "22");
        }
    }