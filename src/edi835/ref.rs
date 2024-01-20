#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct REF{
    reference_id_number_qualifier: String,
    reference_id_number: String,

}

pub fn get_ref(ref_content: String) -> REF {
    let ref_parts: Vec<&str> = ref_content.split("*").collect();
    REF {
        reference_id_number_qualifier: ref_parts[0].to_string(),
        reference_id_number: ref_parts[1].to_string(),
    }
}
