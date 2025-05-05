use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Configuration for an EDI segment element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementConfig {
    pub name: String,
    pub required: bool,
    pub max_length: usize,
    pub description: String,
}

/// Configuration for an EDI segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentConfig {
    pub segment_id: String,
    pub name: String,
    pub elements: Vec<ElementConfig>,
    pub description: String,
}

/// Registry for segment configurations
pub struct SegmentRegistry {
    segments: HashMap<String, SegmentConfig>,
}

impl SegmentRegistry {
    /// Create a new empty registry
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            segments: HashMap::new(),
        }
    }
    
    /// Register a segment configuration
    pub fn register(&mut self, config: SegmentConfig) {
        self.segments.insert(config.segment_id.clone(), config);
    }
    
    /// Get a segment configuration by ID
    #[allow(dead_code)]
    pub fn get_config(&self, segment_id: &str) -> Option<&SegmentConfig> {
        self.segments.get(segment_id)
    }
    
    /// Check if a segment ID is registered
    pub fn has_segment(&self, segment_id: &str) -> bool {
        self.segments.contains_key(segment_id)
    }
    
    /// Get all registered segment IDs
    #[allow(dead_code)]
    pub fn get_segment_ids(&self) -> Vec<String> {
        self.segments.keys().cloned().collect()
    }
}

// Global segment registry
pub static SEGMENT_REGISTRY: Lazy<Mutex<SegmentRegistry>> = Lazy::new(|| {
    let mut registry = SegmentRegistry::new();
    
    // Register common segments
    register_common_segments(&mut registry);
    
    Mutex::new(registry)
});

/// Register common segments used across multiple transaction sets
fn register_common_segments(registry: &mut SegmentRegistry) {
    // ISA - Interchange Control Header
    registry.register(SegmentConfig {
        segment_id: "ISA".to_string(),
        name: "Interchange Control Header".to_string(),
        description: "To start and identify an interchange of zero or more functional groups and interchange-related control segments".to_string(),
        elements: vec![
            ElementConfig {
                name: "authorization_information_qualifier".to_string(),
                required: true,
                max_length: 2,
                description: "Code to identify the type of information in the Authorization Information".to_string(),
            },
            ElementConfig {
                name: "authorization_information".to_string(),
                required: true,
                max_length: 10,
                description: "Information used for additional identification or authorization of the interchange sender or the data in the interchange".to_string(),
            },
            // Add other ISA elements...
        ],
    });
    
    // GS - Functional Group Header
    registry.register(SegmentConfig {
        segment_id: "GS".to_string(),
        name: "Functional Group Header".to_string(),
        description: "To indicate the beginning of a functional group and to provide control information".to_string(),
        elements: vec![
            ElementConfig {
                name: "functional_identifier_code".to_string(),
                required: true,
                max_length: 2,
                description: "Code identifying a group of application related transaction sets".to_string(),
            },
            // Add other GS elements...
        ],
    });
    
    // ST - Transaction Set Header
    registry.register(SegmentConfig {
        segment_id: "ST".to_string(),
        name: "Transaction Set Header".to_string(),
        description: "To indicate the start of a transaction set and to assign a control number".to_string(),
        elements: vec![
            ElementConfig {
                name: "transaction_set_identifier_code".to_string(),
                required: true,
                max_length: 3,
                description: "Code uniquely identifying a Transaction Set".to_string(),
            },
            ElementConfig {
                name: "transaction_set_control_number".to_string(),
                required: true,
                max_length: 9,
                description: "Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set".to_string(),
            },
            // Add other ST elements...
        ],
    });
    
    // SE - Transaction Set Trailer
    registry.register(SegmentConfig {
        segment_id: "SE".to_string(),
        name: "Transaction Set Trailer".to_string(),
        description: "To indicate the end of a transaction set and provide the count of the transmitted segments".to_string(),
        elements: vec![
            ElementConfig {
                name: "number_of_included_segments".to_string(),
                required: true,
                max_length: 10,
                description: "Total number of segments included in a transaction set including ST and SE segments".to_string(),
            },
            ElementConfig {
                name: "transaction_set_control_number".to_string(),
                required: true,
                max_length: 9,
                description: "Identifying control number that must be unique within the transaction set functional group assigned by the originator for a transaction set".to_string(),
            },
        ],
    });
    
    // GE - Functional Group Trailer
    registry.register(SegmentConfig {
        segment_id: "GE".to_string(),
        name: "Functional Group Trailer".to_string(),
        description: "To indicate the end of a functional group and to provide control information".to_string(),
        elements: vec![
            ElementConfig {
                name: "number_of_transaction_sets_included".to_string(),
                required: true,
                max_length: 6,
                description: "Total number of transaction sets included in the functional group or interchange (transmission) group terminated by the trailer containing this data element".to_string(),
            },
            ElementConfig {
                name: "group_control_number".to_string(),
                required: true,
                max_length: 9,
                description: "Assigned number originated and maintained by the sender".to_string(),
            },
        ],
    });
    
    // IEA - Interchange Control Trailer
    registry.register(SegmentConfig {
        segment_id: "IEA".to_string(),
        name: "Interchange Control Trailer".to_string(),
        description: "To define the end of an interchange of zero or more functional groups and interchange-related control segments".to_string(),
        elements: vec![
            ElementConfig {
                name: "number_of_included_functional_groups".to_string(),
                required: true,
                max_length: 5,
                description: "A count of the number of functional groups included in an interchange".to_string(),
            },
            ElementConfig {
                name: "interchange_control_number".to_string(),
                required: true,
                max_length: 9,
                description: "A control number assigned by the interchange sender".to_string(),
            },
        ],
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_segment_registry() {
        let registry = SEGMENT_REGISTRY.lock().unwrap();
        
        // Test common segments are registered
        assert!(registry.has_segment("ISA"));
        assert!(registry.has_segment("GS"));
        assert!(registry.has_segment("ST"));
        assert!(registry.has_segment("SE"));
        assert!(registry.has_segment("GE"));
        assert!(registry.has_segment("IEA"));
        
        // Test segment retrieval
        let isa_config = registry.get_config("ISA").unwrap();
        assert_eq!(isa_config.segment_id, "ISA");
        assert_eq!(isa_config.name, "Interchange Control Header");
        
        // Test element configuration
        assert!(!isa_config.elements.is_empty());
        assert_eq!(isa_config.elements[0].name, "authorization_information_qualifier");
        assert!(isa_config.elements[0].required);
    }
}
