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
pub fn parse_loop2000c(content: &str) -> Loop2000c {
    let mut loop2000c = Loop2000c::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("HL*") {
            loop2000c.hl = segment.to_string();
        } else if segment.starts_with("PAT*") {
            loop2000c.pat = segment.to_string();
        } else if segment.starts_with("DMG*") {
            loop2000c.dmg = Some(segment.to_string());
        } else if segment.starts_with("NM1*QC*") {
            loop2000c.nm1_patient = Some(segment.to_string());
        } else if segment.starts_with("N3*") {
            loop2000c.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2000c.n4 = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2000c.ref_patient.push(segment.to_string());
        } else if segment.starts_with("DTP*") {
            loop2000c.dtp.push(segment.to_string());
        }
    }
    
    loop2000c
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
