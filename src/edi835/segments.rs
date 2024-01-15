#[allow(dead_code)]
#[derive(Debug)]
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
pub fn get_isa(isa_content: &str) -> ISA {
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
pub struct GS {
    functional_id_code: String,
    app_sender_id: String,
    app_receiver_id: String,
    date: String,
    time: String,
    group_control_number: String,
    responsible_agency: String,
    version_number: String,
}

pub fn get_gs(gs_content: &str) -> GS {
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
pub struct ST {
    transaction_set_id: String,
    transaction_set_control_number: String,
}

pub fn get_st(st_content: &str) -> ST {
    let st_parts: Vec<&str> = st_content.split("*").collect();
    ST {
        transaction_set_id: st_parts[0].to_string(),
        transaction_set_control_number: st_parts[1].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BPR {
    payer_id: String,
    payer_name: String,
    payer_address: String,
    payer_city: String,
    payer_state: String,
    payer_zip: String,
}

pub fn get_bpr(bpr_content: &str) -> BPR {
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
pub struct TRN {
    trace_type_code: String,
    reference_id: String,
    originating_company_id: String,
}

pub fn get_trn(trn_content: &str) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    TRN {
        trace_type_code: trn_parts[0].to_string(),
        reference_id: trn_parts[1].to_string(),
        originating_company_id: trn_parts[2].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct REF{
    receiver_id_number: String,
    receiver_reference_id: String,

}

pub fn get_ref(ref_content: &str) -> REF {
    let ref_parts: Vec<&str> = ref_content.split("*").collect();
    REF {
        receiver_id_number: ref_parts[0].to_string(),
        receiver_reference_id: ref_parts[1].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct NM1{
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

pub fn get_nm1(nm1_content: &str) -> NM1 {
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct N1{
    payer_id_code: String,
    payee_name: String,
    // payee_identification_code_qualifier: String,
    // payee_identification_code: String,
}

pub fn get_n1(n1_content: &str) -> N1 {
    let n1_parts: Vec<&str> = n1_content.split("*").collect();
    N1 {
        payer_id_code: n1_parts[0].to_string(),
        payee_name: n1_parts[1].to_string(),
        // payee_identification_code_qualifier: n1_parts[2].to_string(),
        // payee_identification_code: n1_parts[3].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct N3{
    payee_address: String,
    payee_address2: String,
}

pub fn get_n3(n3_content: &str) -> N3 {
    let n3_parts: Vec<&str> = n3_content.split("*").collect();
    if n3_parts.len() == 1 {
        N3 {
            payee_address: n3_parts[0].to_string(),
            payee_address2: "".to_string(),
        }
    } else {
        N3 {
            payee_address: n3_parts[0].to_string(),
            payee_address2: n3_parts[1].to_string(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct N4{
    payee_city: String,
    payee_state: String,
    payee_zip: String,
}

pub fn get_n4(n4_content: &str) -> N4 {
    let n4_parts: Vec<&str> = n4_content.split("*").collect();
    N4 {
        payee_city: n4_parts[0].to_string(),
        payee_state: n4_parts[1].to_string(),
        payee_zip: n4_parts[2].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LX {
    claim_sequence_number: String,
}

pub fn get_lx(lx_content: &str) -> LX {
    let lx_parts: Vec<&str> = lx_content.split("*").collect();
    LX {
        claim_sequence_number: lx_parts[0].to_string(),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PER {
    contact_function_code: String,
    contact_name: String,
    contact_number_qualifier: String,
    contact_number: String,
}

pub fn get_per(per_content: &str) -> PER {
    let per_parts: Vec<&str> = per_content.split("*").collect();
    PER {
        contact_function_code: per_parts[0].to_string(),
        contact_name: per_parts[1].to_string(),
        contact_number_qualifier: per_parts[2].to_string(),
        contact_number: per_parts[3].to_string(),
    }
}
