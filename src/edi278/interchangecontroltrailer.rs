use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::iea::*;
use crate::segments::ge::*;
use crate::segments::se::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub se_segments: SE,
    pub ge_segments: GE,
    pub iea_segments: IEA,
}

pub fn get_interchange_trailer(mut contents: String) -> (InterchangeTrailer, String) {
    let mut se_segments = SE::default();
    let mut ge_segments = GE::default();
    let mut iea_segments = IEA::default();
    
    if contents.contains("SE") {
        info!("SE segment found, ");
        se_segments = get_se(get_segment_contents("SE", &contents));
        info!("SE segment parsed");

        contents = content_trim("SE", contents);
    }
    
    if contents.contains("GE") {
        info!("GE segment found, ");
        ge_segments = get_ge(get_segment_contents("GE", &contents));
        info!("GE segment parsed");

        contents = content_trim("GE", contents);
    }
    
    if contents.contains("IEA") {
        info!("IEA segment found, ");
        iea_segments = get_iea(get_segment_contents("IEA", &contents));
        info!("IEA segment parsed");

        contents = content_trim("IEA", contents);
    }
    
    info!("Interchange Trailer parsed\n");
    
    let trailer = InterchangeTrailer {
        se_segments,
        ge_segments,
        iea_segments,
    };
    
    return (trailer, contents)
}

pub fn write_interchange_trailer(trailer: InterchangeTrailer) -> String {
    let mut contents = String::new();
    contents.push_str(&write_se(trailer.se_segments));
    contents.push_str(&write_ge(trailer.ge_segments));
    contents.push_str(&write_iea(trailer.iea_segments));
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_interchange_trailer() {
        let contents = String::from("SE*16*0001~GE*1*1~IEA*1*000000001~");
        let (trailer, contents) = get_interchange_trailer(contents);
        assert_eq!(trailer.se_segments.number_of_segment, "16");
        assert_eq!(trailer.se_segments.transaction_set_control_number, "0001");
        assert_eq!(trailer.ge_segments.number_of_transitions, "1");
        assert_eq!(trailer.ge_segments.group_control_number, "1");
        assert_eq!(trailer.iea_segments.number_of_included_group, "1");
        assert_eq!(trailer.iea_segments.interchange_control_number, "000000001");
        assert_eq!(contents, "");
    }
}
