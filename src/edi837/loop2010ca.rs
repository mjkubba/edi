use serde::{Serialize, Deserialize};

/// Loop2010CA - Patient Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ca {
    /// Patient Name
    pub nm1: String,
    /// Patient Address
    pub n3: Option<String>,
    /// Patient City, State, ZIP Code
    pub n4: Option<String>,
    /// Patient Demographic Information
    pub dmg: Option<String>,
    /// Patient Secondary Identification
    pub ref_segments: Vec<String>,
}

/// Write Loop2010CA to EDI format
pub fn write_loop2010ca(loop2010ca: &Loop2010ca) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010ca.nm1);
    result.push_str("\n");
    
    // Write N3 segment if present
    if let Some(n3) = &loop2010ca.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2010ca.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2010ca.dmg {
        result.push_str(dmg);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2010ca.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010CA from EDI content
pub fn parse_loop2010ca(content: &str) -> Loop2010ca {
    let mut loop2010ca = Loop2010ca::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*QC*") {
            loop2010ca.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010ca.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2010ca.n4 = Some(segment.to_string());
        } else if segment.starts_with("DMG*") {
            loop2010ca.dmg = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2010ca.ref_segments.push(segment.to_string());
        }
    }
    
    loop2010ca
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010ca() {
        let content = "NM1*QC*1*DOE*JANE****MI*123456789B\nN3*123 MAIN ST\nN4*ANYTOWN*PA*17111\nDMG*D8*19800519*F\nREF*SY*987654321";
        
        let loop2010ca = parse_loop2010ca(content);
        
        assert_eq!(loop2010ca.nm1, "NM1*QC*1*DOE*JANE****MI*123456789B");
        assert_eq!(loop2010ca.n3, Some("N3*123 MAIN ST".to_string()));
        assert_eq!(loop2010ca.n4, Some("N4*ANYTOWN*PA*17111".to_string()));
        assert_eq!(loop2010ca.dmg, Some("DMG*D8*19800519*F".to_string()));
        assert_eq!(loop2010ca.ref_segments, vec!["REF*SY*987654321".to_string()]);
    }
    
    #[test]
    fn test_write_loop2010ca() {
        let mut loop2010ca = Loop2010ca::default();
        loop2010ca.nm1 = "NM1*QC*1*DOE*JANE****MI*123456789B".to_string();
        loop2010ca.n3 = Some("N3*123 MAIN ST".to_string());
        loop2010ca.n4 = Some("N4*ANYTOWN*PA*17111".to_string());
        loop2010ca.dmg = Some("DMG*D8*19800519*F".to_string());
        loop2010ca.ref_segments.push("REF*SY*987654321".to_string());
        
        let result = write_loop2010ca(&loop2010ca);
        
        assert!(result.contains("NM1*QC*1*DOE*JANE****MI*123456789B\n"));
        assert!(result.contains("N3*123 MAIN ST\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111\n"));
        assert!(result.contains("DMG*D8*19800519*F\n"));
        assert!(result.contains("REF*SY*987654321\n"));
    }
}
