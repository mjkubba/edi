# EDI to JSON Converter Specification

## Overview

This specification outlines the development plan for enhancing an existing open-source application that converts X12 EDI formats to JSON. The application currently supports 835 (Health Care Claim Payment/Advice) and 999 (Implementation Acknowledgment) formats, and this specification details the approach for fixing existing issues and implementing additional X12 formats.

## Current State Assessment

The application is built in Rust and follows a modular architecture with separate components for:
- Segment parsing and representation
- Loop structure handling
- Transaction set processing
- File I/O operations

Currently implemented transaction sets:
- 835 (Health Care Claim Payment/Advice)
- 999 (Implementation Acknowledgment)

### Existing Architecture

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
│   ├── helper/                  # Utility functions and shared helpers
│   ├── segments/               # EDI segment definitions and processors
│   └── main.rs                 # Application entry point
```

### Known Issues

1. CTX segment implementation in 999 format may not conform to standards
2. Error handling needs improvement for malformed input files
3. Table 1 content placement issues in both 835 and 999 formats
4. Missing unit and integration tests
5. Limited transaction set support (only 835 and 999)

## Development Plan

The development will proceed in phases, with each phase focusing on specific aspects of the application.

### Phase 1: Fix Existing Issues

#### 1.1 CTX Segment Implementation

**Issue:** The CTX segment implementation may not conform to the 999 standards.

**Solution:**
- Review the X12 999 implementation guide for CTX segment requirements
- Compare current implementation against standards
- Update the CTX segment implementation to match the standards
- Add validation for required and situational elements

#### 1.2 Error Handling Improvements

**Issue:** The application needs better error handling for malformed input files.

**Solution:**
- Implement robust error handling for file reading operations
- Add validation for EDI file structure before processing
- Provide meaningful error messages for common issues
- Implement graceful exit strategies for critical errors

#### 1.3 Table 1 Content Placement

**Issue:** Table 1 content placement issues in both 835 and 999 formats.

**Solution:**
- Review the structure of Table 1 in both 835 and 999 formats
- Ensure proper nesting of Table 1 content in the JSON output
- Fix any inconsistencies in the Table 1 implementation
- Add validation for Table 1 structure

#### 1.4 Testing Framework

**Issue:** Missing unit and integration tests.

**Solution:**
- Implement unit tests for segment parsing
- Add integration tests for end-to-end processing
- Create test fixtures for various EDI formats
- Implement test coverage reporting

### Phase 2: Implement Additional Transaction Sets

#### 2.1 Common Infrastructure Updates

Before implementing new transaction sets, update the common infrastructure:

- Create a generic transaction set processor
- Implement a configuration-driven approach for segment definitions
- Enhance the loop detection and processing logic
- Standardize error handling across all transaction sets

#### 2.2 Transaction Set: 270/271 (Health Care Eligibility Benefit Inquiry and Response)

Implement support for the 270/271 transaction set:

- Create segment definitions specific to 270/271
- Implement loop structures (2000A, 2000B, 2000C, 2000D, etc.)
- Add validation for 270/271-specific requirements
- Implement bidirectional conversion (EDI to JSON and JSON to EDI)

#### 2.3 Transaction Set: 276/277 (Health Care Claim Status Request and Response)

Implement support for the 276/277 transaction set:

- Create segment definitions specific to 276/277
- Implement loop structures (2000A, 2000B, 2000C, 2000D, etc.)
- Add validation for 276/277-specific requirements
- Implement bidirectional conversion (EDI to JSON and JSON to EDI)

#### 2.4 Transaction Set: 837 (Health Care Claim)

Implement support for the 837 transaction set (Professional, Institutional, and Dental):

- Create segment definitions specific to 837P, 837I, and 837D
- Implement loop structures (2000A, 2000B, 2300, 2400, etc.)
- Add validation for 837-specific requirements
- Implement bidirectional conversion (EDI to JSON and JSON to EDI)

### Phase 3: Advanced Features and Optimizations

#### 3.1 Performance Optimization

- Profile the application to identify performance bottlenecks
- Optimize memory usage for large EDI files
- Implement parallel processing for batch operations
- Add benchmarking tools for performance measurement

#### 3.2 Schema Validation

- Implement JSON schema validation for output
- Add EDI schema validation for input
- Provide schema documentation for users

#### 3.3 Additional Features

- Add support for custom delimiters
- Implement pretty printing for JSON output
- Add support for EDI file validation without conversion
- Implement a command-line interface for advanced options

## Implementation Details

### Segment Implementation Approach

Each segment should follow this implementation pattern:

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

### Loop Implementation Approach

Each loop should follow this implementation pattern:

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

### Transaction Set Implementation Approach

Each transaction set should follow this implementation pattern:

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

## Questions for Clarification

1. Are there specific EDI transaction sets that should be prioritized beyond the ones mentioned in this specification?
2. Are there any specific validation rules or business logic that should be implemented for each transaction set?
3. Should the application support any specific EDI dialects or variations?
4. Are there any performance requirements or constraints for processing large EDI files?
5. Should the application support any specific output formats beyond JSON (e.g., XML, CSV)?
6. Are there any specific error handling or logging requirements?
7. Should the application support any specific character encodings beyond UTF-8?
8. Are there any specific security considerations for handling sensitive healthcare data?

## Assumptions

1. The application will continue to be developed in Rust.
2. The application will maintain backward compatibility with existing functionality.
3. The application will follow the same architectural pattern for new transaction sets.
4. The application will support both reading (EDI to JSON) and writing (JSON to EDI) operations.
5. The application will validate input and output according to X12 standards.
6. The application will handle common EDI delimiters (segment, element, component, repetition).
7. The application will support the X12 5010 version of the transaction sets.

## Next Steps

1. Review and approve this specification
2. Prioritize the issues and features
3. Begin implementation of Phase 1
4. Conduct regular reviews and updates to the specification as needed

## Appendix A: X12 Transaction Set Structure

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

## Appendix B: Segment Definitions

This section will be expanded with detailed segment definitions for each transaction set as implementation progresses.
