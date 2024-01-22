#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ISA {
    information_qualifier: String,
    authorization_information: String,
    security_information_qualifier: String,
    security_information: String,
    sender_id_qualifier: String,
    sender_id: String,
    receiver_id_qualifier: String,
    receiver_id: String,
    date: String,
    time: String,
    control_number_identifier: String,
    control_version_number: String,
    control_number: String,
    ack_indicator: String,
    usage_indicator: String,
    component_element_separator: String,
}

// function to get the ISA struct
pub fn get_isa(isa_content: String) -> ISA {
    let isa_parts: Vec<&str> = isa_content.split("*").collect();
    ISA {
        information_qualifier: isa_parts[0].to_string(),
        authorization_information: isa_parts[1].to_string(),
        security_information_qualifier: isa_parts[2].to_string(),
        security_information: isa_parts[3].to_string(),
        sender_id_qualifier: isa_parts[4].to_string(),
        sender_id: isa_parts[5].to_string(),
        receiver_id_qualifier: isa_parts[6].to_string(),
        receiver_id: isa_parts[7].to_string(),
        date: isa_parts[8].to_string(),
        time: isa_parts[9].to_string(),
        control_number_identifier: isa_parts[10].to_string(),
        control_version_number: isa_parts[11].to_string(),
        control_number: isa_parts[12].to_string(),
        ack_indicator: isa_parts[13].to_string(),
        usage_indicator: isa_parts[14].to_string(),
        component_element_separator: isa_parts[15].to_string(),
    }
}

// impl Default for ISA {
//     fn default () -> ISA {
//         ISA{
//             information_qualifier: String::from(""),
//             authorization_information: String::from(""),
//             security_information_qualifier: String::from(""),
//             security_information: String::from(""),
//             sender_id_qualifier: String::from(""),
//             sender_id: String::from(""),
//             receiver_id_qualifier: String::from(""),
//             receiver_id: String::from(""),
//             date: String::from(""),
//             time: String::from(""),
//             control_number_identifier: String::from(""),
//             control_version_number: String::from(""),
//             control_number: String::from(""),
//             ack_indicator: String::from(""),
//             usage_indicator: String::from(""),
//             component_element_separator: String::from(""),
//         }
//     }
// }