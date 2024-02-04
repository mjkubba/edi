use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct REF{
    pub reference_id_number_qualifier: String,
    pub reference_id_number: String,

}

pub fn get_ref(ref_content: String) -> REF {
    let ref_parts: Vec<&str> = ref_content.split("*").collect();
    REF {
        reference_id_number_qualifier: ref_parts[0].to_string(),
        reference_id_number: ref_parts[1].to_string(),
    }
}

pub fn write_ref(rref:REF) -> String {
    if rref.reference_id_number_qualifier.is_empty() {
        return String::new();
    }
    let mut ref_content = String::new();
    ref_content.push_str("REF*");
    ref_content.push_str(&rref.reference_id_number_qualifier);
    ref_content.push_str("*");
    ref_content.push_str(&rref.reference_id_number);
    ref_content.push_str("~");
    ref_content
}
    
