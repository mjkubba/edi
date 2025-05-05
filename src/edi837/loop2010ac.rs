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
pub fn parse_loop2010ac(content: &str) -> (Loop2010ac, String) {
    let mut loop2010ac = Loop2010ac::default();
    let mut remaining_content = content.to_string();
    
    // Parse NM1 segment
    if let Some(nm1_pos) = remaining_content.find("NM1*PE*") {
        let nm1_end = remaining_content[nm1_pos..].find('~').unwrap_or(remaining_content.len()) + nm1_pos;
        loop2010ac.nm1 = remaining_content[nm1_pos..=nm1_end].to_string();
        remaining_content = remaining_content[nm1_end + 1..].to_string();
    }
    
    // Parse N3 segment if present
    if let Some(n3_pos) = remaining_content.find("N3*") {
        // Check if this N3 belongs to this loop or the next one
        if !remaining_content[..n3_pos].contains("NM1*") && 
           !remaining_content[..n3_pos].contains("HL*") {
            let n3_end = remaining_content[n3_pos..].find('~').unwrap_or(remaining_content.len()) + n3_pos;
            loop2010ac.n3 = Some(remaining_content[n3_pos..=n3_end].to_string());
            remaining_content = remaining_content[n3_end + 1..].to_string();
        }
    }
    
    // Parse N4 segment if present
    if let Some(n4_pos) = remaining_content.find("N4*") {
        // Check if this N4 belongs to this loop or the next one
        if !remaining_content[..n4_pos].contains("NM1*") && 
           !remaining_content[..n4_pos].contains("HL*") {
            let n4_end = remaining_content[n4_pos..].find('~').unwrap_or(remaining_content.len()) + n4_pos;
            loop2010ac.n4 = Some(remaining_content[n4_pos..=n4_end].to_string());
            remaining_content = remaining_content[n4_end + 1..].to_string();
        }
    }
    
    // Parse REF segments
    while let Some(ref_pos) = remaining_content.find("REF*") {
        // Check if this REF belongs to this loop or the next one
        if remaining_content[..ref_pos].contains("NM1*") || 
           remaining_content[..ref_pos].contains("HL*") {
            break;
        }
        
        let ref_end = remaining_content[ref_pos..].find('~').unwrap_or(remaining_content.len()) + ref_pos;
        loop2010ac.ref_segments.push(remaining_content[ref_pos..=ref_end].to_string());
        remaining_content = remaining_content[ref_end + 1..].to_string();
    }
    
    (loop2010ac, remaining_content)
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
