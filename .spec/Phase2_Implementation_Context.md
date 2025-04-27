# EDI Parser Phase 2 Implementation Context

## Project Overview

The EDI Parser is a Rust-based library for parsing and generating Electronic Data Interchange (EDI) files in healthcare X12 formats. The project aims to support multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837 (Claims).

## Phase 2 Goals

Phase 2 of the project focuses on:

1. Implementing common infrastructure updates to support multiple transaction sets
2. Implementing Transaction Set 270 (Health Care Eligibility Benefit Inquiry)
3. Implementing Transaction Set 271 (Health Care Eligibility Benefit Response)
4. Setting up the foundation for future transaction sets (276/277, 837)

## Technical Context

### EDI X12 Format

EDI X12 is a standard format for electronic data interchange used in healthcare. It consists of:

- **Interchange Control (ISA/IEA)**: The outermost envelope that contains one or more functional groups
- **Functional Group (GS/GE)**: Contains one or more transaction sets of the same type
- **Transaction Set (ST/SE)**: Contains the actual business data
- **Segments**: Individual data elements grouped by function (e.g., NM1 for name information)
- **Elements**: Individual data fields within segments

### Transaction Set 270/271

The 270/271 transaction set pair is used for eligibility, coverage, or benefit inquiry (270) and response (271):

- **270**: Sent by providers to payers to inquire about a patient's eligibility for healthcare services
- **271**: Sent by payers to providers in response to a 270 inquiry, containing eligibility information

### Loop Structure

Both 270 and 271 use a hierarchical loop structure:

- **Loop 2000A**: Information Source level
- **Loop 2000B**: Information Receiver level
- **Loop 2000C**: Subscriber level
- **Loop 2000D**: Dependent level

The 271 adds additional loops for benefit information:
- **Loop 2110C/D**: Eligibility or Benefit Information
- **Loop 2115C/D**: Eligibility or Benefit Additional Information
- **Loop 2120C/D**: Subscriber/Dependent Benefit Related Entity

## Implementation Approach

### Error Handling

We're using a Result-based approach with custom error types:
```rust
pub type EdiResult<T> = Result<T, EdiError>;

pub enum EdiError {
    ParseError(String),
    ValidationError(String),
    IoError(std::io::Error),
    MissingSegment(String),
    MalformedSegment(String),
    UnsupportedFormat(String),
}
```

### Segment Processing

Each segment type has its own module with:
- A struct representing the segment data
- A function to parse the segment from a string
- A function to generate the segment as a string

Example:
```rust
pub struct BHT {
    pub bht01_hierarchical_structure_code: String,
    pub bht02_transaction_set_purpose_code: String,
    // ...
}

pub fn get_bht(bht_content: String) -> BHT { /* ... */ }
pub fn write_bht(bht: BHT) -> String { /* ... */ }
```

### Loop Processing

Loops are organized hierarchically, with each loop potentially containing other loops:
```rust
pub struct Loop2000A {
    pub hl_segments: HL,
    pub nm1_segments: NM1,
    pub per_segments: Vec<PER>,
    // ...
}

pub fn get_loop_2000a(contents: String) -> EdiResult<(Loop2000A, String)> { /* ... */ }
pub fn write_loop_2000a(loop2000a: &Loop2000A) -> String { /* ... */ }
```

### Transaction Set Processing

Each transaction set has a controller module that orchestrates the parsing and generation:
```rust
pub struct Edi270 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    // ...
}

pub fn get_270(contents: String) -> EdiResult<(Edi270, String)> { /* ... */ }
pub fn write_270(edi270: &Edi270) -> String { /* ... */ }
```

## Current Status

As of now, we have:
- Implemented the common infrastructure components
- Completed the Transaction Set 270 implementation
- Completed the Transaction Set 271 implementation
- Set up the foundation for future transaction sets

## Next Steps

1. Create sample 271 EDI files for testing
2. Update the main application to support 271 processing
3. Begin implementation of Transaction Sets 276/277
4. Plan for Transaction Set 837 implementation
