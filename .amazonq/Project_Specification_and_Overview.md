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

### Phase 4: Additional Transaction Sets (FUTURE)
- Transaction Set: 278 (Health Care Services Review) - ASC X12N/005010X217
  - Create segment definitions specific to 278
  - Implement loop structures for request and response
  - Add validation for 278-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)

- Transaction Set: 820 (Payroll Deducted and Other Group Premium Payment) - ASC X12N/005010X218
  - Create segment definitions specific to 820
  - Implement loop structures for premium payment
  - Add validation for 820-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)

- Transaction Set: 834 (Benefit Enrollment and Maintenance) - ASC X12N/005010X220
  - Create segment definitions specific to 834
  - Implement loop structures for enrollment and maintenance
  - Add validation for 834-specific requirements
  - Implement bidirectional conversion (EDI to JSON and JSON to EDI)

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

## 5. X12 Transaction Set Structures

### 835 (Health Care Claim Payment/Advice) - ASC X12N/005010X221
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

### 999 (Implementation Acknowledgment) - ASC X12C/005010X231
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      AK1 - Functional Group Response Header
      Loop 2000 - Transaction Set Response Header
        AK2 - Transaction Set Response Header
        Loop 2100 - Error Identification
          IK3 - Error Identification
          CTX - Context
          Loop 2110 - Implementation Data Element Note
            IK4 - Implementation Data Element Note
            CTX - Context
        IK5 - Transaction Set Response Trailer
      AK9 - Functional Group Response Trailer
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

### 270/271 (Health Care Eligibility Benefit Inquiry and Response) - ASC X12N/005010X279
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

