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


pub fn check_if_segement_in_loop(segment: &str, anchor: &str, contents:String) -> bool {
    let segment_pos = contents.find(&segment);
    let anchor_pos = contents.find(&anchor);
    if segment_pos < anchor_pos {
        return true;
    }
    return false;
}

pub fn check_for_expected_codes(codes: &str ,content:String) -> bool {
    if codes.contains(&content) {
        return true;
    }
    return false;
}


pub fn get_loop_contents(segment_start: &str, anchor: &str, contents: String) -> String {
    let mut tmp_contents= contents.clone();
    let remaining_loop_count= contents.matches(segment_start).count();
    if remaining_loop_count > 1 {
        let skipped_content = &contents[3..];
        let foundanchor = skipped_content.find(anchor).unwrap();
        tmp_contents = contents[..foundanchor+3].to_string();
    }
    tmp_contents
}


pub fn get_table2(contents:String) -> String {
    let mut tmp_contents= contents.clone();
    let remaining_clp_count= contents.matches("CLP").count();
    if remaining_clp_count > 1 {
        let skipped_content = &contents[3..];
        let foundclp = skipped_content.find("CLP").unwrap();
        tmp_contents = contents[..foundclp+3].to_string();
    }
    tmp_contents
}

pub fn get_999_2000(contents:String) -> String {
    let mut tmp_contents= contents.clone();
    let remaining_clp_count= contents.matches("AK2").count();
    if remaining_clp_count > 1 {
        let skipped_content = &contents[3..];
        let foundclp = skipped_content.find("AK2").unwrap();
        tmp_contents = contents[..foundclp+3].to_string();
    }
    tmp_contents
}



pub fn get_segment_contents(key:&str, contents:  &str) -> String {
    let segment_content = get_full_segment_contents(key,contents);
    info!("segment_content: {}",segment_content);
    let start_skip = key.len() + 1;
    let content = &segment_content[start_skip..];
    content.to_string()
}

pub fn get_full_segment_contents(key:&str, contents: &str) -> String {
    let nkey = key.to_string() + "*";
    let index = contents.find(&nkey).unwrap();
    let start = &contents[index..];
    let end = start.find("~").unwrap();
    let content = &start[..end];
    content.to_string()
}


pub fn content_trim(key: &str, contents:String) -> String {
    let to_remove = get_full_segment_contents(&key, &contents)+"~";
    contents.replacen(&to_remove, "", 1).trim_start_matches("~").to_string()
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
        assert_eq!(result, "DTM*472*20190324");
    }

    #[test]
    fn test_content_trim() {
        let key = "ST";
        let contents = "~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~BPR*I*132*C*CHK************20190331";
        let result = content_trim(key, contents.to_string());
        assert_eq!(result, "GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~BPR*I*132*C*CHK************20190331");
    }
}