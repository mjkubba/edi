use std::fs::File;
use std::io::Read;


fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // find how many nm1 segments are in the file
    let nm1_count = contents.matches("NM1").count();
    println!("Number of NM1 segments: {}", nm1_count);
    
    // find the first occurrence of "NM1" in the contents of the file and extract the content between "NM1" and "~"
    let name_index = contents.find("NM1").unwrap();
    let name_start = &contents[name_index..];
    let name_end = name_start.find("~").unwrap();
    let name_content = &name_start[4..name_end];

    // Split the name content into parts and set new vars for each segment
    let nm1_parts: Vec<&str> = name_content.split("*").collect();
    let nm101_entity_id = nm1_parts[0];
    let nm102_entity_type = nm1_parts[1];
    let nm103_lastname = nm1_parts[2];
    let nm104_firstname = nm1_parts[3];
    let nm105_middle_initial = nm1_parts[4];
    let nm106_suffix = nm1_parts[5];
    let nm107_title = nm1_parts[6];
    let nm108_id_code = nm1_parts[7];
    let nm109_member_number = nm1_parts[8];
    
    // Print the extracted content
    println!("First NM1 section");
    println!("Entity ID: {}", nm101_entity_id);
    println!("Entity Type: {}", nm102_entity_type);
    println!("Last Name: {}", nm103_lastname);
    println!("First Name: {}", nm104_firstname);
    println!("Middle Initial: {}", nm105_middle_initial);
    println!("Suffix: {}", nm106_suffix);
    println!("Title: {}", nm107_title);
    println!("ID Code: {}", nm108_id_code);
    println!("Member Number: {}", nm109_member_number);

    // find the second occurrence of "NM1" in the contents of the file and extract the content between "NM1" and "~"
    let other_nm1 = &name_start[name_end+1..];
    println!("{:?}", other_nm1);
    let provider_end = other_nm1.find("~").unwrap();
    let provider_content = &other_nm1[4..provider_end];
    println!("{:?}", provider_content);

    // Split the provider content into parts and set new vars for each segment
    let provider_parts: Vec<&str> = provider_content.split("*").collect();
    let nm101_provider_entity_id = provider_parts[0];
    let nm102_provider_entity_type = provider_parts[1];
    let nm103_provider_lastname = provider_parts[2];
    let nm104_provider_firstname = provider_parts[3];
    let nm105_provider_middle_initial = provider_parts[4];
    let nm106_provider_prefix = provider_parts[5];
    let nm107_provider_suffix = provider_parts[6];
    let nm108_provider_id_code = provider_parts[7];
    let nm109_provider_nat_identifier = provider_parts[8];

    // print the other nm1 content
    println!("Second NM1 section");
    println!("Entity ID: {}", nm101_provider_entity_id);
    println!("Entity Type: {}", nm102_provider_entity_type);
    println!("Last Name: {}", nm103_provider_lastname);
    println!("First Name: {}", nm104_provider_firstname);
    println!("Middle Initial: {}", nm105_provider_middle_initial);
    println!("Prefix: {}", nm106_provider_prefix);
    println!("Suffix: {}", nm107_provider_suffix);
    println!("ID Code: {}", nm108_provider_id_code);
    println!("National Identifier: {}", nm109_provider_nat_identifier);
    
}