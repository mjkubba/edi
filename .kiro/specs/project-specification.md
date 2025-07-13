# EDI Parser Project Specification and Overview

## 1. Introduction

This document outlines the development plan for enhancing an existing open-source application that converts X12 EDI formats to JSON. The application currently supports multiple healthcare X12 transaction sets and this specification details the approach for implementing additional X12 formats and enhancing existing implementations.

### Project Purpose and Scope
- Convert EDI formats to JSON and vice versa
- Support multiple healthcare X12 transaction sets (835, 999, 270/271, 276/277, 837P/I/D)
- Provide robust error handling and validation
- Maintain a modular and extensible architecture
- Support future implementation of additional transaction sets (278, 820, 834)

### Current State Assessment
The application is built in Rust and follows a modular architecture with separate components for:
- Segment parsing and representation
- Loop structure handling
- Transaction set processing
- File I/O operations

Currently implemented transaction sets:
- 835 (Health Care Claim Payment/Advice) - ASC X12N/005010X221
- 999 (Implementation Acknowledgment) - ASC X12C/005010X231
- 270/271 (Health Care Eligibility Benefit Inquiry and Response) - ASC X12N/005010X279
- 276/277 (Health Care Claim Status Request and Response) - ASC X12N/005010X212
- 837P (Health Care Claim: Professional) - ASC X12N/005010X222
- 837I (Health Care Claim: Institutional) - ASC X12N/005010X223
- 837D (Health Care Claim: Dental) - ASC X12N/005010X224

## 2. Architecture

### Directory Structure
```
edi/
├── src/
│   ├── edi835/                  # Healthcare Claim Payment/Advice format implementation
│   │   ├── controller.rs        # Main control logic for 835 processing
│   │   ├── interchangecontrol.rs # Interchange control handling
│   │   ├── interchangecontroltrailer.rs # Trailer handling
│   │   ├── loop1000a.rs        # Implementation of 1000A loop
│   │   ├── loop1000b.rs        # Implementation of 1000B loop
│   │   ├── loop2000.rs         # Implementation of 2000 loop
│   │   ├── loop2100.rs         # Implementation of 2100 loop
│   │   ├── loop2110.rs         # Implementation of 2110 loop
│   │   ├── table1.rs           # Table 1 definitions
│   │   └── table3.rs           # Table 3 definitions
│   ├── edi999/                  # Implementation Acknowledgment format implementation
│   │   ├── controller.rs        # Main control logic for 999 processing
│   │   ├── interchangecontrol.rs # Interchange control handling
│   │   ├── interchangecontroltrailer.rs # Trailer handling
│   │   ├── loop2000.rs         # Implementation of 999 2000 loop
│   │   ├── loop2100.rs         # Implementation of 999 2100 loop
│   │   ├── loop2110.rs         # Implementation of 999 2110 loop
│   │   ├── table1.rs           # Table 1 definitions
│   │   └── table1trailer.rs     # Table 1 trailer definitions
│   ├── edi270/                  # Health Care Eligibility Benefit Inquiry implementation
│   ├── edi271/                  # Health Care Eligibility Benefit Response implementation
│   ├── edi276/                  # Health Care Claim Status Request implementation
│   ├── edi277/                  # Health Care Claim Status Response implementation
│   ├── edi837/                  # Health Care Claim implementation (Professional, Institutional, Dental)
│   ├── helper/                  # Utility functions and shared helpers
│   ├── segments/               # EDI segment definitions and processors
│   ├── error.rs                # Error handling module
│   ├── transaction_processor.rs # Generic transaction set processor
│   ├── segment_config.rs       # Configuration-driven segment definitions
│   ├── loop_processor.rs       # Enhanced loop detection and processing
│   ├── lib.rs                  # Library exports
│   └── main.rs                 # Application entry point
```

### Component Overview
- **Segment Parsers**: Convert EDI segment strings to structured data
- **Loop Handlers**: Process hierarchical loops within transaction sets
- **Controllers**: Coordinate the parsing and generation of transaction sets
- **Helper Functions**: Provide utility functions for common operations
- **Error Handling**: Standardized error types and handling mechanisms
- **Transaction Processor**: Generic processor for handling different transaction sets
- **Segment Configuration**: Configuration-driven approach for segment definitions
- **Loop Processor**: Enhanced detection and processing of loops

## 3. Development Plan

### Phase 1: Fix Existing Issues (COMPLETED)
- Fixed CTX segment implementation in 999 format
- Improved error handling for malformed input files
- Addressed Table 1 content placement issues in both 835 and 999 formats
- Implemented testing framework with unit and integration tests

### Phase 2: Implement Additional Transaction Sets (COMPLETED)
- Common Infrastructure Updates
  - Created a generic transaction set processor
  - Implemented a configuration-driven approach for segment definitions
  - Enhanced the loop detection and processing logic
  - Standardized error handling across all transaction sets

