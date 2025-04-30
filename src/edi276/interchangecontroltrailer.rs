use serde::{Serialize, Deserialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub iea01_number_of_included_functional_groups: String,
    pub iea02_interchange_control_number: String,
}

pub fn get_interchange_trailer(contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();
    let mut remaining_content = contents.clone();

    // Find the IEA segment
    if let Some(iea_segment_start) = contents.find("IEA") {
        let iea_segment_end = contents[iea_segment_start..].find('~').unwrap_or(contents.len() - iea_segment_start);
        let iea_segment = &contents[iea_segment_start..iea_segment_start + iea_segment_end];
        
        let iea_elements: Vec<&str> = iea_segment.split('*').collect();
        
        if iea_elements.len() >= 3 {
            interchange_trailer.iea01_number_of_included_functional_groups = iea_elements[1].to_string();
            interchange_trailer.iea02_interchange_control_number = iea_elements[2].to_string();
        }
        
        // Remove the IEA segment from the remaining content
        remaining_content = contents[..iea_segment_start].to_string();
    }
    
    (interchange_trailer, remaining_content)
}

pub fn write_interchange_trailer(interchange_trailer: &InterchangeTrailer) -> String {
    format!(
        "IEA*{}*{}~",
        interchange_trailer.iea01_number_of_included_functional_groups,
        interchange_trailer.iea02_interchange_control_number
    )
}
