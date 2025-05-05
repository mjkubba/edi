use serde::{Serialize, Deserialize};

/// Loop2010BB - Payer Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010bb {
    /// Payer Name
    pub nm1: String,
    /// Payer Address
    pub n3: Option<String>,
    /// Payer City, State, ZIP Code
    pub n4: Option<String>,
    /// Payer Secondary Identification
    pub ref_segments: Vec<String>,
}

/// Write Loop2010BB to EDI format
pub fn write_loop2010bb(loop2010bb: &Loop2010bb) -> String {
    let mut result = String::new();
    
    // Write NM1 segment
    result.push_str(&loop2010bb.nm1);
    result.push_str("\n");
    
    // Write N3 segment if present
    if let Some(n3) = &loop2010bb.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2010bb.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }
    
    // Write REF segments
    for ref_segment in &loop2010bb.ref_segments {
        result.push_str(ref_segment);
        result.push_str("\n");
    }
    
    result
}

/// Parse Loop2010BB from EDI content
pub fn parse_loop2010bb(content: &str) -> Loop2010bb {
    let mut loop2010bb = Loop2010bb::default();
    let segments: Vec<&str> = content.split('\n').collect();
    
    for segment in segments {
        if segment.starts_with("NM1*PR*") {
            loop2010bb.nm1 = segment.to_string();
        } else if segment.starts_with("N3*") {
            loop2010bb.n3 = Some(segment.to_string());
        } else if segment.starts_with("N4*") {
            loop2010bb.n4 = Some(segment.to_string());
        } else if segment.starts_with("REF*") {
            loop2010bb.ref_segments.push(segment.to_string());
        }
    }
    
    loop2010bb
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_loop2010bb() {
        let content = "NM1*PR*2*MEDICARE*****PI*00435\nN3*P.O. BOX 12345\nN4*ANYTOWN*PA*17111\nREF*2U*123456789\nREF*NF*12345";
        
        let loop2010bb = parse_loop2010bb(content);
        
        assert_eq!(loop2010bb.nm1, "NM1*PR*2*MEDICARE*****PI*00435");
        assert_eq!(loop2010bb.n3, Some("N3*P.O. BOX 12345".to_string()));
        assert_eq!(loop2010bb.n4, Some("N4*ANYTOWN*PA*17111".to_string()));
        assert_eq!(loop2010bb.ref_segments, vec!["REF*2U*123456789".to_string(), "REF*NF*12345".to_string()]);
    }
    
    #[test]
    fn test_write_loop2010bb() {
        let mut loop2010bb = Loop2010bb::default();
        loop2010bb.nm1 = "NM1*PR*2*MEDICARE*****PI*00435".to_string();
        loop2010bb.n3 = Some("N3*P.O. BOX 12345".to_string());
        loop2010bb.n4 = Some("N4*ANYTOWN*PA*17111".to_string());
        loop2010bb.ref_segments.push("REF*2U*123456789".to_string());
        loop2010bb.ref_segments.push("REF*NF*12345".to_string());
        
        let result = write_loop2010bb(&loop2010bb);
        
        assert!(result.contains("NM1*PR*2*MEDICARE*****PI*00435\n"));
        assert!(result.contains("N3*P.O. BOX 12345\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111\n"));
        assert!(result.contains("REF*2U*123456789\n"));
        assert!(result.contains("REF*NF*12345\n"));
    }
}
