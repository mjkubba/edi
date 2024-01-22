pub fn get_segment_contents(key:&str, contents:  &str) -> String {
    let segment_content = get_full_segment_contents(key,contents);
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

// fn content_trim(key: &str, contents:String) -> String {
//     contents.trim_start_matches(&get_full_segment_contents(&key, &contents)).trim_start_matches("~").to_string()
// }

pub fn content_trim(key: &str, contents:String) -> String {
    contents.replace(&get_full_segment_contents(&key, &contents), "").trim_start_matches("~").to_string()
}