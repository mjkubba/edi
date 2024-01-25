#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]
pub struct IEA {
    pub number_of_included_group: String,
    pub interchange_control_number: String,
}

pub fn get_iea(iea_content: String) -> IEA {
    let iea_parts: Vec<&str> = iea_content.split("*").collect();
    IEA {
        number_of_included_group: iea_parts[0].to_string(),
        interchange_control_number: iea_parts[1].to_string(),
    }
}