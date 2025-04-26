use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::error::{EdiResult, EdiError};

/// Configuration for an EDI loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConfig {
    pub loop_id: String,
    pub name: String,
    pub start_segment: String,
    pub end_segment: Option<String>,
    pub required_segments: Vec<String>,
    pub optional_segments: Vec<String>,
    pub child_loops: Vec<String>,
    pub max_occurrences: Option<usize>,
    pub transaction_set: String,
}

/// Registry for loop configurations
pub struct LoopRegistry {
    loops: HashMap<String, LoopConfig>,
    loops_by_transaction: HashMap<String, Vec<String>>,
}

impl LoopRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            loops: HashMap::new(),
            loops_by_transaction: HashMap::new(),
        }
    }
    
    /// Register a loop configuration
    pub fn register(&mut self, config: LoopConfig) {
        let transaction = config.transaction_set.clone();
        let loop_id = config.loop_id.clone();
        
        self.loops.insert(loop_id.clone(), config);
        
        // Add to transaction index
        self.loops_by_transaction
            .entry(transaction)
            .or_insert_with(Vec::new)
            .push(loop_id);
    }
    
    /// Get a loop configuration by ID
    pub fn get_config(&self, loop_id: &str) -> Option<&LoopConfig> {
        self.loops.get(loop_id)
    }
    
    /// Get all loop IDs for a transaction set
    pub fn get_loops_for_transaction(&self, transaction_set: &str) -> Vec<String> {
        self.loops_by_transaction
            .get(transaction_set)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Detect which loop the content belongs to
    pub fn detect_loop(&self, contents: &str, transaction_set: &str) -> Option<&LoopConfig> {
        let loop_ids = self.get_loops_for_transaction(transaction_set);
        
        for loop_id in loop_ids {
            if let Some(config) = self.loops.get(&loop_id) {
                if contents.starts_with(&config.start_segment) {
                    return Some(config);
                }
            }
        }
        
        None
    }
}

// Global loop registry
pub static LOOP_REGISTRY: Lazy<Mutex<LoopRegistry>> = Lazy::new(|| {
    let mut registry = LoopRegistry::new();
    
    // Register common loops
    register_835_loops(&mut registry);
    register_999_loops(&mut registry);
    
    Mutex::new(registry)
});

/// Register loops for 835 transaction set
fn register_835_loops(registry: &mut LoopRegistry) {
    // Loop 1000A - Payer Identification
    registry.register(LoopConfig {
        loop_id: "1000A".to_string(),
        name: "Payer Identification".to_string(),
        start_segment: "N1".to_string(),
        end_segment: None,
        required_segments: vec!["N1".to_string()],
        optional_segments: vec!["N3".to_string(), "N4".to_string(), "REF".to_string(), "PER".to_string()],
        child_loops: vec![],
        max_occurrences: Some(1),
        transaction_set: "835".to_string(),
    });
    
    // Loop 1000B - Payee Identification
    registry.register(LoopConfig {
        loop_id: "1000B".to_string(),
        name: "Payee Identification".to_string(),
        start_segment: "N1".to_string(),
        end_segment: None,
        required_segments: vec!["N1".to_string()],
        optional_segments: vec!["N3".to_string(), "N4".to_string(), "REF".to_string(), "RDM".to_string()],
        child_loops: vec![],
        max_occurrences: Some(1),
        transaction_set: "835".to_string(),
    });
    
    // Loop 2000 - Header Number
    registry.register(LoopConfig {
        loop_id: "2000".to_string(),
        name: "Header Number".to_string(),
        start_segment: "LX".to_string(),
        end_segment: None,
        required_segments: vec!["LX".to_string()],
        optional_segments: vec!["TS3".to_string(), "TS2".to_string()],
        child_loops: vec!["2100".to_string()],
        max_occurrences: None,
        transaction_set: "835".to_string(),
    });
    
    // Loop 2100 - Claim Payment Information
    registry.register(LoopConfig {
        loop_id: "2100".to_string(),
        name: "Claim Payment Information".to_string(),
        start_segment: "CLP".to_string(),
        end_segment: None,
        required_segments: vec!["CLP".to_string()],
        optional_segments: vec![
            "CAS".to_string(), "NM1".to_string(), "MIA".to_string(), 
            "MOA".to_string(), "REF".to_string(), "DTM".to_string(), 
            "PER".to_string(), "AMT".to_string(), "QTY".to_string()
        ],
        child_loops: vec!["2110".to_string()],
        max_occurrences: None,
        transaction_set: "835".to_string(),
    });
    
    // Loop 2110 - Service Payment Information
    registry.register(LoopConfig {
        loop_id: "2110".to_string(),
        name: "Service Payment Information".to_string(),
        start_segment: "SVC".to_string(),
        end_segment: None,
        required_segments: vec!["SVC".to_string()],
        optional_segments: vec![
            "DTM".to_string(), "CAS".to_string(), "REF".to_string(), 
            "AMT".to_string(), "QTY".to_string(), "LQ".to_string()
        ],
        child_loops: vec![],
        max_occurrences: None,
        transaction_set: "835".to_string(),
    });
}

