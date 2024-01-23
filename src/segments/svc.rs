#[derive(Debug, Default,PartialEq)]
#[allow(dead_code)]
pub struct SVC {
    pub svc01_1_product_or_service_is_qualifier: String,
    pub svc02_line_item_charge_amount: String,
    pub svc03_line_item_provider_payment_amount: String,
    pub scv04_product_service_id: String,
    pub svc05_unit_of_service_paid_count: String,
    pub svc06_composite_medical_procedure_id: String,
    pub svc07_original_units_of_service_count: String,



}

pub fn get_svc(svc_content: String) -> SVC {
    let svc_parts: Vec<&str> = svc_content.split("*").collect();
    let mut scv04_product_service_id: String ="".to_string();
    let mut svc05_unit_of_service_paid_count: String ="".to_string();
    let mut svc06_composite_medical_procedure_id: String ="".to_string();
    let mut svc07_original_units_of_service_count: String ="".to_string();
    
    
    if svc_parts.get(3).is_some()  {
        scv04_product_service_id = svc_parts[3].to_string();
    }
    if svc_parts.get(4).is_some()  {
        svc05_unit_of_service_paid_count = svc_parts[4].to_string();
    }
    if svc_parts.get(5).is_some()  {
        svc06_composite_medical_procedure_id = svc_parts[5].to_string();
    }
    if svc_parts.get(6).is_some()  {
        svc07_original_units_of_service_count = svc_parts[6].to_string();
    }

    SVC {
        svc01_1_product_or_service_is_qualifier: svc_parts[0].to_string(),
        svc02_line_item_charge_amount: svc_parts[1].to_string(),
        svc03_line_item_provider_payment_amount: svc_parts[2].to_string(),
        scv04_product_service_id,
        svc05_unit_of_service_paid_count,
        svc06_composite_medical_procedure_id,
        svc07_original_units_of_service_count,
    }
}