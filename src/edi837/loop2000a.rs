use serde::{Serialize, Deserialize};

/// HL - Hierarchical Level for Billing/Pay-to Provider
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HL {
    pub segment_id: String,
    pub hierarchical_id_number: String,
    pub hierarchical_parent_id_number: Option<String>,
    pub hierarchical_level_code: String,
    pub hierarchical_child_code: String,
}

/// NM1 - Billing Provider Name
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NM1 {
    pub segment_id: String,
    pub entity_identifier_code: String,
    pub entity_type_qualifier: String,
    pub name_last_or_organization_name: String,
    pub name_first: Option<String>,
    pub name_middle: Option<String>,
    pub name_prefix: Option<String>,
    pub name_suffix: Option<String>,
    pub identification_code_qualifier: Option<String>,
    pub identification_code: Option<String>,
    pub entity_relationship_code: Option<String>,
    pub entity_identifier_code_2: Option<String>,
}

/// PRV - Billing Provider Specialty Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PRV {
    pub segment_id: String,
    pub provider_code: String,
    pub reference_identification_qualifier: String,
    pub reference_identification: String,
    pub state_or_province_code: Option<String>,
    pub provider_specialty_information: Option<String>,
    pub provider_organization_code: Option<String>,
}

/// Loop2000a structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000a {
    pub hl: HL,
    pub prv: Option<PRV>,
    pub loop2010aa: Vec<Loop2010aa>,
    pub loop2000b: Vec<Loop2000b>,
}

/// Loop2010aa structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010aa {
    pub nm1: NM1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub ref_: Vec<REF>,
    pub per: Vec<PER>,
}

/// Loop2000b structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000b {
    pub hl: HL,
    pub sbr: SBR,
    pub loop2010ba: Loop2010ba,
    pub loop2010bb: Loop2010bb,
    pub loop2000c: Vec<Loop2000c>,
}

/// N3 - Billing Provider Address
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct N3 {
    pub segment_id: String,
    pub address_information: String,
    pub address_information_2: Option<String>,
}

/// N4 - Billing Provider City, State, ZIP Code
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct N4 {
    pub segment_id: String,
    pub city_name: String,
    pub state_or_province_code: String,
    pub postal_code: String,
    pub country_code: Option<String>,
    pub location_qualifier: Option<String>,
    pub location_identifier: Option<String>,
    pub country_subdivision_code: Option<String>,
}

/// REF - Billing Provider Additional Identification
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct REF {
    pub segment_id: String,
    pub reference_identification_qualifier: String,
    pub reference_identification: String,
    pub description: Option<String>,
    pub reference_identifier: Option<String>,
}

/// PER - Billing Provider Contact Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PER {
    pub segment_id: String,
    pub contact_function_code: String,
    pub name: String,
    pub communication_number_qualifier: Option<String>,
    pub communication_number: Option<String>,
    pub communication_number_qualifier_2: Option<String>,
    pub communication_number_2: Option<String>,
    pub communication_number_qualifier_3: Option<String>,
    pub communication_number_3: Option<String>,
    pub contact_inquiry_reference: Option<String>,
}

/// SBR - Subscriber Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SBR {
    pub segment_id: String,
    pub payer_responsibility_sequence_number_code: String,
    pub individual_relationship_code: Option<String>,
    pub reference_identification: Option<String>,
    pub name: Option<String>,
    pub insurance_type_code: Option<String>,
    pub coordination_of_benefits_code: Option<String>,
    pub yes_no_condition_or_response_code: Option<String>,
    pub employment_status_code: Option<String>,
    pub claim_filing_indicator_code: Option<String>,
}

/// Loop2010ba structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ba {
    pub nm1: NM1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub ref_: Vec<REF>,
}

/// Loop2010bb structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010bb {
    pub nm1: NM1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub ref_: Vec<REF>,
}

/// Loop2000c structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000c {
    pub hl: HL,
    pub pat: PAT,
    pub loop2010ca: Loop2010ca,
    pub loop2300: Vec<Loop2300>,
}

/// DMG - Subscriber Demographic Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DMG {
    pub segment_id: String,
    pub date_time_period_format_qualifier: String,
    pub date_time_period: String,
    pub gender_code: Option<String>,
    pub marital_status_code: Option<String>,
    pub race_or_ethnicity_code: Option<String>,
    pub citizenship_status_code: Option<String>,
    pub country_code: Option<String>,
    pub basis_of_verification_code: Option<String>,
}

/// PAT - Patient Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PAT {
    pub segment_id: String,
    pub individual_relationship_code: String,
    pub patient_location_code: Option<String>,
    pub employment_status_code: Option<String>,
    pub student_status_code: Option<String>,
    pub date_time_period_format_qualifier: Option<String>,
    pub date_time_period: Option<String>,
    pub unit_or_basis_for_measurement_code: Option<String>,
    pub weight: Option<String>,
    pub pregnancy_indicator: Option<String>,
}

