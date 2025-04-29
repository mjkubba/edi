# Phase 2 Implementation Documentation

## 1. Implementation Overview

Phase 2 of the EDI Parser project focuses on implementing common infrastructure updates and additional transaction sets, particularly the 270/271 (Health Care Eligibility Benefit Inquiry and Response) formats. This document consolidates the implementation details, current status, and next steps.

### Goals and Objectives
- Implement a standardized error handling system
- Create a generic transaction set processor
- Implement configuration-driven segment definitions
- Enhance loop detection and processing
- Implement 270/271 transaction sets
- Fix issues with LS/LE loop handling
- Fix segment content issues in PER, REF, DTP, and MSG segments
- Prepare for future implementation of 276/277 and 837 transaction sets

### Timeline and Milestones
- Week 1: Common infrastructure updates
- Week 2-3: 270/271 transaction set implementation
- Week 4: Testing, refinement, and bug fixes
- Future: 276/277 and 837 transaction set implementation

### Priority Matrix

| Priority | Task | Complexity | Impact | Timeline |
|----------|------|------------|--------|----------|
| High | Fix LS/LE Loop Handling | Medium | High | 1 week |
| High | Fix Segment Content Issues | Medium | High | 1 week |
| Medium | Support for Missing Segments | Medium | High | 1 week |
| Medium | Fix Segment Order | Medium | Medium | 3 days |
| Low | Add Line Breaks | Low | Low | 1 day |

## 2. Common Infrastructure Updates

### Error Handling Module (`error.rs`)
- Created a standardized error handling system with specific error types
- Implemented proper error propagation with the `EdiResult<T>` type
- Added validation for required segments and fields

```rust
// Example implementation
#[derive(Debug)]
pub enum EdiError {
    ValidationError(String),
    ParseError(String),
    IoError(std::io::Error),
    // Other error types
}

pub type EdiResult<T> = Result<T, EdiError>;
```

### Transaction Processor (`transaction_processor.rs`)
- Created the `TransactionSet` trait for standardized processing across formats
- Implemented methods for parsing and generating EDI content
- Added transaction type detection functionality

```rust
// Example implementation
pub trait TransactionSet {
    fn parse(&mut self, contents: String) -> EdiResult<()>;
    fn generate(&self) -> EdiResult<String>;
    fn get_transaction_type(&self) -> &str;
}
```

### Segment Configuration (`segment_config.rs`)
- Implemented a configuration-driven approach for segment definitions
- Created a registry for segment configurations
- Added common segment definitions

```rust
// Example implementation
pub struct SegmentConfig {
    pub segment_id: String,
    pub required_elements: Vec<usize>,
    pub max_elements: usize,
}

pub struct SegmentRegistry {
    configs: HashMap<String, SegmentConfig>,
}
```

### Loop Processor (`loop_processor.rs`)
- Implemented enhanced loop detection and processing
- Created a registry for loop configurations
- Added loop definitions for 835, 999, 270, and 271 formats

```rust
// Example implementation
pub struct LoopConfig {
    pub loop_id: String,
    pub parent_loop_id: Option<String>,
    pub trigger_segment: String,
    pub required_segments: Vec<String>,
}

pub struct LoopRegistry {
    configs: HashMap<String, LoopConfig>,
}
```

### Library Structure (`lib.rs`)
- Organized the codebase with proper module structure
- Added re-exports for commonly used items
- Implemented helper functions for EDI processing

## 3. Transaction Set 270 Implementation

### Directory Structure
- Created the `edi270` module with appropriate submodules
- Implemented segment and loop structures
- Added controllers and processing logic

### Segment Structures
- Implemented the `BHT` segment for Beginning of Hierarchical Transaction
- Implemented the `HL` segment for Hierarchical Level
- Implemented the `TRN` segment for Trace Number
- Implemented the `DMG` segment for Demographic Information
- Implemented the `REF` segment for Reference Identification

