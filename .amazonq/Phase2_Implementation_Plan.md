# EDI Parser Phase 2 Implementation Plan

## Overview

Based on our comprehensive testing of the EDI parser with X279 files, we've identified several areas for improvement. This document outlines a structured implementation plan to address these issues and enhance the parser's capabilities for handling complex real-world EDI files.

## Implementation Priorities

| Priority | Task | Complexity | Impact | Timeline |
|----------|------|------------|--------|----------|
| High | Support for Missing Segments (DTP, EQ, NM1*P3) | Medium | High | 1 week |
| Medium | Fix LS/LE Loop Handling | Medium | High | 3 days |
| Medium | Add Validation | Medium | Medium | 3 days |
| Low | Improve Segment Order | Low | Medium | 2 days |

## Detailed Implementation Plan

### 1. Support for Missing Segments

#### 1.1 DTP Segment in 270 Format

**Tasks:**
- Create DTP segment structure in the `edi270/segments` module
- Add parsing logic in `edi270/controller.rs`
- Add generation logic in `edi270/controller.rs`
- Update the Loop2000C structure to include DTP segments
- Add tests for DTP segment parsing and generation

**Implementation Details:**
```rust
// In edi270/segments/dtp.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DTP {
    pub dtp01_date_time_qualifier: String,
    pub dtp02_date_time_format_qualifier: String,
    pub dtp03_date_time_value: String,
}

// In edi270/loop2000c.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    // Existing fields
    pub dtp_segments: Vec<DTP>,
    // Other fields
}

// In edi270/controller.rs
pub fn get_dtp(dtp_content: String) -> DTP {
    let dtp_parts: Vec<&str> = dtp_content.split('*').collect();
    
    DTP {
        dtp01_date_time_qualifier: dtp_parts.get(1).unwrap_or(&"").to_string(),
        dtp02_date_time_format_qualifier: dtp_parts.get(2).unwrap_or(&"").to_string(),
        dtp03_date_time_value: dtp_parts.get(3).unwrap_or(&"").to_string(),
    }
}

pub fn write_dtp(dtp: DTP) -> String {
    format!("DTP*{}*{}*{}~", 
        dtp.dtp01_date_time_qualifier,
        dtp.dtp02_date_time_format_qualifier,
        dtp.dtp03_date_time_value
    )
}
```

#### 1.2 EQ Segment in 270 Format

**Tasks:**
- Create EQ segment structure in the `edi270/segments` module
- Add parsing logic in `edi270/controller.rs`
- Add generation logic in `edi270/controller.rs`
- Update the Loop2000C structure to include EQ segments
- Add tests for EQ segment parsing and generation

**Implementation Details:**
```rust
// In edi270/segments/eq.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EQ {
    pub eq01_service_type_code: String,
    pub eq02_composite_medical_procedure_identifier: String,
    pub eq03_coverage_level_code: String,
    pub eq04_insurance_type_code: String,
}

// In edi270/loop2000c.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    // Existing fields
    pub eq_segments: Vec<EQ>,
    // Other fields
}

// In edi270/controller.rs
pub fn get_eq(eq_content: String) -> EQ {
    let eq_parts: Vec<&str> = eq_content.split('*').collect();
    
    EQ {
        eq01_service_type_code: eq_parts.get(1).unwrap_or(&"").to_string(),
        eq02_composite_medical_procedure_identifier: eq_parts.get(2).unwrap_or(&"").to_string(),
        eq03_coverage_level_code: eq_parts.get(3).unwrap_or(&"").to_string(),
        eq04_insurance_type_code: eq_parts.get(4).unwrap_or(&"").to_string(),
    }
}

pub fn write_eq(eq: EQ) -> String {
    format!("EQ*{}*{}*{}*{}~", 
        eq.eq01_service_type_code,
        eq.eq02_composite_medical_procedure_identifier,
        eq.eq03_coverage_level_code,
        eq.eq04_insurance_type_code
    )
}
```

#### 1.3 NM1*P3 Segment in LS/LE Loop

**Tasks:**
- Update the Loop2115C structure to properly handle NM1*P3 segments
- Enhance the parsing logic to correctly identify and process NM1*P3 segments within LS/LE loops
- Update the generation logic to include NM1*P3 segments in the output
- Add tests for NM1*P3 segment parsing and generation

**Implementation Details:**
```rust
// In edi271/loop2115c.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop2115C {
    pub nm1_segments: NM1,
    // Other fields
}

// In edi271/controller.rs
pub fn get_loop_2115c(contents: String) -> Vec<Loop2115C> {
    let mut loop2115c_vec: Vec<Loop2115C> = Vec::new();
    
    // Extract content between LS and LE
    if let Some(ls_le_content) = extract_between_segments(contents.clone(), "LS", "LE") {
        // Find all NM1*P3 segments
        let nm1_segments = extract_segments(ls_le_content, "NM1");
        
        for nm1_content in nm1_segments {
            if nm1_content.contains("NM1*P3") {
                let nm1 = get_nm1(nm1_content);
                
                let loop2115c = Loop2115C {
                    nm1_segments: nm1,
                    // Initialize other fields
                };
                
                loop2115c_vec.push(loop2115c);
            }
        }
    }
    
    loop2115c_vec
}

pub fn write_loop_2115c(loop2115c: &Loop2115C) -> String {
    let mut contents = String::new();
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2115c.nm1_segments.clone()));
    
    // Write other segments...
    
    contents
}
```

