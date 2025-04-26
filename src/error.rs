// This file is a placeholder for future error handling implementation
// It will be implemented in a later phase

pub type EdiResult<T> = Result<T, EdiError>;

#[derive(Debug)]
pub enum EdiError {
    ParseError(String),
    ValidationError(String),
    IoError(std::io::Error),
    MissingSegment(String),
    MalformedSegment(String),
    UnsupportedFormat(String),
}

impl From<std::io::Error> for EdiError {
    fn from(err: std::io::Error) -> Self {
        EdiError::IoError(err)
    }
}
