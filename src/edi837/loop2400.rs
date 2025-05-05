use serde::{Serialize, Deserialize};

/// Loop2400 - Service Line Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2400 {
    /// Service Line Number
    pub lx: String,
    /// Professional Service
    pub sv1: Option<String>,
    /// Institutional Service
    pub sv2: Option<String>,
    /// Dental Service
    pub sv3: Option<String>,
    /// Service Line Dates
    pub dtp: Vec<String>,
    /// Service Line Identifications
    pub ref_segments: Vec<String>,
    /// Service Line Amounts
    pub amt: Vec<String>,
    /// Service Line Quantities
    pub qty: Vec<String>,
    /// Service Line Note
    pub nte: Vec<String>,
    /// Service Line Pricing/Repricing Information
    pub hcp: Option<String>,
}

/// Write Loop2400 to EDI format
pub fn write_loop2400(loop2400: &Loop2400) -> String {
    let mut result = String::new();
    
    // Write LX segment
    result.push_str(&loop2400.lx);
    result.push_str("\n");
    
    // Write SV1 segment if present
    if let Some(sv1) = &loop2400.sv1 {
        result.push_str(sv1);
        result.push_str("\n");
    }
    
    // Write SV2 segment if present
    if let Some(sv2) = &loop2400.sv2 {
        result.push_str(sv2);
        result.push_str("\n");
    }
    
    // Write SV3 segment if present
    if let Some(sv3) = &loop2400.sv3 {
        result.push_str(sv3);
        result.push_str("\n");
    }
    
    // Write DTP segments
    for dtp in &loop2400.dtp {
        result.push_str(dtp);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2400.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    // Write AMT segments
    for amt in &loop2400.amt {
        result.push_str(amt);
        result.push_str("\n");
    }
    
    // Write QTY segments
    for qty in &loop2400.qty {
        result.push_str(qty);
        result.push_str("\n");
    }
    
    // Write NTE segments
    for nte in &loop2400.nte {
        result.push_str(nte);
        result.push_str("\n");
    }
    
    // Write HCP segment if present
    if let Some(hcp) = &loop2400.hcp {
        result.push_str(hcp);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2400 from EDI content
pub fn parse_loop2400(content: &str) -> Loop2400 {
    let mut loop2400 = Loop2400::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("LX*") {
            loop2400.lx = segment.to_string();
        } else if segment.starts_with("SV1*") {
            loop2400.sv1 = Some(segment.to_string());
        } else if segment.starts_with("SV2*") {
            loop2400.sv2 = Some(segment.to_string());
        } else if segment.starts_with("SV3*") {
            loop2400.sv3 = Some(segment.to_string());
        } else if segment.starts_with("DTP*") {
            loop2400.dtp.push(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2400.ref_segments.push(segment.to_string());
        } else if segment.starts_with("AMT*") {
            loop2400.amt.push(segment.to_string());
        } else if segment.starts_with("QTY*") {
            loop2400.qty.push(segment.to_string());
        } else if segment.starts_with("NTE*") {
            loop2400.nte.push(segment.to_string());
        } else if segment.starts_with("HCP*") {
            loop2400.hcp = Some(segment.to_string());
        }
    }
    
    loop2400
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2400_professional() {
        let content = "LX*1\nSV1*HC:99213*85*UN*1***1\nDTP*472*D8*20230115\nREF*6R*12345\nAMT*AAE*85\nQTY*NE*1";
        
        let loop2400 = parse_loop2400(content);
        
        assert_eq!(loop2400.lx, "LX*1");
        assert_eq!(loop2400.sv1, Some("SV1*HC:99213*85*UN*1***1".to_string()));
        assert_eq!(loop2400.dtp, vec!["DTP*472*D8*20230115".to_string()]);
        assert_eq!(loop2400.ref_segments, vec!["REF*6R*12345".to_string()]);
        assert_eq!(loop2400.amt, vec!["AMT*AAE*85".to_string()]);
        assert_eq!(loop2400.qty, vec!["QTY*NE*1".to_string()]);
    }
    
    #[test]
    fn test_parse_loop2400_institutional() {
        let content = "LX*1\nSV2*0450*HC:99283*150*UN*1\nDTP*472*D8*20230115\nREF*6R*12345";
        
        let loop2400 = parse_loop2400(content);
        
        assert_eq!(loop2400.lx, "LX*1");
        assert_eq!(loop2400.sv2, Some("SV2*0450*HC:99283*150*UN*1".to_string()));
        assert_eq!(loop2400.dtp, vec!["DTP*472*D8*20230115".to_string()]);
        assert_eq!(loop2400.ref_segments, vec!["REF*6R*12345".to_string()]);
    }
    
    #[test]
    fn test_parse_loop2400_dental() {
        let content = "LX*1\nSV3*AD:D2150*85*UN*1*2\nDTP*472*D8*20230115\nREF*6R*12345\nNTE*ADD*COMPOSITE FILLING";
        
        let loop2400 = parse_loop2400(content);
        
        assert_eq!(loop2400.lx, "LX*1");
        assert_eq!(loop2400.sv3, Some("SV3*AD:D2150*85*UN*1*2".to_string()));
        assert_eq!(loop2400.dtp, vec!["DTP*472*D8*20230115".to_string()]);
        assert_eq!(loop2400.ref_segments, vec!["REF*6R*12345".to_string()]);
        assert_eq!(loop2400.nte, vec!["NTE*ADD*COMPOSITE FILLING".to_string()]);
    }
    
    #[test]
    fn test_write_loop2400() {
        let mut loop2400 = Loop2400::default();
        loop2400.lx = "LX*1".to_string();
        loop2400.sv1 = Some("SV1*HC:99213*85*UN*1***1".to_string());
        loop2400.dtp.push("DTP*472*D8*20230115".to_string());
        loop2400.ref_segments.push("REF*6R*12345".to_string());
        loop2400.amt.push("AMT*AAE*85".to_string());
        loop2400.qty.push("QTY*NE*1".to_string());
        
        let result = write_loop2400(&loop2400);
        
        assert!(result.contains("LX*1\n"));
        assert!(result.contains("SV1*HC:99213*85*UN*1***1\n"));
        assert!(result.contains("DTP*472*D8*20230115\n"));
        assert!(result.contains("REF*6R*12345\n"));
        assert!(result.contains("AMT*AAE*85\n"));
        assert!(result.contains("QTY*NE*1\n"));
    }
}
