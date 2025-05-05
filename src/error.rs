use std::fmt;

/// Result type for EDI operations
pub type EdiResult<T> = Result<T, EdiError>;

/// Error types for EDI operations
#[derive(Debug)]
pub enum EdiError {
    /// Error parsing EDI content
    ParseError(String),
    
    /// Error validating EDI content
    ValidationError(String),
    
    /// I/O error
    IoError(std::io::Error),
    
    /// Missing required segment
    MissingSegment(String),
    
    /// Malformed segment
    MalformedSegment(String),
    
    /// Unsupported format
    UnsupportedFormat(String),
    
    /// Missing required field
    MissingField(String),
    
    /// Invalid field value
    InvalidFieldValue(String),
    
    /// Invalid segment order
    InvalidSegmentOrder(String),
}

impl fmt::Display for EdiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            EdiError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            EdiError::IoError(err) => write!(f, "I/O error: {}", err),
            EdiError::MissingSegment(segment) => write!(f, "Missing required segment: {}", segment),
            EdiError::MalformedSegment(segment) => write!(f, "Malformed segment: {}", segment),
            EdiError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
            EdiError::MissingField(field) => write!(f, "Missing required field: {}", field),
            EdiError::InvalidFieldValue(msg) => write!(f, "Invalid field value: {}", msg),
            EdiError::InvalidSegmentOrder(msg) => write!(f, "Invalid segment order: {}", msg),
        }
    }
}

impl std::error::Error for EdiError {}

impl From<std::io::Error> for EdiError {
    fn from(err: std::io::Error) -> Self {
        EdiError::IoError(err)
    }
}

impl From<serde_json::Error> for EdiError {
    fn from(err: serde_json::Error) -> Self {
        EdiError::ParseError(err.to_string())
    }
}
