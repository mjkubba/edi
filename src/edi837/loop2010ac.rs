use serde::{Serialize, Deserialize};

/// Loop2010AC - Pay-to Plan Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ac {
    /// Pay-to Plan Name
    pub nm1: String,
    /// Pay-to Plan Address
    pub n3: Option<String>,
    /// Pay-to Plan City, State, ZIP Code
    pub n4: Option<String>,
    /// Pay-to Plan Secondary Identification
    pub ref_segments: Vec<String>,
}

/// Write Loop2010AC to EDI format
pub fn write_loop2010ac(loop2010ac: &Loop2010ac) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010ac.nm1);
    result.push_str("\n");
    
    // Write N3 segment if present
    if let Some(n3) = &loop2010ac.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2010ac.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2010ac.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010AC from EDI content
pub fn parse_loop2010ac(content: &str) -> Loop2010ac {
    let mut loop2010ac = Loop2010ac::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*PE*") {
            loop2010ac.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010ac.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2010ac.n4 = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2010ac.ref_segments.push(segment.to_string());
        }
    }
    
    loop2010ac
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010ac() {
        let content = "NM1*PE*2*INSURANCE COMPANY****PI*12345\nN3*300 MAIN STREET\nN4*ANYTOWN*AL*35242\nREF*2U*123456789";
        
        let loop2010ac = parse_loop2010ac(content);
        
        assert_eq!(loop2010ac.nm1, "NM1*PE*2*INSURANCE COMPANY****PI*12345");
        assert_eq!(loop2010ac.n3, Some("N3*300 MAIN STREET".to_string()));
        assert_eq!(loop2010ac.n4, Some("N4*ANYTOWN*AL*35242".to_string()));
        assert_eq!(loop2010ac.ref_segments, vec!["REF*2U*123456789".to_string()]);
    }
    
    #[test]
    fn test_write_loop2010ac() {
        let mut loop2010ac = Loop2010ac::default();
        loop2010ac.nm1 = "NM1*PE*2*INSURANCE COMPANY****PI*12345".to_string();
        loop2010ac.n3 = Some("N3*300 MAIN STREET".to_string());
        loop2010ac.n4 = Some("N4*ANYTOWN*AL*35242".to_string());
        loop2010ac.ref_segments.push("REF*2U*123456789".to_string());
        
        let result = write_loop2010ac(&loop2010ac);
        
        assert!(result.contains("NM1*PE*2*INSURANCE COMPANY****PI*12345\n"));
        assert!(result.contains("N3*300 MAIN STREET\n"));
        assert!(result.contains("N4*ANYTOWN*AL*35242\n"));
        assert!(result.contains("REF*2U*123456789\n"));
    }
}
