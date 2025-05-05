use serde::{Serialize, Deserialize};

/// Loop2300 - Claim Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    /// Claim Information
    pub clm: String,
    /// Claim Dates
    pub dtp: Vec<String>,
    /// Claim Supplemental Information
    pub pwk: Vec<String>,
    /// Claim Note
    pub nte: Vec<String>,
    /// Health Care Information Codes
    pub hi: Vec<String>,
    /// Claim Amounts
    pub amt: Vec<String>,
    /// Claim Quantities
    pub qty: Vec<String>,
    /// Claim Identifications
    pub ref_segments: Vec<String>,
    /// Claim Provider Information
    pub prv: Option<String>,
    /// Claim Pricing/Repricing Information
    pub hcp: Option<String>,
}

/// Write Loop2300 to EDI format
pub fn write_loop2300(loop2300: &Loop2300) -> String {
    let mut result = String::new();
    
    // Write CLM segment
    result.push_str(&loop2300.clm);
    result.push_str("\n");
    
    // Write DTP segments
    for dtp in &loop2300.dtp {
        result.push_str(dtp);
        result.push_str("\n");
    }
    
    // Write PWK segments
    for pwk in &loop2300.pwk {
        result.push_str(pwk);
        result.push_str("\n");
    }
    
    // Write NTE segments
    for nte in &loop2300.nte {
        result.push_str(nte);
        result.push_str("\n");
    }
    
    // Write HI segments
    for hi in &loop2300.hi {
        result.push_str(hi);
        result.push_str("\n");
    }
    
    // Write AMT segments
    for amt in &loop2300.amt {
        result.push_str(amt);
        result.push_str("\n");
    }
    
    // Write QTY segments
    for qty in &loop2300.qty {
        result.push_str(qty);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2300.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    // Write PRV segment if present
    if let Some(prv) = &loop2300.prv {
        result.push_str(prv);
        result.push_str("\n");
    }
    
    // Write HCP segment if present
    if let Some(hcp) = &loop2300.hcp {
        result.push_str(hcp);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2300 from EDI content
pub fn parse_loop2300(content: &str) -> Loop2300 {
    let mut loop2300 = Loop2300::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("CLM*") {
            loop2300.clm = segment.to_string();
        } else if segment.starts_with("DTP*") {
            loop2300.dtp.push(segment.to_string());
        } else if segment.starts_with("PWK*") {
            loop2300.pwk.push(segment.to_string());
        } else if segment.starts_with("NTE*") {
            loop2300.nte.push(segment.to_string());
        } else if segment.starts_with("HI*") {
            loop2300.hi.push(segment.to_string());
        } else if segment.starts_with("AMT*") {
            loop2300.amt.push(segment.to_string());
        } else if segment.starts_with("QTY*") {
            loop2300.qty.push(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2300.ref_segments.push(segment.to_string());
        } else if segment.starts_with("PRV*") {
            loop2300.prv = Some(segment.to_string());
        } else if segment.starts_with("HCP*") {
            loop2300.hcp = Some(segment.to_string());
        }
    }
    
    loop2300
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2300() {
        let content = "CLM*12345*100***11:B:1*Y*A*Y*Y*P\nDTP*434*RD8*20230101-20230131\nHI*BK:J4500*BF:R6889\nREF*D9*12345\nAMT*AU*100\nQTY*CA*1\nPRV*BI*PXC*207RC0000X\nHCP*01*100**1";
        
        let loop2300 = parse_loop2300(content);
        
        assert_eq!(loop2300.clm, "CLM*12345*100***11:B:1*Y*A*Y*Y*P");
        assert_eq!(loop2300.dtp, vec!["DTP*434*RD8*20230101-20230131".to_string()]);
        assert_eq!(loop2300.hi, vec!["HI*BK:J4500*BF:R6889".to_string()]);
        assert_eq!(loop2300.ref_segments, vec!["REF*D9*12345".to_string()]);
        assert_eq!(loop2300.amt, vec!["AMT*AU*100".to_string()]);
        assert_eq!(loop2300.qty, vec!["QTY*CA*1".to_string()]);
        assert_eq!(loop2300.prv, Some("PRV*BI*PXC*207RC0000X".to_string()));
        assert_eq!(loop2300.hcp, Some("HCP*01*100**1".to_string()));
    }
    
    #[test]
    fn test_write_loop2300() {
        let mut loop2300 = Loop2300::default();
        loop2300.clm = "CLM*12345*100***11:B:1*Y*A*Y*Y*P".to_string();
        loop2300.dtp.push("DTP*434*RD8*20230101-20230131".to_string());
        loop2300.hi.push("HI*BK:J4500*BF:R6889".to_string());
        loop2300.ref_segments.push("REF*D9*12345".to_string());
        loop2300.amt.push("AMT*AU*100".to_string());
        loop2300.qty.push("QTY*CA*1".to_string());
        loop2300.prv = Some("PRV*BI*PXC*207RC0000X".to_string());
        loop2300.hcp = Some("HCP*01*100**1".to_string());
        
        let result = write_loop2300(&loop2300);
        
        assert!(result.contains("CLM*12345*100***11:B:1*Y*A*Y*Y*P\n"));
        assert!(result.contains("DTP*434*RD8*20230101-20230131\n"));
        assert!(result.contains("HI*BK:J4500*BF:R6889\n"));
        assert!(result.contains("REF*D9*12345\n"));
        assert!(result.contains("AMT*AU*100\n"));
        assert!(result.contains("QTY*CA*1\n"));
        assert!(result.contains("PRV*BI*PXC*207RC0000X\n"));
        assert!(result.contains("HCP*01*100**1\n"));
    }
}
