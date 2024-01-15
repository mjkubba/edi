use std::fs::File;
use std::io::Read;


fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // 
    let nanme_index = contents.find("NM1").unwrap();
    let name_start = &contents[nanme_index..];
    let name_end = name_start.find("~").unwrap();
    let name_content = &name_start[4..name_end];
        
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
    
    println!("Entity ID: {}", nm101_entity_id);
    println!("Entity Type: {}", nm102_entity_type);
    println!("Last Name: {}", nm103_lastname);
    println!("First Name: {}", nm104_firstname);
    println!("Middle Initial: {}", nm105_middle_initial);
    println!("Suffix: {}", nm106_suffix);
    println!("Title: {}", nm107_title);
    println!("ID Code: {}", nm108_id_code);
    println!("Member Number: {}", nm109_member_number);




    

}