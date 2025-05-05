use serde::{Serialize, Deserialize};

/// Loop2010AB - Pay-to Address
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ab {
    /// Pay-to Provider Name
    pub nm1: String,
    /// Pay-to Provider Address
    pub n3: String,
    /// Pay-to Provider City, State, ZIP Code
    pub n4: String,
    /// Pay-to Provider Secondary Identification
    pub ref_segments: Vec<String>,
}

/// Write Loop2010AB to EDI format
pub fn write_loop2010ab(loop2010ab: &Loop2010ab) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010ab.nm1);
    result.push_str("\n");
    
    // Write N3 segment
    result.push_str(&loop2010ab.n3);
    result.push_str("\n");
    
    // Write N4 segment
    result.push_str(&loop2010ab.n4);
    result.push_str("\n");
    
    // Write REF segments
    for ref_segment in &loop2010ab.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010AB from EDI content
pub fn parse_loop2010ab(content: &str) -> Loop2010ab {
    let mut loop2010ab = Loop2010ab::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*87*") {
            loop2010ab.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010ab.n3 = segment.to_string();
        } else if segment.starts_with("N4*") {
            loop2010ab.n4 = segment.to_string();
        } else if segment.starts_with("REF*") {
            loop2010ab.ref_segments.push(segment.to_string());
        }
    }
    
    loop2010ab
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010ab() {
        let content = "NM1*87*2*ACME BILLING SERVICE****XX*1234567890\nN3*200 MAIN STREET\nN4*ANYTOWN*AL*35242\nREF*EI*987654321";
        
        let loop2010ab = parse_loop2010ab(content);
        
        assert_eq!(loop2010ab.nm1, "NM1*87*2*ACME BILLING SERVICE****XX*1234567890");
        assert_eq!(loop2010ab.n3, "N3*200 MAIN STREET");
        assert_eq!(loop2010ab.n4, "N4*ANYTOWN*AL*35242");
        assert_eq!(loop2010ab.ref_segments, vec!["REF*EI*987654321".to_string()]);
    }
    
    #[test]
    fn test_write_loop2010ab() {
        let mut loop2010ab = Loop2010ab::default();
        loop2010ab.nm1 = "NM1*87*2*ACME BILLING SERVICE****XX*1234567890".to_string();
        loop2010ab.n3 = "N3*200 MAIN STREET".to_string();
        loop2010ab.n4 = "N4*ANYTOWN*AL*35242".to_string();
        loop2010ab.ref_segments.push("REF*EI*987654321".to_string());
        
        let result = write_loop2010ab(&loop2010ab);
        
        assert!(result.contains("NM1*87*2*ACME BILLING SERVICE****XX*1234567890\n"));
        assert!(result.contains("N3*200 MAIN STREET\n"));
        assert!(result.contains("N4*ANYTOWN*AL*35242\n"));
        assert!(result.contains("REF*EI*987654321\n"));
    }
}
