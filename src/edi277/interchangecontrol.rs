use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeHeader {
    pub isa01_authorization_qualifier: String,
    pub isa02_authorization_information: String,
    pub isa03_security_qualifier: String,
    pub isa04_security_information: String,
    pub isa05_interchange_id_qualifier: String,
    pub isa06_interchange_sender_id: String,
    pub isa07_interchange_id_qualifier: String,
    pub isa08_interchange_receiver_id: String,
    pub isa09_interchange_date: String,
    pub isa10_interchange_time: String,
    pub isa11_repetition_separator: String,
    pub isa12_interchange_control_version_number: String,
    pub isa13_interchange_control_number: String,
    pub isa14_acknowledgment_requested: String,
    pub isa15_usage_indicator: String,
    pub isa16_component_element_separator: String,
}

pub fn get_interchange_header(contents: String) -> (InterchangeHeader, String) {
    let mut interchange_header = InterchangeHeader::default();
    let mut remaining_content = contents.clone();

    if contents.starts_with("ISA") {
        let isa_segment = contents.split('~').next().unwrap_or("");
        let isa_elements: Vec<&str> = isa_segment.split('*').collect();

        if isa_elements.len() >= 17 {
            interchange_header.isa01_authorization_qualifier = isa_elements[1].to_string();
            interchange_header.isa02_authorization_information = isa_elements[2].to_string();
            interchange_header.isa03_security_qualifier = isa_elements[3].to_string();
            interchange_header.isa04_security_information = isa_elements[4].to_string();
            interchange_header.isa05_interchange_id_qualifier = isa_elements[5].to_string();
            interchange_header.isa06_interchange_sender_id = isa_elements[6].to_string();
            interchange_header.isa07_interchange_id_qualifier = isa_elements[7].to_string();
            interchange_header.isa08_interchange_receiver_id = isa_elements[8].to_string();
            interchange_header.isa09_interchange_date = isa_elements[9].to_string();
            interchange_header.isa10_interchange_time = isa_elements[10].to_string();
            interchange_header.isa11_repetition_separator = isa_elements[11].to_string();
            interchange_header.isa12_interchange_control_version_number = isa_elements[12].to_string();
            interchange_header.isa13_interchange_control_number = isa_elements[13].to_string();
            interchange_header.isa14_acknowledgment_requested = isa_elements[14].to_string();
            interchange_header.isa15_usage_indicator = isa_elements[15].to_string();
            interchange_header.isa16_component_element_separator = isa_elements[16].to_string();
        }

        // Remove the ISA segment from the remaining content
        remaining_content = contents.replacen(isa_segment, "", 1);
        if remaining_content.starts_with("~") {
            remaining_content = remaining_content[1..].to_string();
        }
    }

    (interchange_header, remaining_content)
}

pub fn write_interchange_control(interchange_header: &InterchangeHeader) -> String {
    format!(
        "ISA*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}*{}~",
        interchange_header.isa01_authorization_qualifier,
        interchange_header.isa02_authorization_information,
        interchange_header.isa03_security_qualifier,
        interchange_header.isa04_security_information,
        interchange_header.isa05_interchange_id_qualifier,
        interchange_header.isa06_interchange_sender_id,
        interchange_header.isa07_interchange_id_qualifier,
        interchange_header.isa08_interchange_receiver_id,
        interchange_header.isa09_interchange_date,
        interchange_header.isa10_interchange_time,
        interchange_header.isa11_repetition_separator,
        interchange_header.isa12_interchange_control_version_number,
        interchange_header.isa13_interchange_control_number,
        interchange_header.isa14_acknowledgment_requested,
        interchange_header.isa15_usage_indicator,
        interchange_header.isa16_component_element_separator
    )
}
