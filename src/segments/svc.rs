use crate::helper::edihelper::{build_segment, get_element};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]

pub struct SVC {
    pub svc01_1_product_or_service_is_qualifier: String,
    pub svc02_line_item_charge_amount: String,
    pub svc03_line_item_provider_payment_amount: String,
    pub svc04_product_service_id: String,
    pub svc05_unit_of_service_paid_count: String,
    pub svc06_composite_medical_procedure_id: String,
    pub svc07_original_units_of_service_count: String,
}

pub fn get_svc(svc_content: String) -> SVC {
    let svc_parts: Vec<&str> = svc_content.split("*").collect();
    let mut svc04_product_service_id: String = "".to_string();
    let mut svc05_unit_of_service_paid_count: String = "".to_string();
    let mut svc06_composite_medical_procedure_id: String = "".to_string();
    let mut svc07_original_units_of_service_count: String = "".to_string();

    if svc_parts.get(3).is_some() {
        svc04_product_service_id = get_element(&svc_parts, 3);
    }
    if svc_parts.get(4).is_some() {
        svc05_unit_of_service_paid_count = get_element(&svc_parts, 4);
    }
    if svc_parts.get(5).is_some() {
        svc06_composite_medical_procedure_id = get_element(&svc_parts, 5);
    }
    if svc_parts.get(6).is_some() {
        svc07_original_units_of_service_count = get_element(&svc_parts, 6);
    }

    SVC {
        svc01_1_product_or_service_is_qualifier: get_element(&svc_parts, 0),
        svc02_line_item_charge_amount: get_element(&svc_parts, 1),
        svc03_line_item_provider_payment_amount: get_element(&svc_parts, 2),
        svc04_product_service_id,
        svc05_unit_of_service_paid_count,
        svc06_composite_medical_procedure_id,
        svc07_original_units_of_service_count,
    }
}

pub fn write_svc(svc: SVC) -> String {
    if svc.svc01_1_product_or_service_is_qualifier.is_empty() {
        return String::new();
    }
    build_segment(&[
        "SVC",
        &svc.svc01_1_product_or_service_is_qualifier,
        &svc.svc02_line_item_charge_amount,
        &svc.svc03_line_item_provider_payment_amount,
        &svc.svc04_product_service_id,
        &svc.svc05_unit_of_service_paid_count,
        &svc.svc06_composite_medical_procedure_id,
        &svc.svc07_original_units_of_service_count,
    ])
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_svc() {
        let svc_content = "100*200*3*4*5*6*7";
        let svc = get_svc(svc_content.to_string());
        assert_eq!(svc.svc01_1_product_or_service_is_qualifier, "100");
        assert_eq!(svc.svc02_line_item_charge_amount, "200");
        assert_eq!(svc.svc03_line_item_provider_payment_amount, "3");
        assert_eq!(svc.svc04_product_service_id, "4");
        assert_eq!(svc.svc05_unit_of_service_paid_count, "5");
        assert_eq!(svc.svc06_composite_medical_procedure_id, "6");
        assert_eq!(svc.svc07_original_units_of_service_count, "7");
    }
}