/// Register loops for 999 transaction set
fn register_999_loops(registry: &mut LoopRegistry) {
    // Loop 2000 - Transaction Set Response Header
    registry.register(LoopConfig {
        loop_id: "2000".to_string(),
        name: "Transaction Set Response Header".to_string(),
        start_segment: "AK2".to_string(),
        end_segment: Some("IK5".to_string()),
        required_segments: vec!["AK2".to_string(), "IK5".to_string()],
        optional_segments: vec![],
        child_loops: vec!["2100".to_string()],
        max_occurrences: None,
        transaction_set: "999".to_string(),
    });
    
    // Loop 2100 - Error Identification
    registry.register(LoopConfig {
        loop_id: "2100".to_string(),
        name: "Error Identification".to_string(),
        start_segment: "IK3".to_string(),
        end_segment: None,
        required_segments: vec!["IK3".to_string()],
        optional_segments: vec!["CTX".to_string()],
        child_loops: vec!["2110".to_string()],
        max_occurrences: None,
        transaction_set: "999".to_string(),
    });
    
    // Loop 2110 - Implementation Data Element Note
    registry.register(LoopConfig {
        loop_id: "2110".to_string(),
        name: "Implementation Data Element Note".to_string(),
        start_segment: "IK4".to_string(),
        end_segment: None,
        required_segments: vec!["IK4".to_string()],
        optional_segments: vec!["CTX".to_string()],
        child_loops: vec![],
        max_occurrences: None,
        transaction_set: "999".to_string(),
    });
}

/// Helper function to extract a loop from EDI content
pub fn extract_loop(contents: &str, config: &LoopConfig) -> EdiResult<(String, String)> {
    let start_pos = match contents.find(&config.start_segment) {
        Some(pos) => pos,
        None => return Err(EdiError::MissingSegment(config.start_segment.clone())),
    };
    
    let end_pos = if let Some(end_segment) = &config.end_segment {
        match contents[start_pos..].find(end_segment) {
            Some(pos) => start_pos + pos + end_segment.len() + 1, // +1 for the segment terminator
            None => contents.len(),
        }
    } else {
        // Find the next loop start or end of content
        let mut next_pos = contents.len();
        
        for child_id in &config.child_loops {
            if let Some(child_config) = LOOP_REGISTRY.lock().unwrap().get_config(child_id) {
                if let Some(pos) = contents[start_pos..].find(&child_config.start_segment) {
                    let pos = start_pos + pos;
                    if pos < next_pos {
                        next_pos = pos;
                    }
                }
            }
        }
        
        next_pos
    };
    
    let loop_content = contents[start_pos..end_pos].to_string();
    let remaining = contents[end_pos..].to_string();
    
    Ok((loop_content, remaining))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_registry() {
        let registry = LOOP_REGISTRY.lock().unwrap();
        
        // Test 835 loops are registered
        let loops_835 = registry.get_loops_for_transaction("835");
        assert!(loops_835.contains(&"1000A".to_string()));
        assert!(loops_835.contains(&"1000B".to_string()));
        assert!(loops_835.contains(&"2000".to_string()));
        assert!(loops_835.contains(&"2100".to_string()));
        assert!(loops_835.contains(&"2110".to_string()));
        
        // Test 999 loops are registered
        let loops_999 = registry.get_loops_for_transaction("999");
        assert!(loops_999.contains(&"2000".to_string()));
        assert!(loops_999.contains(&"2100".to_string()));
        assert!(loops_999.contains(&"2110".to_string()));
        
        // Test loop retrieval
        let loop_1000a = registry.get_config("1000A").unwrap();
        assert_eq!(loop_1000a.loop_id, "1000A");
        assert_eq!(loop_1000a.name, "Payer Identification");
        assert_eq!(loop_1000a.start_segment, "N1");
        assert_eq!(loop_1000a.transaction_set, "835");
    }
}
