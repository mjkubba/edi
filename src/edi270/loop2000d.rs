use log::info;
use serde::{Serialize, Deserialize};

use crate::segments::hl::*;
use crate::segments::nm1::*;
use crate::segments::trn::*;
use crate::segments::r#ref::*;
use crate::segments::n3::*;
use crate::segments::n4::*;
use crate::segments::dmg::*;
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000D {
    pub hl_segments: HL,
    pub trn_segments: Option<TRN>,
    pub nm1_segments: NM1,
    pub ref_segments: Vec<REF>,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub dmg_segments: Option<DMG>,
}

pub fn get_loop_2000d(mut contents: String) -> (Loop2000D, String) {
    let mut loop2000d = Loop2000D::default();
    
    // Process HL segment (required)
    if contents.contains("HL") {
        info!("HL segment found");
        let hl_content = get_segment_contents("HL", &contents);
        loop2000d.hl_segments = get_hl(hl_content);
        
        // Verify this is a Dependent level HL segment (level code = 23)
        if loop2000d.hl_segments.hl03_hierarchical_level_code != "23" {
            info!("Warning: Expected HL03 code '23' for Dependent level, got '{}'",
                loop2000d.hl_segments.hl03_hierarchical_level_code);
        }
        
        info!("HL segment parsed");
        contents = content_trim("HL", contents);
    } else {
        info!("Warning: Required HL segment not found in Loop 2000D");
    }
    
    // Process TRN segment (situational)
    if contents.contains("TRN") {
        info!("TRN segment found");
        let trn_content = get_segment_contents("TRN", &contents);
        loop2000d.trn_segments = Some(get_trn(trn_content));
        info!("TRN segment parsed");
        contents = content_trim("TRN", contents);
    }
    
    // Process NM1 segment (required)
    if contents.contains("NM1") {
        info!("NM1 segment found");
        let nm1_content = get_segment_contents("NM1", &contents);
        loop2000d.nm1_segments = get_nm1(nm1_content);
        info!("NM1 segment parsed");
        contents = content_trim("NM1", contents);
    } else {
        info!("Warning: Required NM1 segment not found in Loop 2000D");
    }
    
    // Process REF segments (situational, can be multiple)
    while contents.starts_with("REF") {
        info!("REF segment found");
        let ref_content = get_segment_contents("REF", &contents);
        let ref_segment = get_ref(ref_content);
        info!("REF segment parsed");
        loop2000d.ref_segments.push(ref_segment);
        contents = content_trim("REF", contents);
    }
    
    // Process N3 segment (situational)
    if contents.contains("N3") {
        info!("N3 segment found");
        let n3_content = get_segment_contents("N3", &contents);
        loop2000d.n3_segments = Some(get_n3(n3_content));
        info!("N3 segment parsed");
        contents = content_trim("N3", contents);
    }
    
    // Process N4 segment (situational)
    if contents.contains("N4") {
        info!("N4 segment found");
        let n4_content = get_segment_contents("N4", &contents);
        loop2000d.n4_segments = Some(get_n4(n4_content));
        info!("N4 segment parsed");
        contents = content_trim("N4", contents);
    }
    
    // Process DMG segment (situational)
    if contents.contains("DMG") {
        info!("DMG segment found");
        let dmg_content = get_segment_contents("DMG", &contents);
        loop2000d.dmg_segments = Some(get_dmg(dmg_content));
        info!("DMG segment parsed");
        contents = content_trim("DMG", contents);
    }
    
    info!("Loop 2000D parsed");
    (loop2000d, contents)
}

pub fn write_loop_2000d(loop2000d: &Loop2000D) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000d.hl_segments.clone()));
    
    // Write TRN segment if present
    if let Some(trn) = &loop2000d.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000d.nm1_segments.clone()));
    
    // Write all REF segments
    for ref_segment in &loop2000d.ref_segments {
        contents.push_str(&write_ref(ref_segment.clone()));
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000d.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000d.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000d.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_2000d_parsing() {
        let contents = "HL*3*2*23*0~NM1*IL*1*DOE*JANE****MI*98765432101~REF*SY*123456789~DMG*D8*19800101*F~".to_string();
        let (loop2000d, remaining) = get_loop_2000d(contents);
        
        assert_eq!(loop2000d.hl_segments.hl01_hierarchical_id_number, "3");
        assert_eq!(loop2000d.hl_segments.hl02_hierarchical_parent_id_number, "2");
        assert_eq!(loop2000d.hl_segments.hl03_hierarchical_level_code, "23");
        assert_eq!(loop2000d.hl_segments.hl04_hierarchical_child_code, "0");
        
        assert_eq!(loop2000d.nm1_segments.entity_id, "IL");
        assert_eq!(loop2000d.nm1_segments.entity_type, "1");
        assert_eq!(loop2000d.nm1_segments.lastname, "DOE");
        assert_eq!(loop2000d.nm1_segments.firstname, "JANE");
        
        assert_eq!(loop2000d.ref_segments.len(), 1);
        assert_eq!(loop2000d.ref_segments[0].reference_id_number_qualifier, "SY");
        assert_eq!(loop2000d.ref_segments[0].reference_id_number, "123456789");
        
        assert!(loop2000d.dmg_segments.is_some());
        let dmg = loop2000d.dmg_segments.unwrap();
        assert_eq!(dmg.dmg01_date_time_qualifier, "D8");
        assert_eq!(dmg.dmg02_date_time_period, "19800101");
        assert_eq!(dmg.dmg03_gender_code, "F");
        
        assert_eq!(remaining, "");
    }
}
