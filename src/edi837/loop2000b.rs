use serde::{Serialize, Deserialize};

/// Loop2000B - Subscriber Hierarchical Level
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000b {
    /// Hierarchical Level
    pub hl: String,
    /// Subscriber Information
    pub sbr: String,
    /// Patient Information
    pub pat: Option<String>,
    /// Demographic Information
    pub dmg: Option<String>,
    /// Subscriber Name
    pub nm1_subscriber: Option<String>,
    /// Subscriber Address
    pub n3: Option<String>,
    /// Subscriber City, State, ZIP Code
    pub n4: Option<String>,
    /// Subscriber Additional Identification
    pub ref_subscriber: Vec<String>,
    /// Subscriber Date
    pub dtp: Vec<String>,
}

/// Write Loop2000B to EDI format
pub fn write_loop2000b(loop2000b: &Loop2000b) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&loop2000b.hl);
    result.push_str("\n");
    
    // Write SBR segment
    result.push_str(&loop2000b.sbr);
    result.push_str("\n");
    
    // Write PAT segment if present
    if let Some(pat) = &loop2000b.pat {
        result.push_str(pat);
        result.push_str("\n");
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000b.dmg {
        result.push_str(dmg);
        result.push_str("\n");
    }
    
    // Write NM1 segment if present
    if let Some(nm1) = &loop2000b.nm1_subscriber {
        result.push_str(nm1);
        result.push_str("\n");
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000b.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000b.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2000b.ref_subscriber {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    // Write DTP segments
    for dtp in &loop2000b.dtp {
        result.push_str(dtp);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2000B from EDI content
pub fn parse_loop2000b(content: &str) -> Loop2000b {
    let mut loop2000b = Loop2000b::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("HL*") {
            loop2000b.hl = segment.to_string();
        } else if segment.starts_with("SBR*") {
            loop2000b.sbr = segment.to_string();
        } else if segment.starts_with("PAT*") {
            loop2000b.pat = Some(segment.to_string());
        } else if segment.starts_with("DMG*") {
            loop2000b.dmg = Some(segment.to_string());
        } else if segment.starts_with("NM1*IL*") {
            loop2000b.nm1_subscriber = Some(segment.to_string());
        } else if segment.starts_with("N3*") {
            loop2000b.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2000b.n4 = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2000b.ref_subscriber.push(segment.to_string());
        } else if segment.starts_with("DTP*") {
            loop2000b.dtp.push(segment.to_string());
        }
    }
    
    loop2000b
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2000b() {
        let content = "HL*2*1*22*0\nSBR*P*18*******MC\nNM1*IL*1*DOE*JOHN****MI*123456789A\nN3*123 MAIN ST\nN4*ANYTOWN*PA*17111\nREF*SY*123456789\nDTP*307*D8*20230101";
        
        let loop2000b = parse_loop2000b(content);
        
        assert_eq!(loop2000b.hl, "HL*2*1*22*0");
        assert_eq!(loop2000b.sbr, "SBR*P*18*******MC");
        assert_eq!(loop2000b.nm1_subscriber, Some("NM1*IL*1*DOE*JOHN****MI*123456789A".to_string()));
        assert_eq!(loop2000b.n3, Some("N3*123 MAIN ST".to_string()));
        assert_eq!(loop2000b.n4, Some("N4*ANYTOWN*PA*17111".to_string()));
        assert_eq!(loop2000b.ref_subscriber, vec!["REF*SY*123456789".to_string()]);
        assert_eq!(loop2000b.dtp, vec!["DTP*307*D8*20230101".to_string()]);
    }
    
    #[test]
    fn test_write_loop2000b() {
        let mut loop2000b = Loop2000b::default();
        loop2000b.hl = "HL*2*1*22*0".to_string();
        loop2000b.sbr = "SBR*P*18*******MC".to_string();
        loop2000b.nm1_subscriber = Some("NM1*IL*1*DOE*JOHN****MI*123456789A".to_string());
        loop2000b.n3 = Some("N3*123 MAIN ST".to_string());
        loop2000b.n4 = Some("N4*ANYTOWN*PA*17111".to_string());
        loop2000b.ref_subscriber.push("REF*SY*123456789".to_string());
        loop2000b.dtp.push("DTP*307*D8*20230101".to_string());
        
        let result = write_loop2000b(&loop2000b);
        
        assert!(result.contains("HL*2*1*22*0\n"));
        assert!(result.contains("SBR*P*18*******MC\n"));
        assert!(result.contains("NM1*IL*1*DOE*JOHN****MI*123456789A\n"));
        assert!(result.contains("N3*123 MAIN ST\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111\n"));
        assert!(result.contains("REF*SY*123456789\n"));
        assert!(result.contains("DTP*307*D8*20230101\n"));
    }
}
