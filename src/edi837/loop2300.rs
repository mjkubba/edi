use serde::{Serialize, Deserialize};
use crate::edi837::loop2400::Loop2400;

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
    /// Service Line Information
    pub loop2400: Vec<Loop2400>,
    // Add fields for specialized segments
    pub too_segments: Vec<String>,
    pub cl1_segments: Vec<String>,
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
    
    // Write TOO segments (specific to 837D)
    for too in &loop2300.too_segments {
        result.push_str(too);
        result.push_str("\n");
    }
    
    // Write CL1 segments (specific to 837I)
    for cl1 in &loop2300.cl1_segments {
        result.push_str(cl1);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2300 from EDI content
pub fn parse_loop2300(content: &str) -> (Loop2300, String) {
    let mut loop2300 = Loop2300::default();
    let mut remaining_content = content.to_string();
    
    // Parse CLM segment
    if let Some(clm_pos) = remaining_content.find("CLM*") {
        let clm_end = remaining_content[clm_pos..].find('~').unwrap_or(remaining_content.len()) + clm_pos;
        loop2300.clm = remaining_content[clm_pos..=clm_end].to_string();
        remaining_content = remaining_content[clm_end + 1..].to_string();
    }
    
    // Parse DTP segments
    while let Some(dtp_pos) = remaining_content.find("DTP*") {
        // Check if this DTP belongs to this loop or the next one
        if remaining_content[..dtp_pos].contains("CLM*") || 
           remaining_content[..dtp_pos].contains("LX*") {
            break;
        }
        
        let dtp_end = remaining_content[dtp_pos..].find('~').unwrap_or(remaining_content.len()) + dtp_pos;
        loop2300.dtp.push(remaining_content[dtp_pos..=dtp_end].to_string());
        remaining_content = remaining_content[dtp_end + 1..].to_string();
    }
    
    // Parse PWK segments
    while let Some(pwk_pos) = remaining_content.find("PWK*") {
        // Check if this PWK belongs to this loop or the next one
        if remaining_content[..pwk_pos].contains("CLM*") || 
           remaining_content[..pwk_pos].contains("LX*") {
            break;
        }
        
        let pwk_end = remaining_content[pwk_pos..].find('~').unwrap_or(remaining_content.len()) + pwk_pos;
        loop2300.pwk.push(remaining_content[pwk_pos..=pwk_end].to_string());
        remaining_content = remaining_content[pwk_end + 1..].to_string();
    }
    
    // Parse NTE segments
    while let Some(nte_pos) = remaining_content.find("NTE*") {
        // Check if this NTE belongs to this loop or the next one
        if remaining_content[..nte_pos].contains("CLM*") || 
           remaining_content[..nte_pos].contains("LX*") {
            break;
        }
        
        let nte_end = remaining_content[nte_pos..].find('~').unwrap_or(remaining_content.len()) + nte_pos;
        loop2300.nte.push(remaining_content[nte_pos..=nte_end].to_string());
        remaining_content = remaining_content[nte_end + 1..].to_string();
    }
    
    // Parse HI segments
    while let Some(hi_pos) = remaining_content.find("HI*") {
        // Check if this HI belongs to this loop or the next one
        if remaining_content[..hi_pos].contains("CLM*") || 
           remaining_content[..hi_pos].contains("LX*") {
            break;
        }
        
        let hi_end = remaining_content[hi_pos..].find('~').unwrap_or(remaining_content.len()) + hi_pos;
        loop2300.hi.push(remaining_content[hi_pos..=hi_end].to_string());
        remaining_content = remaining_content[hi_end + 1..].to_string();
    }
    
    // Parse AMT segments
    while let Some(amt_pos) = remaining_content.find("AMT*") {
        // Check if this AMT belongs to this loop or the next one
        if remaining_content[..amt_pos].contains("CLM*") || 
           remaining_content[..amt_pos].contains("LX*") {
            break;
        }
        
        let amt_end = remaining_content[amt_pos..].find('~').unwrap_or(remaining_content.len()) + amt_pos;
        loop2300.amt.push(remaining_content[amt_pos..=amt_end].to_string());
        remaining_content = remaining_content[amt_end + 1..].to_string();
    }
    
    // Parse QTY segments
    while let Some(qty_pos) = remaining_content.find("QTY*") {
        // Check if this QTY belongs to this loop or the next one
        if remaining_content[..qty_pos].contains("CLM*") || 
           remaining_content[..qty_pos].contains("LX*") {
            break;
        }
        
        let qty_end = remaining_content[qty_pos..].find('~').unwrap_or(remaining_content.len()) + qty_pos;
        loop2300.qty.push(remaining_content[qty_pos..=qty_end].to_string());
        remaining_content = remaining_content[qty_end + 1..].to_string();
    }
    
    // Parse REF segments
    while let Some(ref_pos) = remaining_content.find("REF*") {
        // Check if this REF belongs to this loop or the next one
        if remaining_content[..ref_pos].contains("CLM*") || 
           remaining_content[..ref_pos].contains("LX*") {
            break;
        }
        
        let ref_end = remaining_content[ref_pos..].find('~').unwrap_or(remaining_content.len()) + ref_pos;
        loop2300.ref_segments.push(remaining_content[ref_pos..=ref_end].to_string());
        remaining_content = remaining_content[ref_end + 1..].to_string();
    }
    
    // Parse PRV segment if present
    if let Some(prv_pos) = remaining_content.find("PRV*") {
        // Check if this PRV belongs to this loop or the next one
        if !remaining_content[..prv_pos].contains("CLM*") && 
           !remaining_content[..prv_pos].contains("LX*") {
            let prv_end = remaining_content[prv_pos..].find('~').unwrap_or(remaining_content.len()) + prv_pos;
            loop2300.prv = Some(remaining_content[prv_pos..=prv_end].to_string());
            remaining_content = remaining_content[prv_end + 1..].to_string();
        }
    }
    
    // Parse HCP segment if present
    if let Some(hcp_pos) = remaining_content.find("HCP*") {
        // Check if this HCP belongs to this loop or the next one
        if !remaining_content[..hcp_pos].contains("CLM*") && 
           !remaining_content[..hcp_pos].contains("LX*") {
            let hcp_end = remaining_content[hcp_pos..].find('~').unwrap_or(remaining_content.len()) + hcp_pos;
            loop2300.hcp = Some(remaining_content[hcp_pos..=hcp_end].to_string());
            remaining_content = remaining_content[hcp_end + 1..].to_string();
        }
    }
    
    // Parse TOO segments (specific to 837D)
    while let Some(too_pos) = remaining_content.find("TOO*") {
        // Check if this TOO belongs to this loop or the next one
        if remaining_content[..too_pos].contains("LX*") {
            break;
        }
        
        let too_end = remaining_content[too_pos..].find('~').unwrap_or(remaining_content.len()) + too_pos;
        loop2300.too_segments.push(remaining_content[too_pos..=too_end].to_string());
        remaining_content = remaining_content[too_end + 1..].to_string();
    }
    
    // Parse CL1 segments (specific to 837I)
    while let Some(cl1_pos) = remaining_content.find("CL1*") {
        // Check if this CL1 belongs to this loop or the next one
        if remaining_content[..cl1_pos].contains("LX*") {
            break;
        }
        
        let cl1_end = remaining_content[cl1_pos..].find('~').unwrap_or(remaining_content.len()) + cl1_pos;
        loop2300.cl1_segments.push(remaining_content[cl1_pos..=cl1_end].to_string());
        remaining_content = remaining_content[cl1_end + 1..].to_string();
    }
    
    (loop2300, remaining_content)
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
