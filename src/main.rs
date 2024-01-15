use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug)]
struct ISA {
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
fn get_isa(isa_content: &str) -> ISA {
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

#[derive(Debug)]
#[allow(dead_code)]
struct GS {
    functional_id_code: String,
    app_sender_id: String,
    app_receiver_id: String,
    date: String,
    time: String,
    group_control_number: String,
    responsible_agency: String,
    version_number: String,
}

fn get_gs(gs_content: &str) -> GS {
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

#[derive(Debug)]
#[allow(dead_code)]
struct ST {
    transaction_set_id: String,
    transaction_set_control_number: String,
}

fn get_st(st_content: &str) -> ST {
    let st_parts: Vec<&str> = st_content.split("*").collect();
    ST {
        transaction_set_id: st_parts[0].to_string(),
        transaction_set_control_number: st_parts[1].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct BPR {
    payer_id: String,
    payer_name: String,
    payer_address: String,
    payer_city: String,
    payer_state: String,
    payer_zip: String,
}

fn get_bpr(bpr_content: &str) -> BPR {
    let bpr_parts: Vec<&str> = bpr_content.split("*").collect();
    BPR {
        payer_id: bpr_parts[0].to_string(),
        payer_name: bpr_parts[1].to_string(),
        payer_address: bpr_parts[2].to_string(),
        payer_city: bpr_parts[3].to_string(),
        payer_state: bpr_parts[4].to_string(),
        payer_zip: bpr_parts[5].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct TRN {
    trace_type_code: String,
    reference_id: String,
    originating_company_id: String,
}

fn get_trn(trn_content: &str) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    TRN {
        trace_type_code: trn_parts[0].to_string(),
        reference_id: trn_parts[1].to_string(),
        originating_company_id: trn_parts[2].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct REF{
    receiver_id_number: String,
    receiver_reference_id: String,

}

fn get_ref(ref_content: &str) -> REF {
    let ref_parts: Vec<&str> = ref_content.split("*").collect();
    REF {
        receiver_id_number: ref_parts[0].to_string(),
        receiver_reference_id: ref_parts[1].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct NM1{
    entity_id: String,
    entity_type: String,
    lastname: String,
    firstname: String,
    middle_initial: String,
    suffix: String,
    title: String,
    id_code: String,
    member_number: String,
}

fn get_nm1(nm1_content: &str) -> NM1 {
    let nm1_parts: Vec<&str> = nm1_content.split("*").collect();
    NM1 {
        entity_id: nm1_parts[0].to_string(),
        entity_type: nm1_parts[1].to_string(),
        lastname: nm1_parts[2].to_string(),
        firstname: nm1_parts[3].to_string(),
        middle_initial: nm1_parts[4].to_string(),
        suffix: nm1_parts[5].to_string(),
        title: nm1_parts[6].to_string(),
        id_code: nm1_parts[7].to_string(),
        member_number: nm1_parts[8].to_string(),
    }
}

#[allow(unused_variables)]
fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // TODO: create structs for these segments
    // find the first occurrence of "ISA" in the contents of the file and extract the content between "ISA" and "~"
    if contents.contains("ISA") {
        println!("ISA segment found");
        let isa_index = contents.find("ISA").unwrap();
        let isa_start = &contents[isa_index..];
        let isa_end = isa_start.find("~").unwrap();
        let isa_content = &isa_start[4..isa_end];
        let isa_segments = get_isa(isa_content);
        println!("{:?}", isa_segments);
        println!("ISA segment parsed");
        println!("\n");
    }
    
    // find how many gs segments are in the file
    if contents.contains("GS") {
        println!("GS segment found");
        // find the first occurrence of "GS" in the contents of the file and extract the content between "GS" and "~"
        let gs_index = contents.find("GS").unwrap();
        let gs_start = &contents[gs_index..];
        let gs_end = gs_start.find("~").unwrap();
        let gs_content = &gs_start[3..gs_end];
        let gs_segments = get_gs(gs_content);
        println!("{:?}", gs_segments);
        println!("GS segment parsed");
        println!("\n");
    }
    
    if contents.contains("ST") {
        println!("ST segment found");
        // find the first occurrence of "ST" in the contents of the file and extract the content between "ST" and "~"
        let st_index = contents.find("ST").unwrap();
        let st_start = &contents[st_index..];
        let st_end = st_start.find("~").unwrap();
        let st_content = &st_start[3..st_end];
        let st_segments = get_st(st_content);
        println!("{:?}", st_segments);
        println!("ST segment parsed");
        println!("\n");
    }

    if contents.contains("BPR") {
        println!("BPR segment found");
        // find the first occurrence of "BPR" in the contents of the file and extract the content between "BPR" and "~"
        let bpr_index = contents.find("BPR").unwrap();
        let bpr_start = &contents[bpr_index..];
        let bpr_end = bpr_start.find("~").unwrap();
        let bpr_content = &bpr_start[4..bpr_end];
        let bpr_segments = get_bpr(bpr_content);
        println!("{:?}", bpr_segments);
        println!("BPR segment parsed");
        println!("\n");
    }

    if contents.contains("TRN") {
        println!("TRN segment found");
        // find the first occurrence of "TRN" in the contents of the file and extract the content between "TRN" and "~"
        let trn_index = contents.find("TRN").unwrap();
        let trn_start = &contents[trn_index..];
        let trn_end = trn_start.find("~").unwrap();
        let trn_content = &trn_start[4..trn_end];
        let trn_segments = get_trn(trn_content);
        println!("{:?}", trn_segments);
        println!("TRN segment parsed");
        println!("\n");
    }

    if contents.contains("REF") {
        println!("REF segment found");
        // find the first occurrence of "REF" in the contents of the file and extract the content between "REF" and "~"
        let ref_index = contents.find("REF").unwrap();
        let ref_start = &contents[ref_index..];
        let ref_end = ref_start.find("~").unwrap();
        let ref_content = &ref_start[4..ref_end];
        let ref_segments = get_ref(ref_content);
        println!("{:?}", ref_segments);
        println!("REF segment parsed");
        println!("\n");
    }

    if contents.contains("NM1") {
        // find how many nm1 segments are in the file
        let nm1_count = contents.matches("NM1").count();
        println!("Number of NM1 segments: {}", nm1_count);

        let mut next_segment =  &contents[contents.find("NM1").unwrap()..];

        for n in 0..nm1_count {
            let nm1_start = next_segment;
            let nm1_end = nm1_start.find("~").unwrap();
            let nm1_content = &nm1_start[4..nm1_end];
            let nm1_segments = get_nm1(nm1_content);
            println!("{:?}", nm1_segments);
            println!("NM1 segment parsed");
            println!("\n");
            next_segment = &nm1_start[nm1_end+1..]
        }
    }
     
    
}