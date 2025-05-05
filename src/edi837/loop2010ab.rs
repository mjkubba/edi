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
pub fn parse_loop2010ab(content: &str) -> (Loop2010ab, String) {
    let mut loop2010ab = Loop2010ab::default();
    let mut remaining_content = content.to_string();
    
    // Parse NM1 segment
    if let Some(nm1_pos) = remaining_content.find("NM1*87*") {
        let nm1_end = remaining_content[nm1_pos..].find('~').unwrap_or(remaining_content.len()) + nm1_pos;
        loop2010ab.nm1 = remaining_content[nm1_pos..=nm1_end].to_string();
        remaining_content = remaining_content[nm1_end + 1..].to_string();
    }
    
    // Parse N3 segment
    if let Some(n3_pos) = remaining_content.find("N3*") {
        let n3_end = remaining_content[n3_pos..].find('~').unwrap_or(remaining_content.len()) + n3_pos;
        loop2010ab.n3 = remaining_content[n3_pos..=n3_end].to_string();
        remaining_content = remaining_content[n3_end + 1..].to_string();
    }
    
    // Parse N4 segment
    if let Some(n4_pos) = remaining_content.find("N4*") {
        let n4_end = remaining_content[n4_pos..].find('~').unwrap_or(remaining_content.len()) + n4_pos;
        loop2010ab.n4 = remaining_content[n4_pos..=n4_end].to_string();
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
        loop2010ab.ref_segments.push(remaining_content[ref_pos..=ref_end].to_string());
        remaining_content = remaining_content[ref_end + 1..].to_string();
    }
    
    (loop2010ab, remaining_content)
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
