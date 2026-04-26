use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterchangeTrailer {
    pub ge01_number_of_transaction_sets: String,
    pub ge02_group_control_number: String,
    pub iea01_number_of_included_functional_groups: String,
    pub iea02_interchange_control_number: String,
}

pub fn get_interchange_trailer(contents: String) -> (InterchangeTrailer, String) {
    let mut interchange_trailer = InterchangeTrailer::default();
    let mut remaining_content = contents.clone();

    // Find the GE segment
    if let Some(ge_segment_start) = remaining_content.find("GE") {
        let ge_segment_end = remaining_content[ge_segment_start..]
            .find('~')
            .unwrap_or(remaining_content.len() - ge_segment_start);
        let ge_segment = &remaining_content[ge_segment_start..ge_segment_start + ge_segment_end];

        let ge_elements: Vec<&str> = ge_segment.split('*').collect();

        if ge_elements.len() >= 3 {
            interchange_trailer.ge01_number_of_transaction_sets = ge_elements[1].to_string();
            interchange_trailer.ge02_group_control_number = ge_elements[2].to_string();
        }

        // Remove the GE segment from the remaining content
        remaining_content = remaining_content[..ge_segment_start].to_string()
            + &remaining_content[ge_segment_start + ge_segment_end + 1..];
    }

    // Find the IEA segment
    if let Some(iea_segment_start) = remaining_content.find("IEA") {
        let iea_segment_end = remaining_content[iea_segment_start..]
            .find('~')
            .unwrap_or(remaining_content.len() - iea_segment_start);
        let iea_segment =
            &remaining_content[iea_segment_start..iea_segment_start + iea_segment_end];

        let iea_elements: Vec<&str> = iea_segment.split('*').collect();

        if iea_elements.len() >= 3 {
            interchange_trailer.iea01_number_of_included_functional_groups =
                iea_elements[1].to_string();
            interchange_trailer.iea02_interchange_control_number = iea_elements[2].to_string();
        }

        // Remove the IEA segment from the remaining content
        remaining_content = remaining_content[..iea_segment_start].to_string();
    }

    (interchange_trailer, remaining_content)
}

pub fn write_interchange_trailer(interchange_trailer: &InterchangeTrailer) -> String {
    format!(
        "GE*{}*{}~\nIEA*{}*{}~",
        interchange_trailer.ge01_number_of_transaction_sets,
        interchange_trailer.ge02_group_control_number,
        interchange_trailer.iea01_number_of_included_functional_groups,
        interchange_trailer.iea02_interchange_control_number
    )
}
