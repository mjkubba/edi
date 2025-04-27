pub mod error;
pub mod transaction_processor;
pub mod segment_config;
pub mod loop_processor;
pub mod edi835;
pub mod edi999;
pub mod edi270;
pub mod helper;
pub mod segments;

// Re-export commonly used items
pub use transaction_processor::{TransactionSet, TransactionProcessor};

/// Detect the transaction set type from EDI content
pub fn detect_transaction_type(contents: &str) -> Option<&'static str> {
    TransactionProcessor::detect_transaction_type(contents)
}

/// Process EDI content into a specific transaction set
pub fn process_edi<T: TransactionSet>(contents: String) -> T {
    TransactionProcessor::process(contents)
}

/// Convert a transaction set to EDI format
pub fn write_edi<T: TransactionSet>(transaction: T) -> String {
    TransactionProcessor::write(transaction)
}
