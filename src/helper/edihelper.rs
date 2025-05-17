use log::info;

pub fn stiuational_element(value: String) -> String {
    if !value.is_empty() {
        let mut to_return = String::from("*");
        to_return.push_str(&value);
        return to_return;
    } else {
        return "".to_string();
    }
}

pub fn check_if_segement_in_loop(segment: &str, anchor: &str, contents: String) -> bool {
    if let (Some(segment_pos), Some(anchor_pos)) = (contents.find(segment), contents.find(anchor)) {
        return segment_pos < anchor_pos;
    }
    // If anchor is not found, assume segment is in the loop
    // This helps with segments at the end of the file
    if contents.find(segment).is_some() && contents.find(anchor).is_none() {
        return true;
    }
    false
}

pub fn check_for_expected_codes(codes: &str, content: String) -> bool {
    codes.contains(&content)
}

pub fn get_loop_contents(segment_start: &str, anchor: &str, contents: String) -> String {
    let mut tmp_contents = contents.clone();
    let remaining_loop_count = contents.matches(segment_start).count();
    
    if remaining_loop_count > 1 {
        if let Some(skipped_content) = contents.get(3..) {
            if let Some(foundanchor) = skipped_content.find(anchor) {
                tmp_contents = contents[..foundanchor+3].to_string();
            }
        }
    }
    
    tmp_contents
}

pub fn get_table2(contents: String) -> String {
    let mut tmp_contents = contents.clone();
    let remaining_clp_count = contents.matches("CLP").count();
    
    if remaining_clp_count > 1 {
        if let Some(skipped_content) = contents.get(3..) {
            if let Some(foundclp) = skipped_content.find("CLP") {
                tmp_contents = contents[..foundclp+3].to_string();
            }
        }
    }
    
    tmp_contents
}

#[allow(dead_code)]
pub fn get_999_2000(contents: String) -> String {
    let mut tmp_contents = contents.clone();
    let remaining_ak2_count = contents.matches("AK2").count();
    
    if remaining_ak2_count > 1 {
        // Find the next AK2 segment
        if let Some(pos) = contents.find("AK2") {
            // Skip the first AK2 and find the next one
            if let Some(next_pos) = contents[pos+3..].find("AK2") {
                // Get content up to the next AK2
                tmp_contents = contents[..pos+3+next_pos].to_string();
            }
        }
    }
    
    tmp_contents
}

pub fn get_segment_contents(key: &str, contents: &str) -> String {
    match get_full_segment_contents(key, contents) {
        Some(segment_content) => {
            info!("segment_content: {}", segment_content);
            let start_skip = key.len() + 1;
            if segment_content.len() > start_skip {
                segment_content[start_skip..].to_string()
            } else {
                String::new()
            }
        },
        None => {
            info!("Warning: Segment {} not found in contents", key);
            String::new()
        }
    }
}

#[allow(dead_code)]
pub fn get_segment_contents_opt(key: &str, contents: &str) -> Option<String> {
    match get_full_segment_contents(key, contents) {
        Some(segment_content) => {
            let start_skip = key.len() + 1;
            if segment_content.len() > start_skip {
                Some(segment_content[start_skip..].to_string())
            } else {
                Some(String::new())
            }
        },
        None => None
    }
}

// Helper function to extract content between LS and LE segments
#[allow(dead_code)]
pub fn extract_between_ls_le(contents: &str) -> Option<String> {
    if let Some(ls_pos) = contents.find("LS*") {
        // Find the end of the LS segment
        if let Some(ls_end) = contents[ls_pos..].find('~') {
            let ls_segment_end = ls_pos + ls_end + 1;
            
            // Find the LE segment after the LS segment
            if let Some(le_pos) = contents[ls_segment_end..].find("LE*") {
                let le_pos_absolute = ls_segment_end + le_pos;
                
                // Return the content between LS~ and LE*
                return Some(contents[ls_segment_end..le_pos_absolute].to_string());
            }
        }
    }
    None
}

// Helper function to get the loop identifier code from LS or LE segment
#[allow(dead_code)]
pub fn get_loop_identifier_code(segment_content: &str) -> String {
    let parts: Vec<&str> = segment_content.split('*').collect();
    if parts.len() > 1 {
        parts[1].trim_end_matches('~').to_string()
    } else {
        String::new()
    }
}

