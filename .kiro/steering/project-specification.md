---
inclusion: fileMatch
fileMatchPattern: 'README*'
---

# EDI Parser Project Specification

## Project Purpose and Scope
- Convert EDI formats to JSON and vice versa
- Support multiple healthcare X12 transaction sets (835, 999, 270/271, 276/277, 837P/I/D)
- Provide robust error handling and validation
- Maintain a modular and extensible architecture
- Support future implementation of additional transaction sets (278, 820, 834)

## Architecture Overview

### Component Overview
- **Segment Parsers**: Convert EDI segment strings to structured data
- **Loop Handlers**: Process hierarchical loops within transaction sets
- **Controllers**: Coordinate the parsing and generation of transaction sets
- **Helper Functions**: Provide utility functions for common operations
- **Error Handling**: Standardized error types and handling mechanisms
- **Transaction Processor**: Generic processor for handling different transaction sets
- **Segment Configuration**: Configuration-driven approach for segment definitions
- **Loop Processor**: Enhanced detection and processing of loops

## Implementation Patterns

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

## Supported Transaction Sets

### Currently Implemented
- **835** (Health Care Claim Payment/Advice) - ASC X12N/005010X221
- **999** (Implementation Acknowledgment) - ASC X12C/005010X231
- **270/271** (Health Care Eligibility Benefit Inquiry and Response) - ASC X12N/005010X279
- **276/277** (Health Care Claim Status Request and Response) - ASC X12N/005010X212
- **837P** (Health Care Claim: Professional) - ASC X12N/005010X222
- **837I** (Health Care Claim: Institutional) - ASC X12N/005010X223
- **837D** (Health Care Claim: Dental) - ASC X12N/005010X224
- **278** (Health Care Services Review) - ASC X12N/005010X217
- **820** (Payroll Deducted and Other Group Premium Payment) - ASC X12N/005010X218

### Planned for Implementation
- **834** (Benefit Enrollment and Maintenance) - ASC X12N/005010X220