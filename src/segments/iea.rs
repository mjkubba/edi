use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
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


pub fn write_iea(iea:IEA) -> String {
    let mut iea_content = String::new();
    iea_content.push_str("IEA*");
    iea_content.push_str(&iea.number_of_included_group);
    iea_content.push_str("*");
    iea_content.push_str(&iea.interchange_control_number);
    iea_content.push_str("~");
    iea_content

}