### 2. Fix LS/LE Loop Handling

#### 2.1 Fix LS Segment Generation

**Tasks:**
- Update the LS segment structure to ensure the loop identifier code is included
- Modify the write_ls function to properly format the LS segment
- Add validation for LS segment content
- Add tests for LS segment parsing and generation

**Implementation Details:**
```rust
// In edi271/segments/ls.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LS {
    pub loop_identifier_code: String,
}

// In edi271/controller.rs
pub fn get_ls(ls_content: String) -> LS {
    let ls_parts: Vec<&str> = ls_content.split('*').collect();
    
    LS {
        loop_identifier_code: ls_parts.get(1).unwrap_or(&"").to_string(),
    }
}

pub fn write_ls(ls: LS) -> String {
    format!("LS*{}~", ls.loop_identifier_code)
}
```

#### 2.2 Improve LS/LE Loop Placement

**Tasks:**
- Update the loop structure to ensure LS/LE segments wrap around the appropriate content
- Modify the write_loop_2110c function to properly place LS/LE segments
- Add tests for LS/LE loop structure

**Implementation Details:**
```rust
// In edi271/controller.rs
pub fn write_loop_2110c(loop2110c: &Loop2110C) -> String {
    let mut contents = String::new();
    
    // Write EB segment
    contents.push_str(&write_eb(loop2110c.eb_segments.clone()));
    
    // Write other segments...
    
    // Write LS and LE segments with NM1 in between
    if let Some(ls) = &loop2110c.ls {
        contents.push_str(&write_ls(ls.clone()));
        
        // Write Loop 2115C segments - these should include the NM1 segments
        for loop2115c in &loop2110c.loop2115c {
            contents.push_str(&write_loop_2115c(loop2115c));
        }
        
        if let Some(le) = &loop2110c.le {
            contents.push_str(&write_le(le.clone()));
        }
    } else {
        // If no LS/LE, just write the Loop 2115C segments normally
        for loop2115c in &loop2110c.loop2115c {
            contents.push_str(&write_loop_2115c(loop2115c));
        }
    }
    
    contents
}
```

### 3. Add Validation

#### 3.1 Implement Required Segment Validation

**Tasks:**
- Define which segments are required for each transaction set
- Add validation to ensure required segments are present
- Provide meaningful error messages when required segments are missing
- Add tests for segment validation

**Implementation Details:**
```rust
// In error.rs
#[derive(Debug)]
pub enum EdiError {
    // Existing error types
    ValidationError(String),
}

// In edi270/controller.rs
pub fn validate_270(edi270: &Edi270) -> EdiResult<()> {
    // Validate required segments
    if edi270.table1.bht_segments.bht01_hierarchical_structure_code.is_empty() {
        return Err(EdiError::ValidationError("BHT01 is required".to_string()));
    }
    
    if edi270.loop2000a.hl_segments.hl01_hierarchical_id_number.is_empty() {
        return Err(EdiError::ValidationError("HL01 in Loop 2000A is required".to_string()));
    }
    
    // Validate other required fields...
    
    Ok(())
}

// In edi271/controller.rs
pub fn validate_271(edi271: &Edi271) -> EdiResult<()> {
    // Similar validation logic for 271
    Ok(())
}
```

#### 3.2 Implement Segment Order Validation

**Tasks:**
- Define the expected order of segments for each transaction set
- Add validation to ensure segments are in the correct order
- Provide warnings when segments are out of order
- Add tests for segment order validation

**Implementation Details:**
```rust
// In validation.rs
pub fn validate_segment_order(segments: &[String], expected_order: &[String]) -> Vec<String> {
    let mut warnings = Vec::new();
    
    // Check if segments are in the expected order
    for (i, segment) in segments.iter().enumerate() {
        if i < expected_order.len() && segment != &expected_order[i] {
            warnings.push(format!("Segment {} is out of order. Expected {}", segment, expected_order[i]));
        }
    }
    
    warnings
}

// Usage in controller.rs
pub fn validate_270_order(contents: &str) -> Vec<String> {
    let segments = extract_all_segment_ids(contents);
    let expected_order = vec![
        "ST".to_string(), "BHT".to_string(), "HL".to_string(), "NM1".to_string(),
        "HL".to_string(), "NM1".to_string(), "HL".to_string(), "TRN".to_string(),
        "NM1".to_string(), "DMG".to_string(), "DTP".to_string(), "EQ".to_string(),
    ];
    
    validate_segment_order(&segments, &expected_order)
}
```

