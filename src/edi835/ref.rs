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
