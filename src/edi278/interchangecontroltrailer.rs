use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::se::*;
use crate::segments::ge::*;
use crate::segments::iea::*;
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
        info!("Warning: Required SE segment not found");
        se_segments = get_se(get_segment_contents("SE", &contents));
        info!("SE segment parsed");
        
        contents = content_trim("SE", contents);
    }
    
    if contents.contains("GE") {
        info!("Warning: Required GE segment not found");
        ge_segments = get_ge(get_segment_contents("GE", &contents));
        info!("GE segment parsed");
        
        contents = content_trim("GE", contents);
    }
    
    if contents.contains("IEA") {
        info!("Warning: Required IEA segment not found");
        iea_segments = get_iea(get_segment_contents("IEA", &contents));
        info!("IEA segment parsed");
        
        contents = content_trim("IEA", contents);
    }
    
    info!("Interchange Control Trailer parsed\n");
    
    let interchange_trailer = InterchangeTrailer {
        se_segments,
        ge_segments,
        iea_segments,
    };
    
    return (interchange_trailer, contents)
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
        let contents = String::from("SE*10*0001~GE*1*1~IEA*1*000000001~");
        let (trailer, contents) = get_interchange_trailer(contents);
        assert_eq!(trailer.se_segments.number_of_segment, "10");
        assert_eq!(trailer.se_segments.transaction_set_control_number, "0001");
        assert_eq!(trailer.ge_segments.number_of_transitions, "1");
        assert_eq!(trailer.ge_segments.group_control_number, "1");
        assert_eq!(trailer.iea_segments.number_of_included_group, "1");
        assert_eq!(trailer.iea_segments.interchange_control_number, "000000001");
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_interchange_trailer() {
        let trailer = InterchangeTrailer {
            se_segments: SE {
                number_of_segment: "10".to_string(),
                transaction_set_control_number: "0001".to_string(),
            },
            ge_segments: GE {
                number_of_transitions: "1".to_string(),
                group_control_number: "1".to_string(),
            },
            iea_segments: IEA {
                number_of_included_group: "1".to_string(),
                interchange_control_number: "000000001".to_string(),
            },
        };
        
        let contents = write_interchange_trailer(trailer);
        assert_eq!(contents, "SE*10*0001~GE*1*1~IEA*1*000000001~");
    }
}
