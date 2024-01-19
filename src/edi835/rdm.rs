#[derive(Debug)]
#[allow(dead_code)]
pub struct RDM {
    rdm01_report_transmission_code: String,
    rdm02_name: String,
    rdm03_communication_number: String,
}

// both rdm02 and 03 are optional, 
// 2 is needed when rdm01 is BM
// 3 is needed when rdm01 is EM,FT or OL

pub fn get_rdm(rdm_content: &str) -> RDM {
    let rdm_parts: Vec<&str> = rdm_content.split("*").collect();
    let mut rdm02_name: String ="".to_string();
    let mut rdm03_communication_number: String ="".to_string();

    if rdm_parts.get(2).is_some() {
        rdm02_name = rdm_parts[2].to_string();
    }
    if rdm_parts.get(3).is_some() {
        rdm03_communication_number = rdm_parts[3].to_string();
    }
    RDM {
        rdm01_report_transmission_code: rdm_parts[0].to_string(),
        rdm02_name,
        rdm03_communication_number,
    }
}