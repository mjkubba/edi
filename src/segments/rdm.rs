use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RDM {
    pub rdm01_report_transmission_code: String,
    pub rdm02_name: String,
    pub rdm03_communication_number: String,
}

// both rdm02 and 03 are optional, 
// 2 is needed when rdm01 is BM
// 3 is needed when rdm01 is EM,FT or OL

pub fn get_rdm(rdm_content: String) -> RDM {
    let rdm_parts: Vec<&str> = rdm_content.split("*").collect();
    let mut rdm02_name: String ="".to_string();
    let mut rdm03_communication_number: String ="".to_string();

    if rdm_parts.get(1).is_some() {
        rdm02_name = rdm_parts[1].to_string();
    }
    if rdm_parts.get(2).is_some() {
        rdm03_communication_number = rdm_parts[2].to_string();
    }
    RDM {
        rdm01_report_transmission_code: rdm_parts[0].to_string(),
        rdm02_name,
        rdm03_communication_number,
    }
}

pub fn write_rdm(rdm:RDM) -> String {
    if rdm.rdm01_report_transmission_code.is_empty() {
        return String::new();
    }
    let mut rdm_content: String = String::new();
    rdm_content.push_str("RDM*");
    rdm_content.push_str(&rdm.rdm01_report_transmission_code);
    
    // Add name if present or if communication number is present
    if !rdm.rdm02_name.is_empty() {
        rdm_content.push_str("*");
        rdm_content.push_str(&rdm.rdm02_name);
    } else if !rdm.rdm03_communication_number.is_empty() {
        rdm_content.push_str("*");
    }
    
    // Add communication number if present
    if !rdm.rdm03_communication_number.is_empty() {
        rdm_content.push_str("*");
        rdm_content.push_str(&rdm.rdm03_communication_number);
    }
    
    rdm_content.push_str("~");
    rdm_content
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_rdm() {
        let rdm_content = "BM*RDM02*RDM03".to_string();
        let rdm = get_rdm(rdm_content);
        assert_eq!(rdm.rdm01_report_transmission_code, "BM");
        assert_eq!(rdm.rdm02_name, "RDM02");
        assert_eq!(rdm.rdm03_communication_number, "RDM03");
    }
}