### 4. Improve Segment Order

#### 4.1 Enhance Segment Ordering Logic

**Tasks:**
- Review the segment order in the original files
- Update the write functions to match the segment order in the original files
- Add tests for segment ordering

**Implementation Details:**
```rust
// In edi270/controller.rs
pub fn write_loop_2000c(loop2000c: &Loop2000C) -> String {
    let mut contents = String::new();
    
    // Write HL segment
    contents.push_str(&write_hl(loop2000c.hl_segments.clone()));
    
    // Write TRN segment if present - in original file, TRN comes after HL
    if let Some(trn) = &loop2000c.trn_segments {
        contents.push_str(&write_trn(trn.clone()));
    }
    
    // Write NM1 segment
    contents.push_str(&write_nm1(loop2000c.nm1_segments.clone()));
    
    // Write N3 segment if present
    if let Some(n3) = &loop2000c.n3_segments {
        contents.push_str(&write_n3(n3.clone()));
    }
    
    // Write N4 segment if present
    if let Some(n4) = &loop2000c.n4_segments {
        contents.push_str(&write_n4(n4.clone()));
    }
    
    // Write DMG segment if present
    if let Some(dmg) = &loop2000c.dmg_segments {
        contents.push_str(&write_dmg(dmg.clone()));
    }
    
    // Write DTP segments
    for dtp in &loop2000c.dtp_segments {
        contents.push_str(&write_dtp(dtp.clone()));
    }
    
    // Write EQ segments
    for eq in &loop2000c.eq_segments {
        contents.push_str(&write_eq(eq.clone()));
    }
    
    // Write other segments...
    
    contents
}
```

#### 4.2 Implement Segment Order Configuration

**Tasks:**
- Create a configuration system for segment order
- Allow the segment order to be defined in a configuration file or structure
- Use the configuration to determine the order of segments in the output
- Add tests for configuration-driven segment ordering

**Implementation Details:**
```rust
// In segment_config.rs
#[derive(Debug, Clone)]
pub struct SegmentOrderConfig {
    pub transaction_set: String,
    pub loop_id: String,
    pub segment_order: Vec<String>,
}

impl SegmentOrderConfig {
    pub fn new(transaction_set: &str, loop_id: &str, segment_order: Vec<String>) -> Self {
        Self {
            transaction_set: transaction_set.to_string(),
            loop_id: loop_id.to_string(),
            segment_order,
        }
    }
}

pub struct SegmentOrderRegistry {
    configs: HashMap<String, HashMap<String, SegmentOrderConfig>>,
}

impl SegmentOrderRegistry {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, config: SegmentOrderConfig) {
        let transaction_map = self.configs
            .entry(config.transaction_set.clone())
            .or_insert_with(HashMap::new);
            
        transaction_map.insert(config.loop_id.clone(), config);
    }
    
    pub fn get_segment_order(&self, transaction_set: &str, loop_id: &str) -> Option<&Vec<String>> {
        self.configs.get(transaction_set)
            .and_then(|transaction_map| transaction_map.get(loop_id))
            .map(|config| &config.segment_order)
    }
}

// Initialize with default configurations
pub fn initialize_segment_order_registry() -> SegmentOrderRegistry {
    let mut registry = SegmentOrderRegistry::new();
    
    // 270 Loop 2000C segment order
    registry.register(SegmentOrderConfig::new(
        "270",
        "2000C",
        vec![
            "HL".to_string(),
            "TRN".to_string(),
            "NM1".to_string(),
            "N3".to_string(),
            "N4".to_string(),
            "DMG".to_string(),
            "DTP".to_string(),
            "EQ".to_string(),
        ]
    ));
    
    // Add more configurations...
    
    registry
}
```

## Testing Strategy

### Unit Tests

- Create unit tests for each new segment type
- Test parsing and generation of each segment
- Test validation functions
- Test segment order functions

### Integration Tests

- Test end-to-end parsing and generation with sample files
- Compare generated output with original input
- Verify that all segments are processed correctly
- Verify that segment order matches the expected order

### Regression Tests

- Ensure existing functionality continues to work
- Test with previously working files
- Verify that no new issues are introduced

## Timeline

| Week | Tasks |
|------|-------|
| Week 1 | Implement support for missing segments (DTP, EQ, NM1*P3) |
| Week 2 | Fix LS/LE loop handling and add validation |
| Week 3 | Improve segment order and conduct comprehensive testing |

## Conclusion

This implementation plan addresses the issues identified during testing of the X279 files. By following this plan, we will enhance the EDI parser's capabilities to better handle complex real-world EDI files. The focus is on supporting all segments, fixing the LS/LE loop handling, improving segment order, and adding validation to ensure the generated files match the expected format.

Once these improvements are implemented, the parser will be better equipped to handle a wider range of EDI files, including complex X279 variants, making it more robust and reliable for production use.
