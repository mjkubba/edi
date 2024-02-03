use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DTM {
    pub date_time_qualifier: String,
    pub date_time: String,
}

pub fn get_dtm(dtm_content: String) -> DTM {
    let dtm_parts: Vec<&str> = dtm_content.split("*").collect();
    DTM {
        date_time_qualifier: dtm_parts[0].to_string(),
        date_time: dtm_parts[1].to_string(),
    }
}

pub fn write_dtm(dtm:DTM) -> String {
    let mut dtm_content = String::new();
    dtm_content.push_str("DTM*");
    dtm_content.push_str(&dtm.date_time_qualifier);
    dtm_content.push_str("*");
    dtm_content.push_str(&dtm.date_time);
    dtm_content.push_str("~");
    dtm_content
}
 