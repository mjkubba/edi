use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::dtp::*;
use crate::segments::sv2::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2100F {
    pub dtp_segments: Vec<DTP>,
    pub sv2_segments: Option<SV2>,
}

pub fn get_loop2100f(mut contents: String) -> (Loop2100F, String) {
    let mut dtp_segments = Vec::new();
    let mut sv2_segments = None;
    
    // Parse DTP segments
    while contents.contains("DTP") && check_if_segement_in_loop("DTP", "SV2", contents.clone()) && 
          check_if_segement_in_loop("DTP", "NM1", contents.clone()) {
        
        info!("DTP segment found, ");
        let dtp_segment = get_dtp(get_segment_contents("DTP", &contents));
        info!("DTP segment parsed");
        
        dtp_segments.push(dtp_segment);
        contents = content_trim("DTP", contents);
    }
    
    // Parse SV2 segment
    if contents.contains("SV2") && check_if_segement_in_loop("SV2", "NM1", contents.clone()) {
        info!("SV2 segment found, ");
        let sv2_content = get_segment_contents("SV2", &contents);
        sv2_segments = Some(get_sv2(sv2_content));
        info!("SV2 segment parsed");
        
        contents = content_trim("SV2", contents);
    }
    
    info!("Loop 2100F parsed\n");
    
    let loop2100f = Loop2100F {
        dtp_segments,
        sv2_segments,
    };
    
    return (loop2100f, contents)
}

pub fn write_loop2100f(loop2100f: Loop2100F) -> String {
    let mut contents = String::new();
    
    for dtp in loop2100f.dtp_segments {
        contents.push_str(&write_dtp(dtp));
    }
    
    if let Some(sv2) = loop2100f.sv2_segments {
        contents.push_str(&write_sv2(sv2));
    }
    
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop2100f() {
        let contents = String::from("DTP*472*D8*20050516~SV2**HC:33510~");
        let (loop2100f, contents) = get_loop2100f(contents);
        
        assert_eq!(loop2100f.dtp_segments.len(), 1);
        assert_eq!(loop2100f.dtp_segments[0].dtp01_date_time_qualifier, "472");
        assert_eq!(loop2100f.dtp_segments[0].dtp02_date_time_period_format_qualifier, "D8");
        assert_eq!(loop2100f.dtp_segments[0].dtp03_date_time_period, "20050516");
        
        assert!(loop2100f.sv2_segments.is_some());
        let sv2 = loop2100f.sv2_segments.unwrap();
        assert_eq!(sv2.sv201_service_line_revenue_code, "");
        assert_eq!(sv2.sv202_procedure_code, "HC:33510");
        
        assert_eq!(contents, "");
    }
    
    #[test]
    fn test_write_loop2100f() {
        let loop2100f = Loop2100F {
            dtp_segments: vec![
                DTP {
                    dtp01_date_time_qualifier: "472".to_string(),
                    dtp02_date_time_period_format_qualifier: "D8".to_string(),
                    dtp03_date_time_period: "20050516".to_string(),
                }
            ],
            sv2_segments: Some(SV2 {
                sv201_service_line_revenue_code: "".to_string(),
                sv202_procedure_code: "HC:33510".to_string(),
                sv203_line_item_charge_amount: "".to_string(),
                sv204_unit_or_basis_for_measurement_code: "".to_string(),
                sv205_service_unit_count: "".to_string(),
                sv206_unit_rate: "".to_string(),
                sv207_amount: "".to_string(),
                sv208_yes_no_condition_or_response_code: "".to_string(),
                sv209_nursing_home_residential_status_code: "".to_string(),
                sv210_level_of_care_code: "".to_string(),
            }),
        };
        
        let contents = write_loop2100f(loop2100f);
        assert!(contents.contains("DTP*472*D8*20050516~"));
        assert!(contents.contains("SV2**HC:33510~"));
    }
}