pub fn get_full_segment_contents(key: &str, contents: &str) -> Option<String> {
    let nkey = key.to_string() + "*";
    
    if let Some(index) = contents.find(&nkey) {
        // Make sure we're at the start of a segment
        if index == 0 || contents.chars().nth(index - 1) == Some('~') || contents.chars().nth(index - 1) == Some('\n') {
            let start = &contents[index..];
            if let Some(end) = start.find("~") {
                let content = &start[..end];
                return Some(content.to_string());
            }
        }
    }
    
    None
}

pub fn content_trim(key: &str, contents: String) -> String {
    if let Some(to_remove) = get_full_segment_contents(key, &contents) {
        let to_remove_with_tilde = to_remove.clone() + "~";
        
        // Check if the segment exists in the content
        if contents.contains(&to_remove_with_tilde) {
            let trimmed = contents.replacen(&to_remove_with_tilde, "", 1);
            return trimmed.trim_start_matches("~").to_string();
        }
    }
    
    // If we couldn't find or remove the segment, return the original content
    // This helps prevent infinite loops
    info!("Warning: Failed to trim segment {} from content", key);
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_segment_contents() {
        let key = "N1";
        let contents = "PER*BL*JANE DOE*TE*9005555555~N1*PE*BAN DDS LLC*FI*999994703~";
        let result = get_segment_contents(key, contents);
        assert_eq!(result, "PE*BAN DDS LLC*FI*999994703");
    }

    #[test]
    fn test_get_segment_contents_opt() {
        let key = "DTM";
        let contents = "SVC*AD|D1110*73*49~DTM*472*20190324~CAS*CO*131*24~AMT*B6*49~";
        let result = get_segment_contents_opt(key, contents);
        assert_eq!(result, Some("472*20190324".to_string()));
        
        let key = "XYZ"; // Non-existent segment
        let result = get_segment_contents_opt(key, contents);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_full_segment_contents() {
        let key = "DTM";
        let contents = "SVC*AD|D1110*73*49~DTM*472*20190324~CAS*CO*131*24~AMT*B6*49~";
        let result = get_full_segment_contents(key, contents);
        assert_eq!(result, Some("DTM*472*20190324".to_string()));
    }

    #[test]
    fn test_content_trim() {
        let key = "ST";
        let contents = "~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~BPR*I*132*C*CHK************20190331";
        let result = content_trim(key, contents.to_string());
        assert_eq!(result, "GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~BPR*I*132*C*CHK************20190331");
    }
    
    #[test]
    fn test_check_if_segement_in_loop() {
        let contents = "NM1*IL*1*DOE*JOHN~REF*SY*123456789~RMR*ZZ*APTC**35~DTM*582****RD8*20120501-20140531~ENT*2~";
        
        // NM1 comes before ENT
        assert!(check_if_segement_in_loop("NM1", "ENT", contents.to_string()));
        
        // REF comes before ENT
        assert!(check_if_segement_in_loop("REF", "ENT", contents.to_string()));
        
        // RMR comes before ENT
        assert!(check_if_segement_in_loop("RMR", "ENT", contents.to_string()));
        
        // DTM comes before ENT
        assert!(check_if_segement_in_loop("DTM", "ENT", contents.to_string()));
        
        // ENT doesn't come before ENT
        assert!(!check_if_segement_in_loop("ENT", "ENT", contents.to_string()));
        
        // Test with segment at the end (no anchor after it)
        let contents_end = "NM1*IL*1*DOE*JOHN~REF*SY*123456789~RMR*ZZ*APTC**35~";
        assert!(check_if_segement_in_loop("RMR", "XYZ", contents_end.to_string()));
    }
    
}
    #[test]
    fn test_segment_not_found() {
        let key = "XYZ"; // Non-existent segment
        let contents = "PER*BL*JANE DOE*TE*9005555555~N1*PE*BAN DDS LLC*FI*999994703~";
        let result = get_segment_contents(key, contents);
        assert_eq!(result, "");
    }
