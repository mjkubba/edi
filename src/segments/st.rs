use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ST {
    pub transaction_set_id: String,
    pub transaction_set_control_number: String,
}

pub fn get_st(st_content: String) -> ST {
    let st_parts: Vec<&str> = st_content.split("*").collect();
    ST {
        transaction_set_id: st_parts[0].to_string(),
        transaction_set_control_number: st_parts[1].to_string(),
    }
}

pub fn write_st(st: ST) -> String {
    let mut st_string = String::new();
    st_string.push_str("ST*");
    st_string.push_str(&st.transaction_set_id);
    st_string.push_str("*");
    st_string.push_str(&st.transaction_set_control_number);
    st_string.push_str("~");
    st_string
}