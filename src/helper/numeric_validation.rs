use log::warn;

/// Validate an X12 Numeric (Nn) data element.
/// Per X12 §B.1.1.3.1.1: digits only, optional leading minus, no plus sign,
/// no commas, no decimal point. Leading zeros suppressed unless for min length.
pub fn is_valid_numeric(value: &str) -> bool {
    if value.is_empty() {
        return true; // empty = absent, not invalid
    }
    let chars = if value.starts_with('-') {
        &value[1..]
    } else {
        value
    };
    !chars.is_empty() && chars.chars().all(|c| c.is_ascii_digit())
}

/// Validate an X12 Decimal (R) data element.
/// Per X12 §B.1.1.3.1.2: digits with optional explicit decimal point,
/// optional leading minus. No plus sign, no commas, no exponential notation.
/// HIPAA limits DE 782 (Monetary Amount) to max 10 chars excluding sign and decimal.
pub fn is_valid_decimal(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    let chars = if value.starts_with('-') {
        &value[1..]
    } else {
        value
    };
    if chars.is_empty() {
        return false;
    }
    let mut dot_count = 0;
    for c in chars.chars() {
        if c == '.' {
            dot_count += 1;
            if dot_count > 1 {
                return false;
            }
        } else if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

/// Warn if a monetary amount field contains an invalid value.
pub fn warn_if_invalid_monetary(segment: &str, field: &str, value: &str) {
    if !value.is_empty() && !is_valid_decimal(value) {
        warn!(
            "Invalid monetary value in {}.{}: '{}' — expected decimal (R) per X12 §B.1.1.3.1.2",
            segment, field, value
        );
    }
}

/// Warn if a numeric (N0) field contains an invalid value.
pub fn warn_if_invalid_numeric(segment: &str, field: &str, value: &str) {
    if !value.is_empty() && !is_valid_numeric(value) {
        warn!(
            "Invalid numeric value in {}.{}: '{}' — expected integer (N0) per X12 §B.1.1.3.1.1",
            segment, field, value
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_numeric() {
        assert!(is_valid_numeric(""));
        assert!(is_valid_numeric("0"));
        assert!(is_valid_numeric("12345"));
        assert!(is_valid_numeric("-1"));
        assert!(is_valid_numeric("000000905"));
    }

    #[test]
    fn test_invalid_numeric() {
        assert!(!is_valid_numeric("12.34"));
        assert!(!is_valid_numeric("abc"));
        assert!(!is_valid_numeric("+1"));
        assert!(!is_valid_numeric("1,000"));
        assert!(!is_valid_numeric("-"));
    }

    #[test]
    fn test_valid_decimal() {
        assert!(is_valid_decimal(""));
        assert!(is_valid_decimal("0"));
        assert!(is_valid_decimal("100"));
        assert!(is_valid_decimal("12.34"));
        assert!(is_valid_decimal("-99999999.99"));
        assert!(is_valid_decimal("99999999"));
    }

    #[test]
    fn test_invalid_decimal() {
        assert!(!is_valid_decimal("abc"));
        assert!(!is_valid_decimal("+1"));
        assert!(!is_valid_decimal("1,000.00"));
        assert!(!is_valid_decimal("1.2.3"));
        assert!(!is_valid_decimal("-"));
        assert!(!is_valid_decimal("1e5"));
    }
}
