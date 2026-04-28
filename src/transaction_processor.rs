use crate::error::EdiResult;
use serde::{de::DeserializeOwned, Serialize};

/// Trait defining common behavior for all transaction sets

pub trait TransactionSet: Serialize + DeserializeOwned + Default {
    /// Parse EDI content into this transaction set
    fn parse(contents: String) -> EdiResult<(Self, String)>
    where
        Self: Sized;

    /// Convert this transaction set to EDI format
    fn to_edi(&self) -> String;

    /// Get the transaction type identifier (e.g., "835", "999", "270")
    fn get_transaction_type() -> &'static str;

    /// Detect if the given content matches this transaction set
    fn detect(contents: &str) -> bool;
}
