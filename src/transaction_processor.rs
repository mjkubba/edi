use serde::{Serialize, de::DeserializeOwned};

/// Trait defining common behavior for all transaction sets
pub trait TransactionSet: Serialize + DeserializeOwned + Default {
    /// Parse EDI content into this transaction set
    fn parse(contents: String) -> (Self, String) where Self: Sized;
    
    /// Convert this transaction set to EDI format
    fn to_edi(&self) -> String;
    
    /// Get the transaction type identifier (e.g., "835", "999", "270")
    fn get_transaction_type() -> &'static str;
    
    /// Detect if the given content matches this transaction set
    fn detect(contents: &str) -> bool;
}

/// Generic processor for transaction sets
pub struct TransactionProcessor;

impl TransactionProcessor {
    /// Process EDI content into a specific transaction set
    pub fn process<T: TransactionSet>(contents: String) -> T {
        if !T::detect(&contents) {
            panic!("Content does not match transaction set {}", T::get_transaction_type());
        }
        
        let (transaction, remaining) = T::parse(contents);
        
        if !remaining.is_empty() {
            log::warn!("Unprocessed segments: {}", remaining);
        }
        
        transaction
    }
    
    /// Convert a transaction set to EDI format
    pub fn write<T: TransactionSet>(transaction: T) -> String {
        transaction.to_edi()
    }
    
    /// Detect the transaction set type from content
    pub fn detect_transaction_type(contents: &str) -> Option<&'static str> {
        if contents.contains("ST*835*") {
            Some("835")
        } else if contents.contains("ST*999*") {
            Some("999")
        } else if contents.contains("ST*270*") {
            Some("270")
        } else if contents.contains("ST*271*") {
            Some("271")
        } else if contents.contains("ST*276*") {
            Some("276")
        } else if contents.contains("ST*277*") {
            Some("277")
        } else if contents.contains("ST*837*") {
            Some("837")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_transaction_type() {
        assert_eq!(TransactionProcessor::detect_transaction_type("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~"), Some("835"));
        assert_eq!(TransactionProcessor::detect_transaction_type("ISA*00* *00**ZZ*123456789*ZZ*987654321*041117*1024*^*00501*000000286*0*P*:~GS*FA*RCVR*SNDR*20041117*1024*287*X*005010X231~ST*999*2870001*005010X231~"), Some("999"));
        assert_eq!(TransactionProcessor::detect_transaction_type("ISA*00*          *00*          *ZZ*SUBMITTERS ID  *ZZ*RECEIVERS ID   *200101*1253*^*00501*000000905*0*T*|~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*270*35681~"), Some("270"));
        assert_eq!(TransactionProcessor::detect_transaction_type("Unknown content"), None);
    }
}
