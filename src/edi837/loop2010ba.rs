use serde::{Serialize, Deserialize};

/// Loop2010BA - Subscriber Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ba {
    /// Subscriber Name
    pub nm1: String,
    /// Subscriber Address
    pub n3: Option<String>,
    /// Subscriber City, State, ZIP Code
    pub n4: Option<String>,
    /// Subscriber Demographic Information
    pub dmg: Option<String>,
    /// Subscriber Secondary Identification
    pub ref_segments: Vec<String>,
}

/// Write Loop2010BA to EDI format
pub fn write_loop2010ba(loop2010ba: &Loop2010ba) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010ba.nm1);
    result.push_str("\n");
    
    // Write N3 segment if present
    if let Some(n3) = &loop2010ba.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2010ba.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2010ba.dmg {
        result.push_str(dmg);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2010ba.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010BA from EDI content
pub fn parse_loop2010ba(content: &str) -> Loop2010ba {
    let mut loop2010ba = Loop2010ba::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*IL*") {
            loop2010ba.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010ba.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2010ba.n4 = Some(segment.to_string());
        } else if segment.starts_with("DMG*") {
            loop2010ba.dmg = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2010ba.ref_segments.push(segment.to_string());
        }
    }
    
    loop2010ba
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010ba() {
        let content = "NM1*IL*1*DOE*JOHN****MI*123456789A\nN3*123 MAIN ST\nN4*ANYTOWN*PA*17111\nDMG*D8*19700501*M\nREF*SY*123456789";
        
        let loop2010ba = parse_loop2010ba(content);
        
        assert_eq!(loop2010ba.nm1, "NM1*IL*1*DOE*JOHN****MI*123456789A");
        assert_eq!(loop2010ba.n3, Some("N3*123 MAIN ST".to_string()));
        assert_eq!(loop2010ba.n4, Some("N4*ANYTOWN*PA*17111".to_string()));
        assert_eq!(loop2010ba.dmg, Some("DMG*D8*19700501*M".to_string()));
        assert_eq!(loop2010ba.ref_segments, vec!["REF*SY*123456789".to_string()]);
    }
    
    #[test]
    fn test_write_loop2010ba() {
        let mut loop2010ba = Loop2010ba::default();
        loop2010ba.nm1 = "NM1*IL*1*DOE*JOHN****MI*123456789A".to_string();
        loop2010ba.n3 = Some("N3*123 MAIN ST".to_string());
        loop2010ba.n4 = Some("N4*ANYTOWN*PA*17111".to_string());
        loop2010ba.dmg = Some("DMG*D8*19700501*M".to_string());
        loop2010ba.ref_segments.push("REF*SY*123456789".to_string());
        
        let result = write_loop2010ba(&loop2010ba);
        
        assert!(result.contains("NM1*IL*1*DOE*JOHN****MI*123456789A\n"));
        assert!(result.contains("N3*123 MAIN ST\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111\n"));
        assert!(result.contains("DMG*D8*19700501*M\n"));
        assert!(result.contains("REF*SY*123456789\n"));
    }
}
