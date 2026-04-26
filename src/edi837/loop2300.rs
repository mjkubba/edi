use crate::edi837::loop2400::Loop2400;
use serde::{Deserialize, Serialize};

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
    /// NM1 segments within claim (rendering provider, etc.)
    pub nm1_segments: Vec<String>,
    /// PRV segments within claim sub-loops
    pub prv_segments: Vec<String>,
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

    // Write REF segments
    for ref_segment in &loop2300.ref_segments {
        result.push_str(ref_segment);
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

    // Write NM1 segments (rendering provider, attending, etc.)
    let mut prv_iter = loop2300.prv_segments.iter();
    for nm1 in &loop2300.nm1_segments {
        result.push_str(nm1);
        result.push_str("\n");
        // Write associated PRV if present
        if let Some(prv) = prv_iter.next() {
            result.push_str(prv);
            result.push_str("\n");
        }
    }

    result
}

/// Parse Loop2300 from EDI content
pub fn parse_loop2300(content: &str) -> (Loop2300, String) {
    let mut loop2300 = Loop2300::default();
    let mut remaining_content = content.to_string();

    // Parse CLM segment
    if let Some(clm_pos) = remaining_content.find("CLM*") {
        let clm_end = remaining_content[clm_pos..]
            .find('~')
            .unwrap_or(remaining_content.len())
            + clm_pos;
        loop2300.clm = remaining_content[clm_pos..=clm_end].to_string();
        remaining_content = remaining_content[clm_end + 1..].to_string();
    }

    // Process remaining segments sequentially until we hit a loop boundary
    loop {
        // Trim leading whitespace/newlines
        let trimmed = remaining_content.trim_start_matches(|c: char| c == '\n' || c == '\r');
        if trimmed.is_empty() {
            break;
        }

        // Check for loop boundaries — these signal end of Loop2300
        if trimmed.starts_with("CLM*")
            || trimmed.starts_with("LX*")
            || trimmed.starts_with("SE*")
            || trimmed.starts_with("HL*")
        {
            break;
        }

        // Find the end of the current segment
        let seg_end = trimmed.find('~').unwrap_or(trimmed.len());
        let segment = trimmed[..=seg_end].to_string();
        let after_segment = &trimmed[seg_end + 1..];

        // Categorize the segment
        if segment.starts_with("DTP*") {
            loop2300.dtp.push(segment);
        } else if segment.starts_with("PWK*") {
            loop2300.pwk.push(segment);
        } else if segment.starts_with("NTE*") {
            loop2300.nte.push(segment);
        } else if segment.starts_with("REF*") {
            loop2300.ref_segments.push(segment);
        } else if segment.starts_with("HI*") {
            loop2300.hi.push(segment);
        } else if segment.starts_with("AMT*") {
            loop2300.amt.push(segment);
        } else if segment.starts_with("QTY*") {
            loop2300.qty.push(segment);
        } else if segment.starts_with("PRV*") {
            // Check if this is a sub-loop PRV (after NM1) or claim-level PRV
            if !loop2300.nm1_segments.is_empty() {
                loop2300.prv_segments.push(segment);
            } else {
                loop2300.prv = Some(segment);
            }
        } else if segment.starts_with("HCP*") {
            loop2300.hcp = Some(segment);
        } else if segment.starts_with("TOO*") {
            loop2300.too_segments.push(segment);
        } else if segment.starts_with("CL1*") {
            loop2300.cl1_segments.push(segment);
        } else if segment.starts_with("NM1*") {
            loop2300.nm1_segments.push(segment);
        } else {
            // Unknown segment — skip it to avoid infinite loop
        }

        remaining_content = after_segment.to_string();
    }

    (loop2300, remaining_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_loop2300() {
        let content = "CLM*12345*100***11:B:1*Y*A*Y*Y*P~DTP*434*RD8*20230101-20230131~HI*BK:J4500*BF:R6889~REF*D9*12345~AMT*AU*100~QTY*CA*1~PRV*BI*PXC*207RC0000X~HCP*01*100**1~";

        let (loop2300, _) = parse_loop2300(content);

        assert_eq!(loop2300.clm, "CLM*12345*100***11:B:1*Y*A*Y*Y*P~");
        assert_eq!(
            loop2300.dtp,
            vec!["DTP*434*RD8*20230101-20230131~".to_string()]
        );
        assert_eq!(loop2300.hi, vec!["HI*BK:J4500*BF:R6889~".to_string()]);
        assert_eq!(loop2300.ref_segments, vec!["REF*D9*12345~".to_string()]);
        assert_eq!(loop2300.amt, vec!["AMT*AU*100~".to_string()]);
        assert_eq!(loop2300.qty, vec!["QTY*CA*1~".to_string()]);
        assert_eq!(loop2300.prv, Some("PRV*BI*PXC*207RC0000X~".to_string()));
        assert_eq!(loop2300.hcp, Some("HCP*01*100**1~".to_string()));
    }

    #[test]
    fn test_write_loop2300() {
        let mut loop2300 = Loop2300::default();
        loop2300.clm = "CLM*12345*100***11:B:1*Y*A*Y*Y*P~".to_string();
        loop2300
            .dtp
            .push("DTP*434*RD8*20230101-20230131~".to_string());
        loop2300.hi.push("HI*BK:J4500*BF:R6889~".to_string());
        loop2300.ref_segments.push("REF*D9*12345~".to_string());
        loop2300.amt.push("AMT*AU*100~".to_string());
        loop2300.qty.push("QTY*CA*1~".to_string());
        loop2300.prv = Some("PRV*BI*PXC*207RC0000X~".to_string());
        loop2300.hcp = Some("HCP*01*100**1~".to_string());

        let result = write_loop2300(&loop2300);

        assert!(result.contains("CLM*12345*100***11:B:1*Y*A*Y*Y*P~\n"));
        assert!(result.contains("DTP*434*RD8*20230101-20230131~\n"));
        assert!(result.contains("HI*BK:J4500*BF:R6889~\n"));
        assert!(result.contains("REF*D9*12345~\n"));
        assert!(result.contains("AMT*AU*100~\n"));
        assert!(result.contains("QTY*CA*1~\n"));
        assert!(result.contains("PRV*BI*PXC*207RC0000X~\n"));
        assert!(result.contains("HCP*01*100**1~\n"));
    }
}
