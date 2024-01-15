#[derive(Debug)]
#[allow(dead_code)]
pub struct PER {
    contact_function_code: String,
    contact_name: String,
    contact_number_qualifier: String,
    contact_number: String,
}

pub fn get_per(per_content: &str) -> PER {
    let per_parts: Vec<&str> = per_content.split("*").collect();
    PER {
        contact_function_code: per_parts[0].to_string(),
        contact_name: per_parts[1].to_string(),
        contact_number_qualifier: per_parts[2].to_string(),
        contact_number: per_parts[3].to_string(),
    }
}