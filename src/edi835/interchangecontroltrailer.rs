use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::iea::*;
use crate::segments::ge::*;
use crate::helper::helper::*;

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub ge_segments: GE,
    pub iea_segments: IEA,
}


pub fn get_interchange_control_trailer(contents: String) -> (GE, IEA) {

    // Interchange Control Trailer
    // IEA Interchange Control Trailer R 1
    // GE FUNCTIONAL GROUP TRAILER R 1

    let mut iea_segments = IEA::default();
    let mut ge_segments = GE::default();

    if contents.contains("GE") {
        info!("GE segment found, ");
        ge_segments = get_ge(get_segment_contents("GE", &contents));
        info!("GE segment parsed");
    }

    if contents.contains("IEA") {
        info!("IEA segment found, ");
        iea_segments = get_iea(get_segment_contents("IEA", &contents));
        info!("IEA segment parsed");
    }

    info!("Interchange Control Trailer parsed\n");

    return (ge_segments,iea_segments)
}

pub fn get_interchange_trailer(mut contents:String) -> (InterchangeTrailer, String) {
    let loop_content = get_loop_content(contents.clone(), "GE", "~~");
    let (ge_segments,iea_segments) = get_interchange_control_trailer(loop_content.clone());
    contents = contents.replace(&loop_content, "");
    let header = InterchangeTrailer {
        ge_segments,
        iea_segments,
    };
    return (header, contents)
}

pub fn write_interchange_trailer(trailer:InterchangeTrailer) -> String {
    let mut contents = String::new();
    contents.push_str(&write_ge(trailer.ge_segments));
    contents.push_str(&write_iea(trailer.iea_segments));
    return contents
}

// unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interchange_control_trailer() {
        let contents = String::from("GE*1*1~IEA*1*000000905~");
        let (ge_segments, iea_segments, contents) = get_interchange_control_trailer(contents);
        assert_eq!(ge_segments.group_control_number, "1");
        assert_eq!(iea_segments.interchange_control_number, "000000905");
        assert_eq!(contents, "");
    }

}