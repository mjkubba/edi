use serde::{Serialize, Deserialize};

/// Loop2000C - Patient Hierarchical Level
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000c {
    /// Hierarchical Level
    pub hl: String,
    /// Patient Information
    pub pat: String,
    /// Demographic Information
    pub dmg: Option<String>,
    /// Patient Name
    pub nm1_patient: Option<String>,
    /// Patient Address
    pub n3: Option<String>,
    /// Patient City, State, ZIP Code
    pub n4: Option<String>,
    /// Patient Additional Identification
    pub ref_patient: Vec<String>,
    /// Patient Date
    pub dtp: Vec<String>,
}

/// Write Loop2000C to EDI format
pub fn write_loop2000c(loop2000c: &Loop2000c) -> String {
    let mut result = String::new();
    
    // Write HL segment
    result.push_str(&loop2000c.hl);
    result.push_str("\n");
    
    // Write PAT segment
    result.push_str(&loop2000c.pat);
    result.push_str("\n");
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000c.dmg {
        result.push_str(dmg);
        result.push_str("\n");
    }
    
    // Write NM1 segment if present
    if let Some(nm1) = &loop2000c.nm1_patient {
        result.push_str(nm1);
        result.push_str("\n");
    }
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000c.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000c.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2000c.ref_patient {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    // Write DTP segments
    for dtp in &loop2000c.dtp {
        result.push_str(dtp);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2000C from EDI content
pub fn parse_loop2000c(content: &str) -> (Loop2000c, String) {
    let mut loop2000c = Loop2000c::default();
    let mut remaining_content = content.to_string();
    
    // Parse HL segment for patient
    if let Some(hl_pos) = remaining_content.find("HL*") {
        // Check if this is a patient HL (hierarchical level code 23)
        let hl_segment = &remaining_content[hl_pos..];
        if hl_segment.contains("*23*") {
            let hl_end = remaining_content[hl_pos..].find('~').unwrap_or(remaining_content.len()) + hl_pos;
            loop2000c.hl = remaining_content[hl_pos..=hl_end].to_string();
            remaining_content = remaining_content[hl_end + 1..].to_string();
        } else {
            // Not a patient HL, return empty loop
            return (loop2000c, remaining_content);
        }
    }
    
    // Parse PAT segment
    if let Some(pat_pos) = remaining_content.find("PAT*") {
        let pat_end = remaining_content[pat_pos..].find('~').unwrap_or(remaining_content.len()) + pat_pos;
        loop2000c.pat = remaining_content[pat_pos..=pat_end].to_string();
        remaining_content = remaining_content[pat_end + 1..].to_string();
    }
    
    // Parse DMG segment if present
    if let Some(dmg_pos) = remaining_content.find("DMG*") {
        // Check if this DMG belongs to this loop or the next one
        if !remaining_content[..dmg_pos].contains("HL*") && 
           !remaining_content[..dmg_pos].contains("NM1*") {
            let dmg_end = remaining_content[dmg_pos..].find('~').unwrap_or(remaining_content.len()) + dmg_pos;
            loop2000c.dmg = Some(remaining_content[dmg_pos..=dmg_end].to_string());
            remaining_content = remaining_content[dmg_end + 1..].to_string();
        }
    }
    
    // Parse NM1 segment for patient
    if let Some(nm1_pos) = remaining_content.find("NM1*QC*") {
        // Check if this NM1 belongs to this loop or the next one
        if !remaining_content[..nm1_pos].contains("HL*") {
            let nm1_end = remaining_content[nm1_pos..].find('~').unwrap_or(remaining_content.len()) + nm1_pos;
            loop2000c.nm1_patient = Some(remaining_content[nm1_pos..=nm1_end].to_string());
            remaining_content = remaining_content[nm1_end + 1..].to_string();
        }
    }
    
    // Parse N3 segment if present
    if let Some(n3_pos) = remaining_content.find("N3*") {
        // Check if this N3 belongs to this loop or the next one
        if !remaining_content[..n3_pos].contains("HL*") && 
           !remaining_content[..n3_pos].contains("NM1*") {
            let n3_end = remaining_content[n3_pos..].find('~').unwrap_or(remaining_content.len()) + n3_pos;
            loop2000c.n3 = Some(remaining_content[n3_pos..=n3_end].to_string());
            remaining_content = remaining_content[n3_end + 1..].to_string();
        }
    }
    
    // Parse N4 segment if present
    if let Some(n4_pos) = remaining_content.find("N4*") {
        // Check if this N4 belongs to this loop or the next one
        if !remaining_content[..n4_pos].contains("HL*") && 
           !remaining_content[..n4_pos].contains("NM1*") {
            let n4_end = remaining_content[n4_pos..].find('~').unwrap_or(remaining_content.len()) + n4_pos;
            loop2000c.n4 = Some(remaining_content[n4_pos..=n4_end].to_string());
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
        loop2000c.ref_patient.push(remaining_content[ref_pos..=ref_end].to_string());
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
        loop2000c.dtp.push(remaining_content[dtp_pos..=dtp_end].to_string());
        remaining_content = remaining_content[dtp_end + 1..].to_string();
    }
    
    (loop2000c, remaining_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2000c() {
        let content = "HL*3*2*23*0\nPAT*19\nDMG*D8*19800519*M\nNM1*QC*1*DOE*JANE****MI*123456789B\nN3*123 MAIN ST\nN4*ANYTOWN*PA*17111\nREF*SY*987654321\nDTP*304*D8*20230101";
        
        let loop2000c = parse_loop2000c(content);
        
        assert_eq!(loop2000c.hl, "HL*3*2*23*0");
        assert_eq!(loop2000c.pat, "PAT*19");
        assert_eq!(loop2000c.dmg, Some("DMG*D8*19800519*M".to_string()));
        assert_eq!(loop2000c.nm1_patient, Some("NM1*QC*1*DOE*JANE****MI*123456789B".to_string()));
        assert_eq!(loop2000c.n3, Some("N3*123 MAIN ST".to_string()));
        assert_eq!(loop2000c.n4, Some("N4*ANYTOWN*PA*17111".to_string()));
        assert_eq!(loop2000c.ref_patient, vec!["REF*SY*987654321".to_string()]);
        assert_eq!(loop2000c.dtp, vec!["DTP*304*D8*20230101".to_string()]);
    }
    
    #[test]
    fn test_write_loop2000c() {
        let mut loop2000c = Loop2000c::default();
        loop2000c.hl = "HL*3*2*23*0".to_string();
        loop2000c.pat = "PAT*19".to_string();
        loop2000c.dmg = Some("DMG*D8*19800519*M".to_string());
        loop2000c.nm1_patient = Some("NM1*QC*1*DOE*JANE****MI*123456789B".to_string());
        loop2000c.n3 = Some("N3*123 MAIN ST".to_string());
        loop2000c.n4 = Some("N4*ANYTOWN*PA*17111".to_string());
        loop2000c.ref_patient.push("REF*SY*987654321".to_string());
        loop2000c.dtp.push("DTP*304*D8*20230101".to_string());
        
        let result = write_loop2000c(&loop2000c);
        
        assert!(result.contains("HL*3*2*23*0\n"));
        assert!(result.contains("PAT*19\n"));
        assert!(result.contains("DMG*D8*19800519*M\n"));
        assert!(result.contains("NM1*QC*1*DOE*JANE****MI*123456789B\n"));
        assert!(result.contains("N3*123 MAIN ST\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111\n"));
        assert!(result.contains("REF*SY*987654321\n"));
        assert!(result.contains("DTP*304*D8*20230101\n"));
    }
}
