#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]
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
        functional_id_code: gs_parts[0].to_string(),
        app_sender_id: gs_parts[1].to_string(),
        app_receiver_id: gs_parts[2].to_string(),
        date: gs_parts[3].to_string(),
        time: gs_parts[4].to_string(),
        group_control_number: gs_parts[5].to_string(),
        responsible_agency: gs_parts[6].to_string(),
        version_number: gs_parts[7].to_string(),
    }
}