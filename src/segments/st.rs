use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct ST {
    pub transaction_set_id: String,
    pub transaction_set_control_number: String,
    pub implementation_conven_ref: String,
}

pub fn get_st(st_content: String) -> ST {
    let st_parts: Vec<&str> = st_content.split("*").collect();
    let mut implementation_conven_ref = String::new();
    if st_parts.get(2).is_some() {
        implementation_conven_ref = get_element(&st_parts, 2);
    }
    ST {
        transaction_set_id: get_element(&st_parts, 0),
        transaction_set_control_number: get_element(&st_parts, 1),
        implementation_conven_ref,
    }
}

pub fn write_st(st: ST) -> String {
    build_segment(&[
        "ST",
        &st.transaction_set_id,
        &st.transaction_set_control_number,
        &st.implementation_conven_ref,
    ])
}
