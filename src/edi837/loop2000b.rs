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
pub fn parse_loop2000b(content: &str) -> (Loop2000b, String) {
    let mut loop2000b = Loop2000b::default();
    let mut remaining_content = content.to_string();
    
    // Parse HL segment for subscriber
    if let Some(hl_pos) = remaining_content.find("HL*") {
        // Check if this is a subscriber HL (hierarchical level code 22)
        let hl_segment = &remaining_content[hl_pos..];
        if hl_segment.contains("*22*") {
            let hl_end = remaining_content[hl_pos..].find('~').unwrap_or(remaining_content.len()) + hl_pos;
            loop2000b.hl = remaining_content[hl_pos..=hl_end].to_string();
            remaining_content = remaining_content[hl_end + 1..].to_string();
        } else {
            // Not a subscriber HL, return empty loop
            return (loop2000b, remaining_content);
        }
    }
    
    // Parse SBR segment
    if let Some(sbr_pos) = remaining_content.find("SBR*") {
        let sbr_end = remaining_content[sbr_pos..].find('~').unwrap_or(remaining_content.len()) + sbr_pos;
        loop2000b.sbr = remaining_content[sbr_pos..=sbr_end].to_string();
        remaining_content = remaining_content[sbr_end + 1..].to_string();
    }
    
    // Parse PAT segment if present
    if let Some(pat_pos) = remaining_content.find("PAT*") {
        // Check if this PAT belongs to this loop or the next one
        if !remaining_content[..pat_pos].contains("HL*") {
            let pat_end = remaining_content[pat_pos..].find('~').unwrap_or(remaining_content.len()) + pat_pos;
            loop2000b.pat = Some(remaining_content[pat_pos..=pat_end].to_string());
            remaining_content = remaining_content[pat_end + 1..].to_string();
        }
    }
    
    // Parse DMG segment if present
    if let Some(dmg_pos) = remaining_content.find("DMG*") {
        // Check if this DMG belongs to this loop or the next one
        if !remaining_content[..dmg_pos].contains("HL*") && 
           !remaining_content[..dmg_pos].contains("NM1*") {
            let dmg_end = remaining_content[dmg_pos..].find('~').unwrap_or(remaining_content.len()) + dmg_pos;
            loop2000b.dmg = Some(remaining_content[dmg_pos..=dmg_end].to_string());
            remaining_content = remaining_content[dmg_end + 1..].to_string();
        }
    }
    
    // Parse NM1 segment for subscriber
    if let Some(nm1_pos) = remaining_content.find("NM1*IL*") {
        // Check if this NM1 belongs to this loop or the next one
        if !remaining_content[..nm1_pos].contains("HL*") {
            let nm1_end = remaining_content[nm1_pos..].find('~').unwrap_or(remaining_content.len()) + nm1_pos;
            loop2000b.nm1_subscriber = Some(remaining_content[nm1_pos..=nm1_end].to_string());
            remaining_content = remaining_content[nm1_end + 1..].to_string();
        }
    }
    
    // Parse N3 segment if present
    if let Some(n3_pos) = remaining_content.find("N3*") {
        // Check if this N3 belongs to this loop or the next one
        if !remaining_content[..n3_pos].contains("HL*") && 
           !remaining_content[..n3_pos].contains("NM1*") {
            let n3_end = remaining_content[n3_pos..].find('~').unwrap_or(remaining_content.len()) + n3_pos;
            loop2000b.n3 = Some(remaining_content[n3_pos..=n3_end].to_string());
            remaining_content = remaining_content[n3_end + 1..].to_string();
        }
    }
    
    // Parse N4 segment if present
    if let Some(n4_pos) = remaining_content.find("N4*") {
        // Check if this N4 belongs to this loop or the next one
        if !remaining_content[..n4_pos].contains("HL*") && 
           !remaining_content[..n4_pos].contains("NM1*") {
            let n4_end = remaining_content[n4_pos..].find('~').unwrap_or(remaining_content.len()) + n4_pos;
            loop2000b.n4 = Some(remaining_content[n4_pos..=n4_end].to_string());
            remaining_content = remaining_content[n4_end + 1..].to_string();
        }
    }
    
    // Parse REF segments
    while let Some(ref_pos) = remaining_content.find("REF*") {
        // Check if this REF belongs to this loop or the next one
        if remaining_content[..ref_pos].contains("HL*") || 
           remaining_content[..ref_pos].contains("NM1*") {
            break;
        }
        
        let ref_end = remaining_content[ref_pos..].find('~').unwrap_or(remaining_content.len()) + ref_pos;
        loop2000b.ref_subscriber.push(remaining_content[ref_pos..=ref_end].to_string());
        remaining_content = remaining_content[ref_end + 1..].to_string();
    }
    
    // Parse DTP segments
    while let Some(dtp_pos) = remaining_content.find("DTP*") {
        // Check if this DTP belongs to this loop or the next one
        if remaining_content[..dtp_pos].contains("HL*") || 
           remaining_content[..dtp_pos].contains("NM1*") {
            break;
        }
        
        let dtp_end = remaining_content[dtp_pos..].find('~').unwrap_or(remaining_content.len()) + dtp_pos;
        loop2000b.dtp.push(remaining_content[dtp_pos..=dtp_end].to_string());
        remaining_content = remaining_content[dtp_end + 1..].to_string();
    }
    
    (loop2000b, remaining_content)
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
