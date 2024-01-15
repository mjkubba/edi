#[derive(Debug)]
#[allow(dead_code)]
pub struct ST {
    transaction_set_id: String,
    transaction_set_control_number: String,
}

pub fn get_st(st_content: &str) -> ST {
    let st_parts: Vec<&str> = st_content.split("*").collect();
    ST {
        transaction_set_id: st_parts[0].to_string(),
        transaction_set_control_number: st_parts[1].to_string(),
    }
}