- Transaction Set: 270/271 (Health Care Eligibility Benefit Inquiry and Response)
  - Created segment definitions specific to 270/271 based on ASC X12N/005010X279
  - Implemented loop structures (2000A, 2000B, 2000C, 2000D, etc.)
  - Added validation for 270/271-specific requirements
  - Implemented bidirectional conversion (EDI to JSON and JSON to EDI)
  - Fixed REF, PER, and DTP segments handling

- Transaction Set: 276/277 (Health Care Claim Status Request and Response)
  - Created segment definitions specific to 276/277 based on ASC X12N/005010X212
  - Implemented loop structures (2000A, 2000B, 2000C, 2000D, etc.)
  - Added validation for 276/277-specific requirements
  - Implemented bidirectional conversion (EDI to JSON and JSON to EDI)
  - Enhanced loop processing for Loop2100A and Loop2100B
  - Added support for STC segments in EDI277

- Transaction Set: 837 (Health Care Claim)
  - Created segment definitions specific to 837P, 837I, and 837D based on:
    - ASC X12N/005010X222 (Professional)
    - ASC X12N/005010X223 (Institutional)
    - ASC X12N/005010X224 (Dental)
  - Implemented loop structures (2000A, 2000B, 2300, 2400, etc.)
  - Added validation for 837-specific requirements
  - Implemented bidirectional conversion (EDI to JSON and JSON to EDI)
  - Added specialized handling for variant-specific segments:
    - TOO segment in 837D (Dental)
    - CL1 segment in 837I (Institutional)
  - Improved format detection logic to better distinguish between variants

### Phase 3: Additional Transaction Sets (FUTURE)
- Transaction Set: 278 (Health Care Services Review) - ASC X12N/005010X217
  - Create segment definitions specific to 278
  - Implement loop structures for request and response
  - Add validation for 278-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)
  - Support specialized segments like UM (Health Care Services Review Information)
  - Handle complex hierarchical structure with multiple service levels

- Transaction Set: 820 (Payroll Deducted and Other Group Premium Payment) - ASC X12N/005010X218
  - Create segment definitions specific to 820
  - Implement loop structures for premium payment
  - Add validation for 820-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)
  - Support financial transaction segments like BPR and ADX
  - Handle both organization and individual premium remittance details

- Transaction Set: 834 (Benefit Enrollment and Maintenance) - ASC X12N/005010X220
  - Create segment definitions specific to 834
  - Implement loop structures for enrollment and maintenance
  - Add validation for 834-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)
  - Support member-level detail segments like INS, HD, and DSB
  - Handle complex member information structures and relationships

### Phase 4: Advanced Features and Optimizations (PLANNED)
- Performance Optimization
  - Profile the application to identify performance bottlenecks
  - Optimize memory usage for large EDI files
  - Implement parallel processing for batch operations
  - Add benchmarking tools for performance measurement

- Schema Validation
  - Implement JSON schema validation for output
  - Add EDI schema validation for input
  - Provide schema documentation for users

- Additional Features
  - Add support for custom delimiters
  - Implement pretty printing for JSON output
  - Add support for EDI file validation without conversion
  - Implement a command-line interface for advanced options

## 4. Implementation Approach

### Segment Implementation Pattern
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SegmentName {
    pub segment_id: String,
    pub element_01: String,
    pub element_02: Option<String>,
    // Additional elements as needed
}

impl SegmentName {
    pub fn new(segment: &str) -> Self {
        // Parse segment string and populate struct fields
    }
    
    pub fn to_edi(&self) -> String {
        // Convert struct to EDI segment string
    }
}
```

### Loop Implementation Pattern
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoopName {
    pub segment_1: Option<Segment1>,
    pub segment_2: Option<Segment2>,
    // Additional segments as needed
    pub child_loops: Vec<ChildLoop>,
}

pub fn get_loop_name(contents: String) -> (LoopName, String) {
    // Parse contents and extract loop
    // Return the loop and remaining contents
}

pub fn write_loop_name(loop_data: LoopName) -> String {
    // Convert loop to EDI string
}
```

### Transaction Set Implementation Pattern
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionSet {
    pub interchange_header: InterchangeHeader,
    pub functional_group: FunctionalGroup,
    pub loops: Vec<Loop>,
    pub interchange_trailer: InterchangeTrailer,
}

impl TransactionSet for TransactionSetName {
    fn parse(contents: String) -> EdiResult<(Self, String)> {
        // Parse contents and extract transaction set
        // Return the transaction set and any remaining content
    }
    
    fn generate(&self) -> EdiResult<String> {
        // Convert transaction set to EDI string
    }
}
```

#[[file:x12-transaction-structures.md]]