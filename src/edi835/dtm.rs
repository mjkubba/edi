#[derive(Debug)]
#[allow(dead_code)]
pub struct DTM {
    date_time_qualifier: String,
    date_time: String,
}

pub fn get_dtm(dtm_content: String) -> DTM {
    let dtm_parts: Vec<&str> = dtm_content.split("*").collect();
    DTM {
        date_time_qualifier: dtm_parts[0].to_string(),
        date_time: dtm_parts[1].to_string(),
    }
}