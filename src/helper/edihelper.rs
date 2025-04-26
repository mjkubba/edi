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

pub fn get_999_2000(contents: String) -> String {
    let mut tmp_contents = contents.clone();
    let remaining_ak2_count = contents.matches("AK2").count();
    
    if remaining_ak2_count > 1 {
        if let Some(skipped_content) = contents.get(3..) {
            if let Some(foundak2) = skipped_content.find("AK2") {
                tmp_contents = contents[..foundak2+3].to_string();
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

pub fn get_full_segment_contents(key: &str, contents: &str) -> Option<String> {
    let nkey = key.to_string() + "*";
    
    if let Some(index) = contents.find(&nkey) {
        let start = &contents[index..];
        if let Some(end) = start.find("~") {
            let content = &start[..end];
            return Some(content.to_string());
        }
    }
    
    None
}

pub fn content_trim(key: &str, contents: String) -> String {
    if let Some(to_remove) = get_full_segment_contents(key, &contents) {
        let to_remove_with_tilde = to_remove + "~";
        contents.replacen(&to_remove_with_tilde, "", 1).trim_start_matches("~").to_string()
    } else {
        contents
    }
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
    fn test_segment_not_found() {
        let key = "XYZ"; // Non-existent segment
        let contents = "PER*BL*JANE DOE*TE*9005555555~N1*PE*BAN DDS LLC*FI*999994703~";
        let result = get_segment_contents(key, contents);
        assert_eq!(result, "");
    }
}
