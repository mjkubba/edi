# EDI Parser Project Specification and Overview

## 1. Introduction

This document outlines the development plan for enhancing an existing open-source application that converts X12 EDI formats to JSON. The application currently supports 835 (Health Care Claim Payment/Advice) and 999 (Implementation Acknowledgment) formats, and this specification details the approach for fixing existing issues and implementing additional X12 formats.

### Project Purpose and Scope
- Convert EDI formats to JSON and vice versa
- Support multiple healthcare X12 transaction sets
- Provide robust error handling and validation
- Maintain a modular and extensible architecture

### Current State Assessment
The application is built in Rust and follows a modular architecture with separate components for:
- Segment parsing and representation
- Loop structure handling
- Transaction set processing
- File I/O operations

Currently implemented transaction sets:
- 835 (Health Care Claim Payment/Advice)
- 999 (Implementation Acknowledgment)

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
│   ├── edi999/                  # Functional Acknowledgment format implementation
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

## 3. Development Plan

### Phase 1: Fix Existing Issues (COMPLETED)
- Fixed CTX segment implementation in 999 format
- Improved error handling for malformed input files
- Addressed Table 1 content placement issues in both 835 and 999 formats
- Implemented testing framework with unit and integration tests

### Phase 2: Implement Additional Transaction Sets (IN PROGRESS)
- Common Infrastructure Updates
  - Created a generic transaction set processor
  - Implemented a configuration-driven approach for segment definitions
  - Enhanced the loop detection and processing logic
  - Standardized error handling across all transaction sets

- Transaction Set: 270/271 (Health Care Eligibility Benefit Inquiry and Response)
  - Created segment definitions specific to 270/271
  - Implemented loop structures (2000A, 2000B, 2000C, 2000D, etc.)
  - Added validation for 270/271-specific requirements
  - Implemented bidirectional conversion (EDI to JSON and JSON to EDI)

- Transaction Set: 276/277 (Health Care Claim Status Request and Response)
  - Create segment definitions specific to 276/277
  - Implement loop structures (2000A, 2000B, 2000C, 2000D, etc.)
  - Add validation for 276/277-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)

- Transaction Set: 837 (Health Care Claim)
  - Create segment definitions specific to 837P, 837I, and 837D
  - Implement loop structures (2000A, 2000B, 2300, 2400, etc.)
  - Add validation for 837-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)

### Phase 3: Advanced Features and Optimizations (PLANNED)
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

pub fn get_transaction_set(contents: String) -> TransactionSet {
    // Parse contents and extract transaction set
}

