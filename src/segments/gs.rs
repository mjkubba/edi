use crate::helper::edihelper::get_element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct GS {
    pub functional_id_code: String,
    pub app_sender_id: String,
    pub app_receiver_id: String,
    pub date: String,
    pub time: String,
    pub group_control_number: String,
    pub responsible_agency: String,
    pub version_number: String,
}

pub fn get_gs(gs_content: String) -> GS {
    let gs_parts: Vec<&str> = gs_content.split("*").collect();
    GS {
        functional_id_code: get_element(&gs_parts, 0),
        app_sender_id: get_element(&gs_parts, 1),
        app_receiver_id: get_element(&gs_parts, 2),
        date: get_element(&gs_parts, 3),
        time: get_element(&gs_parts, 4),
        group_control_number: get_element(&gs_parts, 5),
        responsible_agency: get_element(&gs_parts, 6),
        version_number: get_element(&gs_parts, 7),
    }
}

pub fn write_gs(gs: GS) -> String {
    let mut gs_content = String::new();
    gs_content.push_str("GS*");
    gs_content.push_str(&gs.functional_id_code);
    gs_content.push_str("*");
    gs_content.push_str(&gs.app_sender_id);
    gs_content.push_str("*");
    gs_content.push_str(&gs.app_receiver_id);
    gs_content.push_str("*");
    gs_content.push_str(&gs.date);
    gs_content.push_str("*");
    gs_content.push_str(&gs.time);
    gs_content.push_str("*");
    gs_content.push_str(&gs.group_control_number);
    gs_content.push_str("*");
    gs_content.push_str(&gs.responsible_agency);
    gs_content.push_str("*");
    gs_content.push_str(&gs.version_number);
    gs_content.push_str("~");
    gs_content
}
