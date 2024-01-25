#[derive(Debug, Default,PartialEq,Clone)]
#[allow(dead_code)]
pub struct TRN {
    pub trace_type_code: String,
    pub reference_id: String,
    pub originating_company_id: String,
    pub trn04_reference_id: String,
}

pub fn get_trn(trn_content: String) -> TRN {
    let trn_parts: Vec<&str> = trn_content.split("*").collect();
    let mut trn04_reference_id: String ="".to_string();
    if trn_parts.get(3).is_some()  {
        trn04_reference_id = trn_parts[3].to_string();
    }
    TRN {
        trace_type_code: trn_parts[0].to_string(),
        reference_id: trn_parts[1].to_string(),
        originating_company_id: trn_parts[2].to_string(),
        trn04_reference_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_trn() {
        let trn_content = "TRN*123456789*ABC*123456789".to_string();
        let trn = get_trn(trn_content);
        assert_eq!(trn.trace_type_code, "TRN");
        assert_eq!(trn.reference_id, "123456789");
        assert_eq!(trn.originating_company_id, "ABC");
        assert_eq!(trn.trn04_reference_id, "123456789");
    }
}