/// Loop2010ca structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2010ca {
    pub nm1: NM1,
    pub n3: Option<N3>,
    pub n4: Option<N4>,
    pub dmg: Option<DMG>,
    pub ref_: Vec<REF>,
}

/// Loop2300 structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    pub clm: CLM,
    pub dtp: Vec<DTP>,
    pub ref_: Vec<REF>,
    pub hi: Vec<HI>,
    pub loop2400: Vec<Loop2400>,
}

/// CLM - Claim Information
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CLM {
    pub segment_id: String,
    pub claim_submitter_identifier: String,
    pub monetary_amount: String,
    pub claim_filing_indicator_code: Option<String>,
    pub non_institutional_claim_type_code: Option<String>,
    pub health_care_service_location_information: Option<String>,
    pub yes_no_condition_or_response_code: Option<String>,
    pub provider_accept_assignment_code: Option<String>,
    pub yes_no_condition_or_response_code_2: Option<String>,
    pub release_of_information_code: Option<String>,
    pub patient_signature_source_code: Option<String>,
}

/// DTP - Date or Time Period
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTP {
    pub segment_id: String,
    pub date_time_qualifier: String,
    pub date_time_period_format_qualifier: String,
    pub date_time_period: String,
}

/// HI - Health Care Information Codes
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HI {
    pub segment_id: String,
    pub health_care_code_information: String,
    pub health_care_code_information_2: Option<String>,
    pub health_care_code_information_3: Option<String>,
    pub health_care_code_information_4: Option<String>,
    pub health_care_code_information_5: Option<String>,
    pub health_care_code_information_6: Option<String>,
    pub health_care_code_information_7: Option<String>,
    pub health_care_code_information_8: Option<String>,
    pub health_care_code_information_9: Option<String>,
    pub health_care_code_information_10: Option<String>,
    pub health_care_code_information_11: Option<String>,
    pub health_care_code_information_12: Option<String>,
}

/// Loop2400 structure for EDI837
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2400 {
    pub lx: LX,
    pub sv1: Option<SV1>,
    pub sv2: Option<SV2>,
    pub sv3: Option<SV3>,
    pub dtp: Vec<DTP>,
    pub ref_: Vec<REF>,
}

/// LX - Service Line Number
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LX {
    pub segment_id: String,
    pub assigned_number: String,
}

/// SV1 - Professional Service
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SV1 {
    pub segment_id: String,
    pub composite_medical_procedure_identifier: String,
    pub monetary_amount: String,
    pub unit_or_basis_for_measurement_code: Option<String>,
    pub quantity: Option<String>,
    pub facility_code_value: Option<String>,
    pub service_type_code: Option<String>,
    pub composite_diagnosis_code_pointer: Option<String>,
    pub yes_no_condition_or_response_code: Option<String>,
    pub multiple_procedure_code: Option<String>,
    pub yes_no_condition_or_response_code_2: Option<String>,
    pub yes_no_condition_or_response_code_3: Option<String>,
    pub review_code: Option<String>,
    pub national_or_local_assigned_review_value: Option<String>,
    pub copay_status_code: Option<String>,
    pub health_care_professional_shortage_area_code: Option<String>,
    pub reference_identification: Option<String>,
    pub postal_code: Option<String>,
    pub monetary_amount_2: Option<String>,
    pub level_of_care_code: Option<String>,
    pub provider_agreement_code: Option<String>,
}

/// SV2 - Institutional Service
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SV2 {
    pub segment_id: String,
    pub service_line_revenue_code: String,
    pub composite_medical_procedure_identifier: Option<String>,
    pub monetary_amount: String,
    pub unit_or_basis_for_measurement_code: Option<String>,
    pub quantity: Option<String>,
    pub unit_rate: Option<String>,
    pub monetary_amount_2: Option<String>,
    pub yes_no_condition_or_response_code: Option<String>,
    pub nurse_indicator: Option<String>,
    pub emergency_indicator: Option<String>,
    pub multiple_procedure_code: Option<String>,
    pub yes_no_condition_or_response_code_2: Option<String>,
    pub review_code: Option<String>,
    pub national_or_local_assigned_review_value: Option<String>,
    pub copay_status_code: Option<String>,
    pub service_type_code: Option<String>,
    pub monetary_amount_3: Option<String>,
    pub level_of_care_code: Option<String>,
    pub provider_agreement_code: Option<String>,
}

/// SV3 - Dental Service
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SV3 {
    pub segment_id: String,
    pub composite_medical_procedure_identifier: String,
    pub monetary_amount: String,
    pub facility_code_value: Option<String>,
    pub oral_cavity_designation: Option<String>,
    pub prosthesis_crown_or_inlay_code: Option<String>,
    pub quantity: Option<String>,
    pub description: Option<String>,
    pub copay_status_code: Option<String>,
    pub provider_agreement_code: Option<String>,
    pub yes_no_condition_or_response_code: Option<String>,
    pub composite_diagnosis_code_pointer: Option<String>,
}
