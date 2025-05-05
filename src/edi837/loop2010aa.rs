use serde::{Serialize, Deserialize};

/// Loop2010AA - Billing Provider Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010aa {
    /// Billing Provider Name
    pub nm1: String,
    /// Billing Provider Address
    pub n3: String,
    /// Billing Provider City, State, ZIP Code
    pub n4: String,
    /// Billing Provider Tax Identification
    pub ref_segments: Vec<String>,
    /// Billing Provider Contact Information
    pub per: Option<String>,
}

/// Write Loop2010AA to EDI format
pub fn write_loop2010aa(loop2010aa: &Loop2010aa) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010aa.nm1);
    result.push_str("\n");
    
    // Write N3 segment
    result.push_str(&loop2010aa.n3);
    result.push_str("\n");
    
    // Write N4 segment
    result.push_str(&loop2010aa.n4);
    result.push_str("\n");
    
    // Write REF segments
    for ref_segment in &loop2010aa.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    // Write PER segment if present
    if let Some(per) = &loop2010aa.per {
        result.push_str(per);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010AA from EDI content
pub fn parse_loop2010aa(content: &str) -> (Loop2010aa, String) {
    let mut loop2010aa = Loop2010aa::default();
    let mut remaining_content = content.to_string();
    
    // Parse NM1 segment
    if let Some(nm1_pos) = remaining_content.find("NM1*85*") {
        let nm1_end = remaining_content[nm1_pos..].find('~').unwrap_or(remaining_content.len()) + nm1_pos;
        loop2010aa.nm1 = remaining_content[nm1_pos..=nm1_end].to_string();
        remaining_content = remaining_content[nm1_end + 1..].to_string();
    }
    
    // Parse N3 segment
    if let Some(n3_pos) = remaining_content.find("N3*") {
        let n3_end = remaining_content[n3_pos..].find('~').unwrap_or(remaining_content.len()) + n3_pos;
        loop2010aa.n3 = remaining_content[n3_pos..=n3_end].to_string();
        remaining_content = remaining_content[n3_end + 1..].to_string();
    }
    
    // Parse N4 segment
    if let Some(n4_pos) = remaining_content.find("N4*") {
        let n4_end = remaining_content[n4_pos..].find('~').unwrap_or(remaining_content.len()) + n4_pos;
        loop2010aa.n4 = remaining_content[n4_pos..=n4_end].to_string();
        remaining_content = remaining_content[n4_end + 1..].to_string();
    }
    
    // Parse REF segments
    while let Some(ref_pos) = remaining_content.find("REF*") {
        // Check if this REF belongs to this loop or the next one
        if remaining_content[..ref_pos].contains("NM1*") || 
           remaining_content[..ref_pos].contains("HL*") {
            break;
        }
        
        let ref_end = remaining_content[ref_pos..].find('~').unwrap_or(remaining_content.len()) + ref_pos;
        loop2010aa.ref_segments.push(remaining_content[ref_pos..=ref_end].to_string());
        remaining_content = remaining_content[ref_end + 1..].to_string();
    }
    
    // Parse PER segment
    if let Some(per_pos) = remaining_content.find("PER*") {
        // Check if this PER belongs to this loop or the next one
        if !remaining_content[..per_pos].contains("NM1*") && 
           !remaining_content[..per_pos].contains("HL*") {
            let per_end = remaining_content[per_pos..].find('~').unwrap_or(remaining_content.len()) + per_pos;
            loop2010aa.per = Some(remaining_content[per_pos..=per_end].to_string());
            remaining_content = remaining_content[per_end + 1..].to_string();
        }
    }
    
    (loop2010aa, remaining_content)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010aa() {
        let content = "NM1*85*2*ACME MEDICAL GROUP****XX*1234567890\nN3*100 MAIN STREET\nN4*ANYTOWN*AL*35242\nREF*EI*123456789\nPER*IC*JANE SMITH*TE*5551234567";
        
        let loop2010aa = parse_loop2010aa(content);
        
        assert_eq!(loop2010aa.nm1, "NM1*85*2*ACME MEDICAL GROUP****XX*1234567890");
        assert_eq!(loop2010aa.n3, "N3*100 MAIN STREET");
        assert_eq!(loop2010aa.n4, "N4*ANYTOWN*AL*35242");
        assert_eq!(loop2010aa.ref_segments, vec!["REF*EI*123456789".to_string()]);
        assert_eq!(loop2010aa.per, Some("PER*IC*JANE SMITH*TE*5551234567".to_string()));
    }
    
    #[test]
    fn test_write_loop2010aa() {
        let mut loop2010aa = Loop2010aa::default();
        loop2010aa.nm1 = "NM1*85*2*ACME MEDICAL GROUP****XX*1234567890".to_string();
        loop2010aa.n3 = "N3*100 MAIN STREET".to_string();
        loop2010aa.n4 = "N4*ANYTOWN*AL*35242".to_string();
        loop2010aa.ref_segments.push("REF*EI*123456789".to_string());
        loop2010aa.per = Some("PER*IC*JANE SMITH*TE*5551234567".to_string());
        
        let result = write_loop2010aa(&loop2010aa);
        
        assert!(result.contains("NM1*85*2*ACME MEDICAL GROUP****XX*1234567890\n"));
        assert!(result.contains("N3*100 MAIN STREET\n"));
        assert!(result.contains("N4*ANYTOWN*AL*35242\n"));
        assert!(result.contains("REF*EI*123456789\n"));
        assert!(result.contains("PER*IC*JANE SMITH*TE*5551234567\n"));
    }
}