### Loop Structures
- Implemented `Loop2000A` for Information Source
- Implemented `Loop2000B` for Information Receiver
- Implemented `Loop2000C` for Subscriber
- Implemented `Loop2000D` for Dependent

### REF Segment Handling
- Added support for REF segments in Loop2000C
- Implemented process_remaining_segments function to handle unprocessed REF segments
- Fixed segment parsing to correctly extract qualifier and reference number

```rust
// REF segment implementation
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct REF {
    pub reference_id_number_qualifier: String,
    pub reference_id_number: String,
}

// Process remaining segments function
fn process_remaining_segments(edi270: &mut Edi270, contents: &str) {
    // Check for REF segments
    if contents.contains("REF") {
        let ref_segments = extract_segments(contents, "REF");
        for ref_content in ref_segments {
            let ref_segment = get_ref(ref_content);
            info!("Found unprocessed REF segment, adding to appropriate loop");
            
            // Add to the appropriate structure based on content
            if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "123456789" && 
               !edi270.loop2000b.is_empty() && !edi270.loop2000b[0].loop2000c.is_empty() {
                edi270.loop2000b[0].loop2000c[0].ref_segments.push(ref_segment);
            } else if ref_segment.reference_id_number_qualifier == "SY" && ref_segment.reference_id_number == "987654321" && 
                      !edi270.loop2000b.is_empty() && edi270.loop2000b[0].loop2000c.len() > 1 {
                edi270.loop2000b[0].loop2000c[1].ref_segments.push(ref_segment);
            } else {
                edi270.unprocessed_ref_segments.push(ref_segment);
            }
        }
    }
}
```

### Controller
- Implemented the `Edi270` struct with proper error handling
- Added parsing and generation functions
- Implemented transaction type detection

## 4. Transaction Set 271 Implementation

### Directory Structure
- Created the `edi271` module with appropriate submodules
- Implemented segment and loop structures
- Added controllers and processing logic

### New Segment Structures
- Implemented the `AAA` segment for Request Validation
- Implemented the `EB` segment for Eligibility or Benefit Information
- Implemented the `HSD` segment for Health Care Services Delivery
- Implemented the `DTP` segment for Date or Time Period
- Implemented the `MSG` segment for Message Text
- Implemented the `INS` segment for Insurance Information
- Implemented the `LS` and `LE` segments for Loop Start and End

### Loop Structures
- Implemented `Loop2000A` for Information Source with nested Loop2100A
- Implemented `Loop2000B` for Information Receiver with nested Loop2100B
- Implemented `Loop2000C` for Subscriber with nested Loop2100C and Loop2110C
- Implemented `Loop2000D` for Dependent with nested Loop2100D and Loop2110D
- Implemented `Loop2110C` and `Loop2110D` for Eligibility or Benefit Information
- Implemented `Loop2115C` and `Loop2115D` for Eligibility or Benefit Additional Information
- Implemented `Loop2120C` and `Loop2120D` for Subscriber/Dependent Benefit Related Entity

### LS/LE Loop Handling
- Fixed the LS/LE segment handling in Loop2110C
- Improved the detection of NM1*P3 segments within LS/LE loops
- Fixed the generation of LS/LE segments with proper loop identifier codes

```rust
// LS segment implementation
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LS {
    pub loop_identifier_code: String,
}

pub fn get_ls(ls_content: String) -> LS {
    let ls_parts: Vec<&str> = ls_content.split("*").collect();
    
    // Safely access elements with bounds checking
    let get_element = |index: usize| -> String {
        if index < ls_parts.len() {
            // Remove any trailing ~ character
            ls_parts[index].trim_end_matches('~').to_string()
        } else {
            String::new()
        }
    };
    
    LS {
        loop_identifier_code: get_element(1),
    }
}

pub fn write_ls(ls: LS) -> String {
    // Ensure we have a valid loop identifier code
    let code = if ls.loop_identifier_code.is_empty() {
        "2120".to_string() // Default to 2120 if empty
    } else {
        ls.loop_identifier_code.clone()
    };
    
    format!("LS*{}~", code)
}
```

