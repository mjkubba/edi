use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DTM {
    pub date_time_qualifier: String,
    pub date_time: String,
}

pub fn get_dtm(dtm_content: String) -> DTM {
    let dtm_parts: Vec<&str> = dtm_content.split("*").collect();
    DTM {
        date_time_qualifier: get_element(&dtm_parts, 0),
        date_time: get_element(&dtm_parts, 1),
    }
}

pub fn write_dtm(dtm: DTM) -> String {
    if dtm.date_time_qualifier.is_empty() {
        return String::new();
    }
    let mut dtm_content = String::new();
    dtm_content.push_str("DTM*");
    dtm_content.push_str(&dtm.date_time_qualifier);
    dtm_content.push_str("*");
    dtm_content.push_str(&dtm.date_time);
    dtm_content.push_str("~");
    dtm_content
}
