use log::info;

/// Safely get element at index, returning empty string if absent.
/// Per X12 §B.1.1.3.10, trailing optional elements may be omitted.
pub fn get_element(parts: &[&str], index: usize) -> String {
    parts.get(index).unwrap_or(&"").to_string()
}

/// Build a segment string with proper X12 §B.1.1.3.10 trailing separator suppression.
/// All elements are joined with `*`, then trailing empty separators are removed
/// before appending `~`. Middle empty elements are preserved.
pub fn build_segment(elements: &[&str]) -> String {
    let joined = elements.join("*");
    let trimmed = joined.trim_end_matches('*');
    format!("{}~", trimmed)
}

pub fn check_if_segment_in_loop(segment: &str, anchor: &str, contents: &str) -> bool {
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

pub fn check_for_expected_codes(codes: &str, content: &str) -> bool {
    codes.contains(&content)
}

pub fn get_loop_contents(segment_start: &str, anchor: &str, contents: &str) -> String {
    let mut tmp_contents = contents.to_string();
    let remaining_loop_count = contents.matches(segment_start).count();

    if remaining_loop_count > 1 {
        let skip = segment_start.len();
        if let Some(skipped_content) = contents.get(skip..) {
            if let Some(foundanchor) = skipped_content.find(anchor) {
                tmp_contents = contents[..foundanchor + skip].to_string();
            }
        }
    }

    tmp_contents
}

pub fn get_table2(contents: &str) -> String {
    let mut tmp_contents = contents.to_string();
    let key = "CLP";
    let remaining_clp_count = contents.matches(key).count();

    if remaining_clp_count > 1 {
        let skip = key.len();
        if let Some(skipped_content) = contents.get(skip..) {
            if let Some(foundclp) = skipped_content.find(key) {
                tmp_contents = contents[..foundclp + skip].to_string();
            }
        }
    }

    tmp_contents
}


pub fn get_999_2000(contents: &str) -> String {
    let mut tmp_contents = contents.to_string();
    let remaining_ak2_count = contents.matches("AK2").count();

    if remaining_ak2_count > 1 {
        // Find the next AK2 segment
        if let Some(pos) = contents.find("AK2") {
            // Skip the first AK2 and find the next one
            if let Some(next_pos) = contents[pos + 3..].find("AK2") {
                // Get content up to the next AK2
                tmp_contents = contents[..pos + 3 + next_pos].to_string();
            }
        }
    }

    tmp_contents
}

/// Count occurrences of a segment identifier at segment boundaries only.
/// This avoids counting occurrences of the identifier inside other segment data
/// (e.g., "IK3" appearing inside a CTX segment value).
pub fn count_segment_starts(key: &str, contents: &str) -> usize {
    let nkey = format!("{}*", key);
    let mut count = 0;
    let mut search_from = 0;
    while let Some(pos) = contents[search_from..].find(&nkey) {
        let abs_pos = search_from + pos;
        if abs_pos == 0
            || contents.as_bytes()[abs_pos - 1] == b'~'
            || contents.as_bytes()[abs_pos - 1] == b'\n'
        {
            count += 1;
        }
        search_from = abs_pos + nkey.len();
    }
    count
}

/// Find the position of the next occurrence of a segment identifier at a segment
/// boundary, starting search after `skip` bytes. Returns the absolute position
/// in `contents`, or None if not found.
pub fn find_next_segment_start(key: &str, contents: &str, skip: usize) -> Option<usize> {
    let nkey = format!("{}*", key);
    let mut search_from = skip;
    while let Some(pos) = contents[search_from..].find(&nkey) {
        let abs_pos = search_from + pos;
        if abs_pos == 0
            || contents.as_bytes()[abs_pos - 1] == b'~'
            || contents.as_bytes()[abs_pos - 1] == b'\n'
        {
            return Some(abs_pos);
        }
        search_from = abs_pos + nkey.len();
    }
    None
}

/// Extract the content of a segment, stripping the segment ID and leading separator.
/// For `get_segment_contents("CLP", "CLP*val1*val2~...")`, returns `"val1*val2"`.
/// When callers split the result on `*`, index 0 = first data element (e.g., CLP01).
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
        }
        None => {
            info!("Warning: Segment {} not found in contents", key);
            String::new()
        }
    }
}


pub fn get_segment_contents_opt(key: &str, contents: &str) -> Option<String> {
    match get_full_segment_contents(key, contents) {
        Some(segment_content) => {
            let start_skip = key.len() + 1;
            if segment_content.len() > start_skip {
                Some(segment_content[start_skip..].to_string())
            } else {
                Some(String::new())
            }
        }
        None => None,
    }
}

// Helper function to extract content between LS and LE segments

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
        if index == 0
            || contents.chars().nth(index - 1) == Some('~')
            || contents.chars().nth(index - 1) == Some('\n')
        {
            let start = &contents[index..];
            if let Some(end) = start.find("~") {
                let content = &start[..end];
                return Some(content.to_string());
            }
        }
    }

    None
}

pub fn content_trim(key: &str, contents: &str) -> String {
    if let Some(to_remove) = get_full_segment_contents(key, contents) {
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
    contents.to_string()
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
        let result = content_trim(key, contents);
        assert_eq!(result, "GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~BPR*I*132*C*CHK************20190331");
    }

    #[test]
    fn test_check_if_segment_in_loop() {
        let contents = "NM1*IL*1*DOE*JOHN~REF*SY*123456789~RMR*ZZ*APTC**35~DTM*582****RD8*20120501-20140531~ENT*2~";

        // NM1 comes before ENT
        assert!(check_if_segment_in_loop(
            "NM1",
            "ENT",
            contents
        ));

        // REF comes before ENT
        assert!(check_if_segment_in_loop(
            "REF",
            "ENT",
            contents
        ));

        // RMR comes before ENT
        assert!(check_if_segment_in_loop(
            "RMR",
            "ENT",
            contents
        ));

        // DTM comes before ENT
        assert!(check_if_segment_in_loop(
            "DTM",
            "ENT",
            contents
        ));

        // ENT doesn't come before ENT
        assert!(!check_if_segment_in_loop(
            "ENT",
            "ENT",
            contents
        ));

        // Test with segment at the end (no anchor after it)
        let contents_end = "NM1*IL*1*DOE*JOHN~REF*SY*123456789~RMR*ZZ*APTC**35~";
        assert!(check_if_segment_in_loop(
            "RMR",
            "XYZ",
            contents_end
        ));
    }
}
#[test]
fn test_segment_not_found() {
    let key = "XYZ"; // Non-existent segment
    let contents = "PER*BL*JANE DOE*TE*9005555555~N1*PE*BAN DDS LLC*FI*999994703~";
    let result = get_segment_contents(key, contents);
    assert_eq!(result, "");
}