pub fn write_transaction_set(transaction_set: TransactionSet) -> String {
    // Convert transaction set to EDI string
}
```

## 5. X12 Transaction Set Structures

### 835 (Health Care Claim Payment/Advice)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BPR - Financial Information
      TRN - Reassociation Trace Number
      DTM - Production Date
      Loop 1000A - Payer Identification
        N1 - Payer Identification
        N3 - Payer Address
        N4 - Payer City/State/ZIP
        REF - Additional Payer Identification
        PER - Payer Contact Information
      Loop 1000B - Payee Identification
        N1 - Payee Identification
        N3 - Payee Address
        N4 - Payee City/State/ZIP
        REF - Payee Additional Identification
        RDM - Remittance Delivery Method
      Loop 2000 - Header Number
        LX - Header Number
        TS3 - Provider Summary Information
        TS2 - Provider Supplemental Summary Information
        Loop 2100 - Claim Payment Information
          CLP - Claim Payment Information
          CAS - Claim Adjustment
          NM1 - Patient Name
          NM1 - Insured Name
          NM1 - Corrected Patient/Insured Name
          NM1 - Service Provider Name
          MIA - Inpatient Adjudication Information
          MOA - Outpatient Adjudication Information
          REF - Other Claim Related Identification
          DTM - Statement From or To Date
          PER - Claim Contact Information
          AMT - Claim Supplemental Information
          QTY - Claim Supplemental Information Quantity
          Loop 2110 - Service Payment Information
            SVC - Service Payment Information
            DTM - Service Date
            CAS - Service Adjustment
            REF - Service Identification
            AMT - Service Supplemental Amount
            QTY - Service Supplemental Quantity
            LQ - Health Care Remark Codes
      PLB - Provider Adjustment
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

### 999 (Implementation Acknowledgment)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      AK1 - Functional Group Response Header
      Loop 2000 - Transaction Set Response Header
        AK2 - Transaction Set Response Header
        Loop 2100 - Error Identification
          IK3 - Error Identification
          Loop 2110 - Implementation Data Element Note
            IK4 - Implementation Data Element Note
            CTX - Context
        IK5 - Transaction Set Response Trailer
      AK9 - Functional Group Response Trailer
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

### 270/271 (Health Care Eligibility Benefit Inquiry and Response)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Information Source
        HL - Information Source Level
        NM1 - Information Source Name
        PER - Information Source Contact Information
        Loop 2100A - Payer Name
          NM1 - Payer Name
          Loop 2000B - Information Receiver
            HL - Information Receiver Level
            NM1 - Information Receiver Name
            Loop 2100B - Information Receiver Name
              NM1 - Information Receiver Name
              Loop 2000C - Subscriber
                HL - Subscriber Level
                TRN - Subscriber Trace Number
                Loop 2100C - Subscriber Name
                  NM1 - Subscriber Name
                  REF - Subscriber Additional Identification
                  N3 - Subscriber Address
                  N4 - Subscriber City/State/ZIP
                  DMG - Subscriber Demographic Information
                  INS - Subscriber Relationship
                  DTP - Subscriber Date
                  Loop 2110C - Subscriber Eligibility or Benefit Information
                    EB - Subscriber Eligibility or Benefit Information
                    HSD - Health Care Services Delivery
                    REF - Subscriber Additional Identification
                    DTP - Subscriber Eligibility/Benefit Date
                    AAA - Subscriber Request Validation
                    MSG - Message Text
                    Loop 2115C - Subscriber Eligibility or Benefit Additional Information
                      III - Subscriber Eligibility or Benefit Additional Information
                      Loop 2120C - Subscriber Benefit Related Entity
                        NM1 - Subscriber Benefit Related Entity Name
                        N3 - Subscriber Benefit Related Entity Address
                        N4 - Subscriber Benefit Related Entity City/State/ZIP
                        PER - Subscriber Benefit Related Entity Contact Information
                        PRV - Subscriber Benefit Related Provider Information
                Loop 2000D - Dependent
                  HL - Dependent Level
                  TRN - Dependent Trace Number
                  Loop 2100D - Dependent Name
                    NM1 - Dependent Name
                    REF - Dependent Additional Identification
                    N3 - Dependent Address
                    N4 - Dependent City/State/ZIP
                    DMG - Dependent Demographic Information
                    INS - Dependent Relationship
                    DTP - Dependent Date
                    Loop 2110D - Dependent Eligibility or Benefit Information
                      EB - Dependent Eligibility or Benefit Information
                      HSD - Health Care Services Delivery
                      REF - Dependent Additional Identification
                      DTP - Dependent Eligibility/Benefit Date
                      AAA - Dependent Request Validation
                      MSG - Message Text
                      Loop 2115D - Dependent Eligibility or Benefit Additional Information
                        III - Dependent Eligibility or Benefit Additional Information
                        Loop 2120D - Dependent Benefit Related Entity
                          NM1 - Dependent Benefit Related Entity Name
                          N3 - Dependent Benefit Related Entity Address
                          N4 - Dependent Benefit Related Entity City/State/ZIP
                          PER - Dependent Benefit Related Entity Contact Information
                          PRV - Dependent Benefit Related Provider Information
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 6. Assumptions and Constraints

### Technical Assumptions
1. The application will continue to be developed in Rust.
2. The application will maintain backward compatibility with existing functionality.
3. The application will follow the same architectural pattern for new transaction sets.
4. The application will support both reading (EDI to JSON) and writing (JSON to EDI) operations.
5. The application will validate input and output according to X12 standards.
6. The application will handle common EDI delimiters (segment, element, component, repetition).
7. The application will support the X12 5010 version of the transaction sets.

### Compatibility Requirements
- Maintain compatibility with existing 835 and 999 implementations
- Ensure consistent API across all transaction sets
- Support standard EDI delimiters and formats

### Performance Considerations
- Handle large EDI files efficiently
- Minimize memory usage during parsing and generation
- Provide reasonable performance for batch processing