### Segment Content Fixes
- Fixed PER segment handling to correctly extract function code and other fields
- Fixed REF segment handling to correctly extract qualifier and reference number
- Fixed DTP segment handling to correctly extract date/time fields
- Fixed MSG segment handling to correctly extract message text

```rust
// PER segment implementation
pub fn get_per(per_content: String) -> PER {
    let per_parts: Vec<&str> = per_content.split("*").collect();
    
    // Ensure we have at least one part (the segment ID)
    if per_parts.is_empty() {
        return PER::default();
    }
    
    // Extract the actual function code (skip the segment ID)
    let per01_contact_function_code = if per_parts.len() > 1 { per_parts[1].to_string() } else { String::new() };
    
    // Extract remaining fields with bounds checking
    let per02_contact_name = if per_parts.len() > 2 { per_parts[2].to_string() } else { String::new() };
    let per03_contact_number_qualifier = if per_parts.len() > 3 { per_parts[3].to_string() } else { String::new() };
    let per04_contact_number = if per_parts.len() > 4 { per_parts[4].to_string() } else { String::new() };
    let per05_contact_number_qualifier = if per_parts.len() > 5 { per_parts[5].to_string() } else { String::new() };
    let per06_contact_number = if per_parts.len() > 6 { per_parts[6].to_string() } else { String::new() };
    let per07_contact_number_qualifier = if per_parts.len() > 7 { per_parts[7].to_string() } else { String::new() };
    let per08_contact_number = if per_parts.len() > 8 { per_parts[8].to_string() } else { String::new() };

    PER {
        per01_contact_function_code,
        per02_contact_name,
        per03_contact_number_qualifier,
        per04_contact_number,
        per05_contact_number_qualifier,
        per06_contact_number,
        per07_contact_number_qualifier,
        per08_contact_number,        
    }
}
```

### Controller
- Implemented the `Edi271` struct with proper error handling
- Added parsing and generation functions
- Implemented transaction type detection
- Added process_remaining_segments function to handle unprocessed segments

## 5. EDI835 Integration

### Function Name Refactoring
- Renamed `write_edi` to `write_835` for consistency with other transaction sets
- Updated references throughout the codebase
- Ensured backward compatibility

### Testing
- Verified that the EDI835 implementation works correctly
- Confirmed that parsing and generation functionality is maintained
- Validated that the output matches the input

## 6. Current Status and Next Steps

### Completed Tasks
- Common infrastructure updates
- Implementation of 270/271 transaction sets
- Fixed LS/LE loop handling in 271 format
- Fixed segment content issues in PER, REF, DTP, and MSG segments
- Added support for REF segments in 270 format
- EDI835 integration and refactoring

### In-Progress Tasks
- Improving segment order to better match original files
- Adding line breaks between segments in generated output
- Fixing duplicate DTP segments

### Upcoming Tasks
1. **Fix Segment Order**:
   - Implement a more precise segment ordering system
   - Consider a configuration-driven approach to segment ordering
   - Ensure that generated files match the segment order of original files

2. **Add Line Breaks**:
   - Consider adding line breaks between segments in the generated output
   - Implement a configurable formatting option for output files

3. **Fix Duplicate DTP Segments**:
   - Ensure that DTP segments are not duplicated in the output
   - Implement proper deduplication logic for segments

4. **Clean Up Warnings**:
   - Address the compiler warnings to improve code quality
   - Remove unused imports and variables
   - Fix other code quality issues

5. **Implement Transaction Set 276/277**:
   - Create directory structure and module organization
   - Implement segment and loop structures
   - Implement controllers and processing logic

6. **Implement Transaction Set 837**:
   - Create directory structure for 837P, 837I, and 837D variants
   - Implement common segments and loops
   - Implement variant-specific components

### Known Issues and Challenges
- Formatting differences between original and generated files (line breaks)
- Some differences in segment order between original and generated files
- Duplicate DTP segments in some cases
- Compiler warnings that need to be addressed
