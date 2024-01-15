use std::fs::File;
use std::io::Read;


fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // TODO: create structs for these segments
    // find how many gs segments are in the file
    let gs_count = contents.matches("GS").count();
    println!("Number of GS segments: {}", gs_count);
    
    // find the first occurrence of "GS" in the contents of the file and extract the content between "GS" and "~"
    let gs_index = contents.find("GS").unwrap();
    let gs_start = &contents[gs_index..];
    let gs_end = gs_start.find("~").unwrap();
    let gs_content = &gs_start[2..gs_end];

    // Split the gs content into parts and set new vars for each segment
    let gs_parts: Vec<&str> = gs_content.split("*").collect();
    let gs01_functional_id_code = gs_parts[0];
    let gs02_app_sender_id = gs_parts[1];
    let gs03_app_receiver_id = gs_parts[2];
    let gs04_date = gs_parts[3];
    let gs05_time = gs_parts[4];
    let gs06_group_control_number = gs_parts[5];
    let gs07_responsible_agency = gs_parts[6];
    let gs08_version_id = gs_parts[7];

    // Print the extracted content
    println!("First GS section");
    println!("Functional ID Code: {}", gs01_functional_id_code);
    println!("Application Sender ID: {}", gs02_app_sender_id);
    println!("Application Receiver ID: {}", gs03_app_receiver_id);
    println!("Date: {}", gs04_date);
    println!("Time: {}", gs05_time);
    println!("Group Control Number: {}", gs06_group_control_number);
    println!("Responsible Agency: {}", gs07_responsible_agency);
    println!("Version ID: {}", gs08_version_id);

    // find the first occurrence of "ST" segment
    let st_index = contents.find("ST").unwrap();
    let st_start = &contents[st_index..];
    let st_end = st_start.find("~").unwrap();
    let st_content = &st_start[3..st_end];

    // Print the ST segment
    println!("ST segment: {}", st_content);

    // Split the st content into parts and set new vars for each segment
    let st_parts: Vec<&str> = st_content.split("*").collect();
    let st01_transaction_set_id = st_parts[0];
    let st02_transaction_set_control_number = st_parts[1];

    // Print the extracted content
    println!("First ST section");
    println!("Transaction Set ID: {}", st01_transaction_set_id);
    println!("Transaction Set Control Number: {}", st02_transaction_set_control_number);

    // find the first occurrence of "BPR" segment
    let bpr_index = contents.find("BPR").unwrap();
    let bpr_start = &contents[bpr_index..];
    let bpr_end = bpr_start.find("~").unwrap();
    let bpr_content = &bpr_start[4..bpr_end];

    // Print the BPR segment
    println!("BPR segment: {}", bpr_content);

    // Split the bpr content into parts and set new vars for each segment
    let bpr_parts: Vec<&str> = bpr_content.split("*").collect();
    let bpr01_payer_id = bpr_parts[0];
    let bpr02_payer_name = bpr_parts[1];
    let bpr03_payer_address = bpr_parts[2];
    let bpr04_payer_city = bpr_parts[3];
    let bpr05_payer_state = bpr_parts[4];
    let bpr06_payer_zip = bpr_parts[5];

    // rint the extracted content
    println!("BPR segment");
    println!("Payer ID: {}", bpr01_payer_id);
    println!("Payer Name: {}", bpr02_payer_name);
    println!("Payer Address: {}", bpr03_payer_address);
    println!("Payer City: {}", bpr04_payer_city);
    println!("Payer State: {}", bpr05_payer_state);
    println!("Payer Zip: {}", bpr06_payer_zip);
        
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