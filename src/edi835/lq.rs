#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct LQ {
    lq01_code_list_qualifier: String,
    lq02_remark_code: String,
}

pub fn get_lq(lq_content: String) -> LQ {
    let lq_parts: Vec<&str> = lq_content.split("*").collect();
    LQ {
        lq01_code_list_qualifier: lq_parts[0].to_string(),
        lq02_remark_code: lq_parts[1].to_string(),
    }
}
