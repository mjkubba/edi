use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::dtp::*;
use crate::segments::hi::*;
use crate::segments::hsd::*;
use crate::segments::cl1::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100E {
    pub dtp_segments: Vec<DTP>,
    pub hi_segments: Option<HI>,
    pub hsd_segments: Option<HSD>,
    pub cl1_segments: Option<CL1>,
}

pub fn get_loop2100e(mut contents: String) -> (Loop2100E, String) {
    let mut dtp_segments = Vec::new();
    let mut hi_segments = None;
    let mut hsd_segments = None;
    let mut cl1_segments = None;
    
    // Parse DTP segments
    while contents.contains("DTP") && check_if_segement_in_loop("DTP", "HI", contents.clone()) && 
          check_if_segement_in_loop("DTP", "HSD", contents.clone()) && 
          check_if_segement_in_loop("DTP", "CL1", contents.clone()) && 
          check_if_segement_in_loop("DTP", "NM1", contents.clone()) {
        
        info!("DTP segment found, ");
        let dtp_segment = get_dtp(get_segment_contents("DTP", &contents));
        info!("DTP segment parsed");
        
        dtp_segments.push(dtp_segment);
        contents = content_trim("DTP", contents);
    }
    
    // Parse HI segment
    if contents.contains("HI") && check_if_segement_in_loop("HI", "HSD", contents.clone()) && 
       check_if_segement_in_loop("HI", "CL1", contents.clone()) && 
       check_if_segement_in_loop("HI", "NM1", contents.clone()) {
        
        info!("HI segment found, ");
        hi_segments = Some(get_hi(get_segment_contents("HI", &contents)));
        info!("HI segment parsed");
        
        contents = content_trim("HI", contents);
    }
    
    // Parse HSD segment
    if contents.contains("HSD") && check_if_segement_in_loop("HSD", "CL1", contents.clone()) && 
       check_if_segement_in_loop("HSD", "NM1", contents.clone()) {
        
        info!("HSD segment found, ");
        hsd_segments = Some(get_hsd(get_segment_contents("HSD", &contents)));
        info!("HSD segment parsed");
        
        contents = content_trim("HSD", contents);
    }
    
    // Parse CL1 segment
    if contents.contains("CL1") && check_if_segement_in_loop("CL1", "NM1", contents.clone()) {
        info!("CL1 segment found, ");
        cl1_segments = Some(get_cl1(get_segment_contents("CL1", &contents)));
        info!("CL1 segment parsed");
        
        contents = content_trim("CL1", contents);
    }
    
    info!("Loop 2100E parsed\n");
    
    let loop2100e = Loop2100E {
        dtp_segments,
        hi_segments,
        hsd_segments,
        cl1_segments,
    };
    
    return (loop2100e, contents)
}

pub fn write_loop2100e(loop2100e: Loop2100E) -> String {
    let mut contents = String::new();
    
    for dtp in loop2100e.dtp_segments {
        contents.push_str(&write_dtp(dtp));
    }
    
    if let Some(hi) = loop2100e.hi_segments {
        contents.push_str(&write_hi(hi));
    }
    
    if let Some(hsd) = loop2100e.hsd_segments {
        contents.push_str(&write_hsd(hsd));
    }
    
    if let Some(cl1) = loop2100e.cl1_segments {
        contents.push_str(&write_cl1(cl1));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2100e() {
        let contents = String::from("DTP*435*D8*20050516~HI*BF:41090:D8:20050125~HSD*DY*7~CL1*2~");
        let (loop2100e, contents) = get_loop2100e(contents);
        
        assert_eq!(loop2100e.dtp_segments.len(), 1);
        assert_eq!(loop2100e.dtp_segments[0].dtp01_date_time_qualifier, "435");
        assert_eq!(loop2100e.dtp_segments[0].dtp02_date_time_format_qualifier, "D8");
        assert_eq!(loop2100e.dtp_segments[0].dtp03_date_time_value, "20050516");
        
        assert!(loop2100e.hi_segments.is_some());
        let hi = loop2100e.hi_segments.unwrap();
        assert_eq!(hi.hi01_health_care_code_information, "BF");
        assert_eq!(hi.hi02_health_care_code_information, "41090");
        
        assert!(loop2100e.hsd_segments.is_some());
        let hsd = loop2100e.hsd_segments.unwrap();
        assert_eq!(hsd.hsd01_quantity_qualifier, "DY");
        assert_eq!(hsd.hsd02_quantity, "7");
        
        assert!(loop2100e.cl1_segments.is_some());
        let cl1 = loop2100e.cl1_segments.unwrap();
        assert_eq!(cl1.cl101_admission_type_code, "2");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2100e() {
        let loop2100e = Loop2100E {
            dtp_segments: vec![
                DTP {
                    dtp01_date_time_qualifier: "435".to_string(),
                    dtp02_date_time_format_qualifier: "D8".to_string(),
                    dtp03_date_time_value: "20050516".to_string(),
                }
            ],
            hi_segments: Some(HI {
                hi01_health_care_code_information: "BF".to_string(),
                hi02_health_care_code_information: "41090".to_string(),
                hi03_health_care_code_information: "D8".to_string(),
                hi04_health_care_code_information: "20050125".to_string(),
                hi05_health_care_code_information: "".to_string(),
                hi06_health_care_code_information: "".to_string(),
                hi07_health_care_code_information: "".to_string(),
                hi08_health_care_code_information: "".to_string(),
                hi09_health_care_code_information: "".to_string(),
                hi10_health_care_code_information: "".to_string(),
                hi11_health_care_code_information: "".to_string(),
                hi12_health_care_code_information: "".to_string(),
            }),
            hsd_segments: Some(HSD {
                hsd01_quantity_qualifier: "DY".to_string(),
                hsd02_quantity: "7".to_string(),
                hsd03_unit_of_measure_code: "".to_string(),
                hsd04_sample_selection_modulus: "".to_string(),
                hsd05_time_period_qualifier: "".to_string(),
                hsd06_period_count: "".to_string(),
                hsd07_delivery_frequency_code: "".to_string(),
                hsd08_delivery_pattern_time_code: "".to_string(),
            }),
            cl1_segments: Some(CL1 {
                cl101_admission_type_code: "2".to_string(),
                cl102_admission_source_code: "".to_string(),
                cl103_patient_status_code: "".to_string(),
            }),
        };
        
        let contents = write_loop2100e(loop2100e);
        assert!(contents.contains("DTP*435*D8*20050516~"));
        assert!(contents.contains("HI*BF*41090*D8*20050125~"));
        assert!(contents.contains("HSD*DY*7~"));
        assert!(contents.contains("CL1*2~"));
    }
}
