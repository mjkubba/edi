use log::warn;

/// Validate X12 envelope control numbers and counts.
/// Logs warnings for mismatches but does not fail — allows processing
/// of structurally invalid files while alerting the caller.
///
/// Per X12 specs:
/// - ST02 must equal SE02 (§B.1.1.3.12.1)
/// - SE01 must equal segment count including ST and SE (§B.1.1.3.12.1)
/// - GS06 must equal GE02 (GE segment SEMANTIC note)
/// - ISA13 must equal IEA02 (IEA segment note)

/// Validate that ST02 and SE02 control numbers match.
pub fn validate_st_se(st02: &str, se02: &str) -> bool {
    if st02 != se02 {
        warn!(
            "Envelope error: ST02 ({}) != SE02 ({}). Per X12 spec, these must be identical.",
            st02, se02
        );
        return false;
    }
    true
}

/// Validate that SE01 equals the actual segment count (including ST and SE).
pub fn validate_segment_count(se01: &str, actual_count: usize) -> bool {
    if let Ok(expected) = se01.parse::<usize>() {
        if expected != actual_count {
            warn!(
                "Envelope error: SE01 claims {} segments but found {}.",
                expected, actual_count
            );
            return false;
        }
    }
    true
}

/// Validate that GS06 and GE02 control numbers match.
pub fn validate_gs_ge(gs06: &str, ge02: &str) -> bool {
    if gs06 != ge02 {
        warn!(
            "Envelope error: GS06 ({}) != GE02 ({}). Per X12 spec, these must be identical.",
            gs06, ge02
        );
        return false;
    }
    true
}

/// Validate that ISA13 and IEA02 control numbers match.
pub fn validate_isa_iea(isa13: &str, iea02: &str) -> bool {
    if isa13.trim() != iea02.trim() {
        warn!(
            "Envelope error: ISA13 ({}) != IEA02 ({}). Per X12 spec, these must be identical.",
            isa13, iea02
        );
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_se_match() {
        assert!(validate_st_se("0001", "0001"));
    }

    #[test]
    fn test_st_se_mismatch() {
        assert!(!validate_st_se("0001", "0002"));
    }

    #[test]
    fn test_segment_count_match() {
        assert!(validate_segment_count("15", 15));
    }

    #[test]
    fn test_segment_count_mismatch() {
        assert!(!validate_segment_count("15", 12));
    }

    #[test]
    fn test_gs_ge_match() {
        assert!(validate_gs_ge("1", "1"));
    }

    #[test]
    fn test_gs_ge_mismatch() {
        assert!(!validate_gs_ge("1", "2"));
    }

    #[test]
    fn test_isa_iea_match() {
        assert!(validate_isa_iea("000000905", "000000905"));
    }

    #[test]
    fn test_isa_iea_match_trimmed() {
        assert!(validate_isa_iea("000000905 ", "000000905"));
    }

    #[test]
    fn test_isa_iea_mismatch() {
        assert!(!validate_isa_iea("000000905", "000000906"));
    }
}

/// Validate the X12 envelope of raw EDI content.
/// Extracts ISA13/IEA02, GS06/GE02, ST02/SE02 and checks they match per spec.
pub fn validate_raw_envelope(contents: &str) {
    use crate::helper::edihelper::{get_element, get_segment_contents};

    // ISA13 vs IEA02
    let isa = get_segment_contents("ISA", contents);
    let iea = get_segment_contents("IEA", contents);
    if !isa.is_empty() && !iea.is_empty() {
        let isa_parts: Vec<&str> = isa.split('*').collect();
        let iea_parts: Vec<&str> = iea.split('*').collect();
        validate_isa_iea(&get_element(&isa_parts, 12), &get_element(&iea_parts, 1));
    }

    // GS06 vs GE02
    let gs = get_segment_contents("GS", contents);
    let ge = get_segment_contents("GE", contents);
    if !gs.is_empty() && !ge.is_empty() {
        let gs_parts: Vec<&str> = gs.split('*').collect();
        let ge_parts: Vec<&str> = ge.split('*').collect();
        validate_gs_ge(&get_element(&gs_parts, 5), &get_element(&ge_parts, 1));
    }

    // ST02 vs SE02
    let st = get_segment_contents("ST", contents);
    let se = get_segment_contents("SE", contents);
    if !st.is_empty() && !se.is_empty() {
        let st_parts: Vec<&str> = st.split('*').collect();
        let se_parts: Vec<&str> = se.split('*').collect();
        validate_st_se(&get_element(&st_parts, 1), &get_element(&se_parts, 1));
    }
}
