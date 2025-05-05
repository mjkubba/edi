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
pub fn parse_loop2010aa(content: &str) -> Loop2010aa {
    let mut loop2010aa = Loop2010aa::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*85*") {
            loop2010aa.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010aa.n3 = segment.to_string();
        } else if segment.starts_with("N4*") {
            loop2010aa.n4 = segment.to_string();
        } else if segment.starts_with("REF*") {
            loop2010aa.ref_segments.push(segment.to_string());
        } else if segment.starts_with("PER*") {
            loop2010aa.per = Some(segment.to_string());
        }
    }
    
    loop2010aa
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