### 276/277 (Health Care Claim Status Request and Response) - ASC X12N/005010X212
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Information Source
        HL - Information Source Level
        NM1 - Information Source Name
        Loop 2000B - Information Receiver
          HL - Information Receiver Level
          NM1 - Information Receiver Name
          Loop 2000C - Service Provider
            HL - Service Provider Level
            NM1 - Service Provider Name
            Loop 2000D - Subscriber
              HL - Subscriber Level
              NM1 - Subscriber Name
              DMG - Subscriber Demographic Information
              Loop 2000E - Dependent
                HL - Dependent Level
                NM1 - Dependent Name
                DMG - Dependent Demographic Information
                Loop 2200D - Claim Status Tracking Number
                  TRN - Claim Status Tracking Number
                  STC - Claim Status Information
                  REF - Payer Claim Identification Number
                  DTP - Claim Service Date
                  Loop 2220D - Service Line Information
                    SVC - Service Line Information
                    STC - Service Line Status Information
                    REF - Service Line Item Identification
                    DTP - Service Line Date
              Loop 2200C - Claim Status Tracking Number
                TRN - Claim Status Tracking Number
                STC - Claim Status Information
                REF - Payer Claim Identification Number
                DTP - Claim Service Date
                Loop 2220C - Service Line Information
                  SVC - Service Line Information
                  STC - Service Line Status Information
                  REF - Service Line Item Identification
                  DTP - Service Line Date
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
### 837I (Health Care Claim: Institutional) - ASC X12N/005010X223
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Billing Provider Hierarchical Level
        HL - Billing Provider Hierarchical Level
        PRV - Billing Provider Specialty Information
        CUR - Foreign Currency Information
        Loop 2010AA - Billing Provider Name
          NM1 - Billing Provider Name
          N3 - Billing Provider Address
          N4 - Billing Provider City/State/ZIP
          REF - Billing Provider Tax Identification
          PER - Billing Provider Contact Information
        Loop 2010AB - Pay-to Address
          NM1 - Pay-to Provider Name
          N3 - Pay-to Provider Address
          N4 - Pay-to Provider City/State/ZIP
        Loop 2010AC - Pay-to Plan Name
          NM1 - Pay-to Plan Name
          N3 - Pay-to Plan Address
          N4 - Pay-to Plan City/State/ZIP
          REF - Pay-to Plan Tax Identification
        Loop 2000B - Subscriber Hierarchical Level
          HL - Subscriber Hierarchical Level
          SBR - Subscriber Information
          PAT - Patient Information
          Loop 2010BA - Subscriber Name
            NM1 - Subscriber Name
            N3 - Subscriber Address
            N4 - Subscriber City/State/ZIP
            DMG - Subscriber Demographic Information
            REF - Subscriber Secondary Identification
          Loop 2010BB - Payer Name
            NM1 - Payer Name
            N3 - Payer Address
            N4 - Payer City/State/ZIP
            REF - Payer Secondary Identification
          Loop 2300 - Claim Information
            CLM - Claim Information
            DTP - Date - Statement From or Through
            DTP - Date - Admission
            DTP - Date - Discharge
            CL1 - Institutional Claim Code (Specific to 837I)
            PWK - Claim Supplemental Information
            CN1 - Contract Information
            AMT - Patient Amount Paid
            REF - Medical Record Number
            REF - Prior Authorization
            REF - Claim Identification Number For Clearinghouses and Other Transmission Intermediaries
            K3 - File Information
            NTE - Claim Note
            HI - Health Care Diagnosis Code
            HI - Anesthesia Related Procedure
            HI - Condition Information
            HI - Occurrence Information
            HI - Occurrence Span Information
            HI - Value Information
            HI - Treatment Code Information
            HCP - Claim Pricing/Repricing Information
            Loop 2310A - Attending Provider Name
              NM1 - Attending Provider Name
              PRV - Attending Provider Specialty Information
              REF - Attending Provider Secondary Identification
            Loop 2310B - Operating Physician Name
              NM1 - Operating Physician Name
              PRV - Operating Physician Specialty Information
              REF - Operating Physician Secondary Identification
            Loop 2310C - Other Operating Physician Name
              NM1 - Other Operating Physician Name
              PRV - Other Operating Physician Specialty Information
              REF - Other Operating Physician Secondary Identification
            Loop 2310D - Rendering Provider Name
              NM1 - Rendering Provider Name
              PRV - Rendering Provider Specialty Information
              REF - Rendering Provider Secondary Identification
            Loop 2310E - Service Facility Location
              NM1 - Service Facility Location
              N3 - Service Facility Location Address
              N4 - Service Facility Location City/State/ZIP
              REF - Service Facility Location Secondary Identification
            Loop 2310F - Referring Provider Name
              NM1 - Referring Provider Name
              PRV - Referring Provider Specialty Information
              REF - Referring Provider Secondary Identification
            Loop 2320 - Other Subscriber Information
              SBR - Other Subscriber Information
              CAS - Claim Level Adjustments
              AMT - Coordination of Benefits (COB) Payer Paid Amount
              AMT - Coordination of Benefits (COB) Total Non-Covered Amount
              AMT - Remaining Patient Liability
              OI - Other Insurance Coverage Information
              MIA - Inpatient Adjudication Information
              MOA - Outpatient Adjudication Information
              Loop 2330A - Other Subscriber Name
                NM1 - Other Subscriber Name
                N3 - Other Subscriber Address
                N4 - Other Subscriber City/State/ZIP
                REF - Other Subscriber Secondary Identification
              Loop 2330B - Other Payer Name
                NM1 - Other Payer Name
                N3 - Other Payer Address
                N4 - Other Payer City/State/ZIP
                DTP - Claim Check or Remittance Date
                REF - Other Payer Secondary Identifier
              Loop 2330C - Other Payer Attending Provider
                NM1 - Other Payer Attending Provider
                REF - Other Payer Attending Provider Secondary Identification
              Loop 2330D - Other Payer Operating Physician
                NM1 - Other Payer Operating Physician
                REF - Other Payer Operating Physician Secondary Identification
              Loop 2330E - Other Payer Other Operating Physician
                NM1 - Other Payer Other Operating Physician
                REF - Other Payer Other Operating Physician Secondary Identification
              Loop 2330F - Other Payer Service Facility Location
                NM1 - Other Payer Service Facility Location
                REF - Other Payer Service Facility Location Secondary Identification
              Loop 2330G - Other Payer Rendering Provider
                NM1 - Other Payer Rendering Provider
                REF - Other Payer Rendering Provider Secondary Identification
              Loop 2330H - Other Payer Referring Provider
                NM1 - Other Payer Referring Provider
                REF - Other Payer Referring Provider Secondary Identification
              Loop 2330I - Other Payer Billing Provider
                NM1 - Other Payer Billing Provider
                REF - Other Payer Billing Provider Secondary Identification
            Loop 2400 - Service Line
              LX - Service Line
              SV2 - Institutional Service
              PWK - Line Supplemental Information
              DTP - Date - Service Date
              REF - Line Item Control Number
              AMT - Sales Tax Amount
              NTE - Line Note
              HCP - Line Pricing/Repricing Information
              Loop 2410 - Drug Identification
                LIN - Drug Identification
                CTP - Drug Pricing
                REF - Prescription or Compound Drug Association Number
              Loop 2420A - Attending Provider Name
                NM1 - Attending Provider Name
                PRV - Attending Provider Specialty Information
                REF - Attending Provider Secondary Identification
              Loop 2420B - Operating Physician Name
                NM1 - Operating Physician Name
                PRV - Operating Physician Specialty Information
                REF - Operating Physician Secondary Identification
              Loop 2420C - Other Operating Physician Name
                NM1 - Other Operating Physician Name
                PRV - Other Operating Physician Specialty Information
                REF - Other Operating Physician Secondary Identification
              Loop 2420D - Rendering Provider Name
                NM1 - Rendering Provider Name
                PRV - Rendering Provider Specialty Information
                REF - Rendering Provider Secondary Identification
              Loop 2430 - Line Adjudication Information
                SVD - Line Adjudication
                CAS - Line Adjustment
                DTP - Line Check or Remittance Date
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```
### 837D (Health Care Claim: Dental) - ASC X12N/005010X224
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Billing Provider Hierarchical Level
        HL - Billing Provider Hierarchical Level
        PRV - Billing Provider Specialty Information
        CUR - Foreign Currency Information
        Loop 2010AA - Billing Provider Name
          NM1 - Billing Provider Name
          N3 - Billing Provider Address
          N4 - Billing Provider City/State/ZIP
          REF - Billing Provider Tax Identification
          PER - Billing Provider Contact Information
        Loop 2010AB - Pay-to Address
          NM1 - Pay-to Provider Name
          N3 - Pay-to Provider Address
          N4 - Pay-to Provider City/State/ZIP
        Loop 2010AC - Pay-to Plan Name
          NM1 - Pay-to Plan Name
          N3 - Pay-to Plan Address
          N4 - Pay-to Plan City/State/ZIP
          REF - Pay-to Plan Tax Identification
        Loop 2000B - Subscriber Hierarchical Level
          HL - Subscriber Hierarchical Level
          SBR - Subscriber Information
          PAT - Patient Information
          Loop 2010BA - Subscriber Name
            NM1 - Subscriber Name
            N3 - Subscriber Address
            N4 - Subscriber City/State/ZIP
            DMG - Subscriber Demographic Information
            REF - Subscriber Secondary Identification
          Loop 2010BB - Payer Name
            NM1 - Payer Name
            N3 - Payer Address
            N4 - Payer City/State/ZIP
            REF - Payer Secondary Identification
          Loop 2300 - Claim Information
            CLM - Claim Information
            DTP - Date - Initial Treatment Date
            DTP - Date - Last Seen Date
            DTP - Date - Acute Manifestation
            DTP - Date - Accident
            DTP - Date - Appliance Placement
            DTP - Date - Assumption of Care
            DTP - Date - Last X-ray
            DTP - Date - Placement
            DTP - Date - Replacement
            DN1 - Orthodontic Total Months of Treatment
            DN2 - Tooth Status
            PWK - Claim Supplemental Information
            AMT - Patient Amount Paid
            REF - Predetermination Identification
            REF - Prior Authorization
            REF - Referral Number
            REF - Claim Identification Number For Clearinghouses and Other Transmission Intermediaries
            NTE - Claim Note
            HI - Diagnosis Code
            HCP - Claim Pricing/Repricing Information
            Loop 2310A - Referring Provider Name
              NM1 - Referring Provider Name
              PRV - Referring Provider Specialty Information
              REF - Referring Provider Secondary Identification
            Loop 2310B - Rendering Provider Name
              NM1 - Rendering Provider Name
              PRV - Rendering Provider Specialty Information
              REF - Rendering Provider Secondary Identification
            Loop 2310C - Service Facility Location
              NM1 - Service Facility Location
              N3 - Service Facility Location Address
              N4 - Service Facility Location City/State/ZIP
              REF - Service Facility Location Secondary Identification
            Loop 2310D - Assistant Surgeon Name
              NM1 - Assistant Surgeon Name
              PRV - Assistant Surgeon Specialty Information
              REF - Assistant Surgeon Secondary Identification
            Loop 2320 - Other Subscriber Information
              SBR - Other Subscriber Information
              CAS - Claim Level Adjustments
              AMT - Coordination of Benefits (COB) Payer Paid Amount
              AMT - Coordination of Benefits (COB) Total Non-Covered Amount
              AMT - Remaining Patient Liability
              OI - Other Insurance Coverage Information
              MOA - Outpatient Adjudication Information
              Loop 2330A - Other Subscriber Name
                NM1 - Other Subscriber Name
                N3 - Other Subscriber Address
                N4 - Other Subscriber City/State/ZIP
                REF - Other Subscriber Secondary Identification
              Loop 2330B - Other Payer Name
                NM1 - Other Payer Name
                N3 - Other Payer Address
                N4 - Other Payer City/State/ZIP
                DTP - Claim Check or Remittance Date
                REF - Other Payer Secondary Identifier
              Loop 2330C - Other Payer Referring Provider
                NM1 - Other Payer Referring Provider
                REF - Other Payer Referring Provider Secondary Identification
              Loop 2330D - Other Payer Rendering Provider
                NM1 - Other Payer Rendering Provider
                REF - Other Payer Rendering Provider Secondary Identification
              Loop 2330E - Other Payer Service Facility Location
                NM1 - Other Payer Service Facility Location
                REF - Other Payer Service Facility Location Secondary Identification
              Loop 2330F - Other Payer Assistant Surgeon
                NM1 - Other Payer Assistant Surgeon
                REF - Other Payer Assistant Surgeon Secondary Identification
              Loop 2330G - Other Payer Billing Provider
                NM1 - Other Payer Billing Provider
                REF - Other Payer Billing Provider Secondary Identification
            Loop 2400 - Service Line
              LX - Service Line
              SV3 - Dental Service
              TOO - Tooth Information (Specific to 837D)
              DTP - Date - Service
              DTP - Date - Prior Placement
              DTP - Date - Replacement
              DTP - Date - Appliance Placement
              QTY - Anesthesia Quantity
              REF - Line Item Control Number
              AMT - Sales Tax Amount
              NTE - Line Note
              HCP - Line Pricing/Repricing Information
              Loop 2410 - Drug Identification
                LIN - Drug Identification
                CTP - Drug Pricing
                REF - Prescription or Compound Drug Association Number
              Loop 2420A - Rendering Provider Name
                NM1 - Rendering Provider Name
                PRV - Rendering Provider Specialty Information
                REF - Rendering Provider Secondary Identification
              Loop 2420B - Assistant Surgeon Name
                NM1 - Assistant Surgeon Name
                PRV - Assistant Surgeon Specialty Information
                REF - Assistant Surgeon Secondary Identification
              Loop 2420C - Service Facility Location
                NM1 - Service Facility Location
                N3 - Service Facility Location Address
                N4 - Service Facility Location City/State/ZIP
                REF - Service Facility Location Secondary Identification
              Loop 2420D - Referring Provider Name
                NM1 - Referring Provider Name
                PRV - Referring Provider Specialty Information
                REF - Referring Provider Secondary Identification
              Loop 2430 - Line Adjudication Information
                SVD - Line Adjudication
                CAS - Line Adjustment
                DTP - Line Check or Remittance Date
              Loop 2440 - Form Identification Code
                LQ - Form Identification Code
                FRM - Supporting Documentation
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```
### 278 (Health Care Services Review) - ASC X12N/005010X217 (FUTURE)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Utilization Management Organization (UMO) Level
        HL - UMO Level
        Loop 2010A - UMO Name
          NM1 - UMO Name
          PER - UMO Contact Information
        Loop 2000B - Requester Level
          HL - Requester Level
          Loop 2010B - Requester Name
            NM1 - Requester Name
            REF - Requester Supplemental Identification
            PER - Requester Contact Information
          Loop 2000C - Subscriber Level
            HL - Subscriber Level
            Loop 2010C - Subscriber Name
              NM1 - Subscriber Name
              REF - Subscriber Supplemental Identification
              N3 - Subscriber Address
              N4 - Subscriber City/State/ZIP
              DMG - Subscriber Demographic Information
              INS - Subscriber Relationship
              Loop 2000D - Dependent Level
                HL - Dependent Level
                Loop 2010D - Dependent Name
                  NM1 - Dependent Name
                  REF - Dependent Supplemental Identification
                  N3 - Dependent Address
                  N4 - Dependent City/State/ZIP
                  DMG - Dependent Demographic Information
                Loop 2000E - Service Level
                  HL - Service Level
                  TRN - Service Trace Number
                  UM - Health Care Services Review Information
                  REF - Previous Review Authorization Number
                  DTP - Service Date
                  HI - Service Diagnosis
                  HSD - Health Care Services Delivery
                  CL1 - Institutional Claim Code
                  CR1 - Ambulance Transport Information
                  CR2 - Spinal Manipulation Service Information
                  CR5 - Home Oxygen Therapy Information
                  CR6 - Home Health Care Information
                  PWK - Additional Service Information
                  MSG - Message Text
                  Loop 2010EA - Patient Event Provider Name
                    NM1 - Patient Event Provider Name
                    REF - Patient Event Provider Supplemental Identification
                    N3 - Patient Event Provider Address
                    N4 - Patient Event Provider City/State/ZIP
                    PER - Patient Event Provider Contact Information
                    PRV - Patient Event Provider Information
                  Loop 2010EB - Service Provider Name
                    NM1 - Service Provider Name
                    REF - Service Provider Supplemental Identification
                    N3 - Service Provider Address
                    N4 - Service Provider City/State/ZIP
                    PER - Service Provider Contact Information
                    PRV - Service Provider Information
                  Loop 2010EC - Service Facility Name
                    NM1 - Service Facility Name
                    REF - Service Facility Supplemental Identification
                    N3 - Service Facility Address
                    N4 - Service Facility City/State/ZIP
                    PER - Service Facility Contact Information
                  Loop 2000F - Service Line
                    HL - Service Line Level
                    TRN - Service Line Trace Number
                    UM - Health Care Services Review Information
                    REF - Previous Review Authorization Number
                    DTP - Service Date
                    SV1 - Professional Service
                    SV2 - Institutional Service
                    SV3 - Dental Service
                    TOO - Tooth Information
                    HI - Service Diagnosis
                    HSD - Health Care Services Delivery
                    PWK - Additional Service Information
                    MSG - Message Text
                    Loop 2010F - Service Provider Name
                      NM1 - Service Provider Name
                      REF - Service Provider Supplemental Identification
                      N3 - Service Provider Address
                      N4 - Service Provider City/State/ZIP
                      PER - Service Provider Contact Information
                      PRV - Service Provider Information
              Loop 2000E - Service Level
                HL - Service Level
                TRN - Service Trace Number
                UM - Health Care Services Review Information
                REF - Previous Review Authorization Number
                DTP - Service Date
                HI - Service Diagnosis
                HSD - Health Care Services Delivery
                CL1 - Institutional Claim Code
                CR1 - Ambulance Transport Information
                CR2 - Spinal Manipulation Service Information
                CR5 - Home Oxygen Therapy Information
                CR6 - Home Health Care Information
                PWK - Additional Service Information
                MSG - Message Text
                Loop 2010EA - Patient Event Provider Name
                  NM1 - Patient Event Provider Name
                  REF - Patient Event Provider Supplemental Identification
                  N3 - Patient Event Provider Address
                  N4 - Patient Event Provider City/State/ZIP
                  PER - Patient Event Provider Contact Information
                  PRV - Patient Event Provider Information
                Loop 2010EB - Service Provider Name
                  NM1 - Service Provider Name
                  REF - Service Provider Supplemental Identification
                  N3 - Service Provider Address
                  N4 - Service Provider City/State/ZIP
                  PER - Service Provider Contact Information
                  PRV - Service Provider Information
                Loop 2010EC - Service Facility Name
                  NM1 - Service Facility Name
                  REF - Service Facility Supplemental Identification
                  N3 - Service Facility Address
                  N4 - Service Facility City/State/ZIP
                  PER - Service Facility Contact Information
                Loop 2000F - Service Line
                  HL - Service Line Level
                  TRN - Service Line Trace Number
                  UM - Health Care Services Review Information
                  REF - Previous Review Authorization Number
                  DTP - Service Date
                  SV1 - Professional Service
                  SV2 - Institutional Service
                  SV3 - Dental Service
                  TOO - Tooth Information
                  HI - Service Diagnosis
                  HSD - Health Care Services Delivery
                  PWK - Additional Service Information
                  MSG - Message Text
                  Loop 2010F - Service Provider Name
                    NM1 - Service Provider Name
                    REF - Service Provider Supplemental Identification
                    N3 - Service Provider Address
                    N4 - Service Provider City/State/ZIP
                    PER - Service Provider Contact Information
                    PRV - Service Provider Information
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```
### 820 (Payroll Deducted and Other Group Premium Payment) - ASC X12N/005010X218 (FUTURE)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BPR - Beginning Segment for Payment Order/Remittance Advice
      TRN - Trace
      CUR - Currency
      REF - Reference Identification
      DTM - Date/Time Reference
      Loop 1000A - Premium Receiver's Name
        N1 - Premium Receiver's Name
        N2 - Premium Receiver Additional Name
        N3 - Premium Receiver Address
        N4 - Premium Receiver City/State/ZIP
        RDM - Remittance Delivery Method
        DTM - Premium Receiver Date
      Loop 1000B - Premium Payer's Name
        N1 - Premium Payer's Name
        N2 - Premium Payer Additional Name
        N3 - Premium Payer Address
        N4 - Premium Payer City/State/ZIP
        PER - Premium Payer Administrative Contact
        DTM - Premium Payer Date
      Loop 1000C - Intermediary Bank Information
        N1 - Intermediary Bank Name
        N3 - Intermediary Bank Address
        N4 - Intermediary Bank City/State/ZIP
        PER - Intermediary Bank Contact
        RDM - Intermediary Bank Remittance Delivery Method
      Loop 2000A - Organization Summary Remittance
        ENT - Organization Summary Remittance
        Loop 2100A - Organization Summary Remittance Detail
          RMR - Organization Summary Remittance Detail
          DTM - Organization Summary Remittance Date
          REF - Organization Summary Reference Identification
          Loop 2300A - Organization Summary Remittance Detail Adjustment
            ADX - Organization Summary Remittance Detail Adjustment
            REF - Organization Summary Remittance Detail Adjustment Reference
      Loop 2000B - Individual Remittance
        ENT - Individual Remittance
        Loop 2100B - Individual Name
          NM1 - Individual Name
          N2 - Individual Additional Name
          N3 - Individual Address
          N4 - Individual City/State/ZIP
          DMG - Individual Demographic Information
          REF - Individual Additional Identification
          Loop 2200B - Individual Premium Remittance Detail
            RMR - Individual Premium Remittance Detail
            DTM - Individual Premium Remittance Date
            REF - Individual Premium Reference Identification
            Loop 2300B - Individual Premium Adjustment
              ADX - Individual Premium Adjustment
              REF - Individual Premium Adjustment Reference
      Loop 2000C - Detail Premium Payment
        ENT - Detail Premium Payment
        Loop 2100C - Detail Premium Payment Identification
          NM1 - Detail Premium Payment Identification
          N2 - Detail Premium Payment Additional Name
          N3 - Detail Premium Payment Address
          N4 - Detail Premium Payment City/State/ZIP
          DMG - Detail Premium Payment Demographic Information
          REF - Detail Premium Payment Additional Identification
          Loop 2200C - Detail Premium Payment Insurance Product
            INS - Detail Premium Payment Insurance Product
            REF - Detail Premium Payment Insurance Product Reference
            DTP - Detail Premium Payment Insurance Product Date
            Loop 2300C - Detail Premium Payment Insurance Product Adjustment
              ADX - Detail Premium Payment Insurance Product Adjustment
              REF - Detail Premium Payment Insurance Product Adjustment Reference
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

### 834 (Benefit Enrollment and Maintenance) - ASC X12N/005010X220 (FUTURE)
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BGN - Beginning Segment
      REF - Transaction Set Policy Number
      DTP - File Effective Date
      QTY - Transaction Set Control Totals
      Loop 1000A - Sponsor Name
        N1 - Sponsor Name
        N2 - Sponsor Additional Name
        N3 - Sponsor Address
        N4 - Sponsor City/State/ZIP
        PER - Administrative Communications Contact
      Loop 1000B - Payer
        N1 - Payer
        N2 - Payer Additional Name
        N3 - Payer Address
        N4 - Payer City/State/ZIP
        PER - Administrative Communications Contact
      Loop 1000C - TPA/Broker Name
        N1 - TPA/Broker Name
        N2 - TPA/Broker Additional Name
        N3 - TPA/Broker Address
        N4 - TPA/Broker City/State/ZIP
        PER - Administrative Communications Contact
      Loop 2000 - Member Level Detail
        INS - Member Level Detail
        REF - Subscriber Identifier
        REF - Member Policy Number
        REF - Member Supplemental Identifier
        DTP - Member Level Dates
        Loop 2100A - Member Name
          NM1 - Member Name
          PER - Member Communications Numbers
          N3 - Member Address
          N4 - Member City/State/ZIP
          DMG - Member Demographics
          PM - Member Health Information
          EC - Employment Class
          ICM - Member Income
          AMT - Member Policy Amounts
          HLH - Member Health Information
          HI - Member Health Industry Code
          LUI - Member Language
        Loop 2100B - Incorrect Member Name
          NM1 - Incorrect Member Name
          DMG - Incorrect Member Demographics
        Loop 2100C - Member Mailing Address
          NM1 - Member Mailing Address
          N3 - Member Mail Street Address
          N4 - Member Mail City/State/ZIP
        Loop 2100D - Member Employer
          NM1 - Member Employer
          N3 - Member Employer Street Address
          N4 - Member Employer City/State/ZIP
          PER - Member Employer Communications Numbers
        Loop 2100E - Member School
          NM1 - Member School
          N3 - Member School Street Address
          N4 - Member School City/State/ZIP
          PER - Member School Communications Numbers
        Loop 2100F - Custodial Parent
          NM1 - Custodial Parent
          N3 - Custodial Parent Street Address
          N4 - Custodial Parent City/State/ZIP
          PER - Custodial Parent Communications Numbers
        Loop 2100G - Responsible Person
          NM1 - Responsible Person
          N3 - Responsible Person Street Address
          N4 - Responsible Person City/State/ZIP
          PER - Responsible Person Communications Numbers
        Loop 2100H - Drop Off Location
          NM1 - Drop Off Location
          N3 - Drop Off Location Street Address
          N4 - Drop Off Location City/State/ZIP
          PER - Drop Off Location Communications Numbers
        Loop 2200 - Disability Information
          DSB - Disability Information
          DTP - Disability Eligibility Dates
        Loop 2300 - Health Coverage
          HD - Health Coverage
          DTP - Health Coverage Dates
          AMT - Health Coverage Policy
          REF - Health Coverage Policy Number
          REF - Prior Coverage Months
          IDC - Identification Card
          Loop 2310 - Provider Information
            LX - Provider Information
            NM1 - Provider Name
            N3 - Provider Address
            N4 - Provider City/State/ZIP
            PER - Provider Communications Numbers
            PLA - Provider Change Reason
          Loop 2320 - Coordination of Benefits
            COB - Coordination of Benefits
            REF - Additional Coordination of Benefits Identifiers
            DTP - Coordination of Benefits Eligibility Dates
            Loop 2330 - Coordination of Benefits Related Entity
              NM1 - Coordination of Benefits Related Entity
              N3 - Coordination of Benefits Related Entity Address
              N4 - Coordination of Benefits Related Entity City/State/ZIP
              PER - Coordination of Benefits Related Entity Communications Numbers
        Loop 2700 - Member Reporting Categories
          LX - Member Reporting Categories
          Loop 2750 - Reporting Category
            N1 - Reporting Category
            REF - Reporting Category Reference
            DTP - Reporting Category Date
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 7. Additional Technical Considerations

### Variant-Specific Components
- Implement specialized handling for TOO segment in 837D format
- Implement specialized handling for CL1 segment in 837I format
- Ensure proper format detection logic to distinguish between 837 variants

### Line Breaks and Formatting
- Add line breaks between segments in generated output for better readability
- Implement consistent formatting for all transaction sets
- Ensure proper segment ordering in generated output

### Error Handling and Validation
- Implement comprehensive error handling for all transaction sets
- Add validation for required segments and fields
- Provide clear error messages for malformed input files

### Testing Strategy
- Create unit tests for all new functionality
- Implement integration tests for end-to-end processing
- Add regression tests to ensure backward compatibility
- Test with real-world EDI files from various sources

### Documentation
- Update README.md with new transaction set information
- Add usage examples for all transaction sets
- Document common error scenarios and solutions
- Provide schema documentation for JSON output
