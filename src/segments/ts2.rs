// EDI 835 TS2 - PROVIDER SUMMARY INFORMATION
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct TS2 {
    ts201_total_drg_amount: String,
    ts202_total_fed_spec_amount: String,
    ts203_total_hosp_spec_amount: String,
    ts204_total_disproportionate_share_amount: String,
    ts205_total_capital_amount: String,
    ts206_total_indirect_medical_education_amount: String,
    ts207_total_outlier_day_count: String,
    ts208_total_day_outlier_amount: String,
    ts209_total_cost_outlier_amount: String,
    ts210_average_drg_length_of_stay: String,
    ts211_total_discharge_count: String,
    ts212_total_cost_report_day_count: String,
    ts213_total_covered_day_count: String,
    ts214_total_noncovered_day_count: String,
    ts215_total_msp_pass_through_amount: String,
    ts216_average_drg_weight: String,
    ts217_total_pps_capital_fs_drg_amount: String,
    ts218_total_pps_capital_hs_drg_amount: String,
    ts219_total_pps_dsh_drg_amount: String,
}

pub fn get_ts2(ts2_content: String) -> TS2 {
    let ts2_parts: Vec<&str> = ts2_content.split("*").collect();
    let mut ts201_total_drg_amount: String ="".to_string();
    let mut ts202_total_fed_spec_amount: String ="".to_string();
    let mut ts203_total_hosp_spec_amount: String ="".to_string();
    let mut ts204_total_disproportionate_share_amount: String ="".to_string();
    let mut ts205_total_capital_amount: String ="".to_string();
    let mut ts206_total_indirect_medical_education_amount: String ="".to_string();
    let mut ts207_total_outlier_day_count: String ="".to_string();
    let mut ts208_total_day_outlier_amount: String ="".to_string();
    let mut ts209_total_cost_outlier_amount: String ="".to_string();
    let mut ts210_average_drg_length_of_stay: String ="".to_string();
    let mut ts211_total_discharge_count: String ="".to_string();
    let mut ts212_total_cost_report_day_count: String ="".to_string();
    let mut ts213_total_covered_day_count: String ="".to_string();
    let mut ts214_total_noncovered_day_count: String ="".to_string();
    let mut ts215_total_msp_pass_through_amount: String ="".to_string();
    let mut ts216_average_drg_weight: String ="".to_string();
    let mut ts217_total_pps_capital_fs_drg_amount: String ="".to_string();
    let mut ts218_total_pps_capital_hs_drg_amount: String ="".to_string();
    let mut ts219_total_pps_dsh_drg_amount: String ="".to_string();

    if ts2_parts.get(0).is_some() {
        ts201_total_drg_amount = ts2_parts[0].to_string();
    }
    if ts2_parts.get(1).is_some() {
        ts202_total_fed_spec_amount = ts2_parts[1].to_string();
    }
    if ts2_parts.get(2).is_some() {
        ts203_total_hosp_spec_amount = ts2_parts[2].to_string();
    }
    if ts2_parts.get(3).is_some() {
        ts204_total_disproportionate_share_amount = ts2_parts[3].to_string();
    }
    if ts2_parts.get(4).is_some() {
        ts205_total_capital_amount = ts2_parts[4].to_string();
    }
    if ts2_parts.get(5).is_some() {
        ts206_total_indirect_medical_education_amount = ts2_parts[5].to_string();
    }
    if ts2_parts.get(6).is_some() {
        ts207_total_outlier_day_count = ts2_parts[6].to_string();
    }
    if ts2_parts.get(7).is_some() {
        ts208_total_day_outlier_amount = ts2_parts[7].to_string();
    }
    if ts2_parts.get(8).is_some() {
        ts209_total_cost_outlier_amount = ts2_parts[8].to_string();
    }
    if ts2_parts.get(9).is_some() {
        ts210_average_drg_length_of_stay = ts2_parts[9].to_string();
    }
    if ts2_parts.get(10).is_some() {
        ts211_total_discharge_count = ts2_parts[10].to_string();
    }
    if ts2_parts.get(11).is_some() {
        ts212_total_cost_report_day_count = ts2_parts[11].to_string();
    }
    if ts2_parts.get(12).is_some() {
        ts213_total_covered_day_count = ts2_parts[12].to_string();
    }
    if ts2_parts.get(13).is_some() {
        ts214_total_noncovered_day_count = ts2_parts[13].to_string();
    }
    if ts2_parts.get(14).is_some() {
        ts215_total_msp_pass_through_amount = ts2_parts[14].to_string();
    }
    if ts2_parts.get(15).is_some() {
        ts216_average_drg_weight = ts2_parts[15].to_string();
    }
    if ts2_parts.get(16).is_some() {
        ts217_total_pps_capital_fs_drg_amount = ts2_parts[16].to_string();
    }
    if ts2_parts.get(17).is_some() {
        ts218_total_pps_capital_hs_drg_amount = ts2_parts[17].to_string();
    }
    if ts2_parts.get(18).is_some() {
        ts219_total_pps_dsh_drg_amount = ts2_parts[18].to_string();
    }
    TS2 {
        ts201_total_drg_amount,
        ts202_total_fed_spec_amount,
        ts203_total_hosp_spec_amount,
        ts204_total_disproportionate_share_amount,
        ts205_total_capital_amount,
        ts206_total_indirect_medical_education_amount,
        ts207_total_outlier_day_count,
        ts208_total_day_outlier_amount,
        ts209_total_cost_outlier_amount,
        ts210_average_drg_length_of_stay,
        ts211_total_discharge_count,
        ts212_total_cost_report_day_count,
        ts213_total_covered_day_count,
        ts214_total_noncovered_day_count,
        ts215_total_msp_pass_through_amount,
        ts216_average_drg_weight,
        ts217_total_pps_capital_fs_drg_amount,
        ts218_total_pps_capital_hs_drg_amount,
        ts219_total_pps_dsh_drg_amount,
    }

}