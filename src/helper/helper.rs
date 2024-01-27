pub fn get_segment_contents(key:&str, contents:  &str) -> String {
    let segment_content = get_full_segment_contents(key,contents);
    println!("segment_content: {}",segment_content);
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
    contents.replace(&to_remove, "").trim_start_matches("~").to_string()
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