# EDI Parser and Processor for Healthcare X12 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 formats. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), 278 (Health Care Services Review), and 837P/I/D (Claims).

## Project Status

| Transaction Set | Status | Description |
|----------------|--------|-------------|
| EDI835 (Payment/Remittance Advice) | ✅ Complete | Fully functional with minor formatting differences in output (missing empty fields in SVC segments) |
| EDI270 (Health Care Eligibility Benefit Inquiry) | ✅ Complete | Fully functional with line breaks in generated output, all segments correctly processed |
| EDI271 (Health Care Eligibility Benefit Response) | ✅ Complete | Fully functional with line breaks in generated output, all segments correctly processed including LS/LE |
| EDI999 (Implementation Acknowledgment) | ✅ Complete | Fully functional with special CTX segment handling for both standard and special formats |
| EDI276/277 (Health Care Claim Status) | ✅ Complete | Fully functional with fixed TRN and STC segment handling, all required segments included in output |
| EDI278 (Health Care Services Review) | ✅ Complete | Fully functional with support for AR/HS prefixes in UM segment, some segments missing in output (DTP, SV2, PRV) |
| EDI837P (Health Care Claim Professional) | ✅ Complete | Fully functional with parsing and generation capabilities, all segments correctly processed |
| EDI837I (Health Care Claim Institutional) | ✅ Complete | Fully functional with specialized handling for CL1 segment, parsing and generation working correctly |
| EDI837D (Health Care Claim Dental) | ✅ Complete | Fully functional with specialized handling for TOO segment, parsing and generation working correctly |

## Repository Structure
```
edi/
├── src/                         # Source code directory
│   ├── edi835/                  # Healthcare Claim Payment/Advice format implementation
│   ├── edi999/                  # Functional Acknowledgment format implementation
│   ├── edi270/                  # Eligibility Benefit Inquiry format implementation
│   ├── edi271/                  # Eligibility Benefit Response format implementation
│   ├── edi276/                  # Health Care Claim Status Request implementation
│   ├── edi277/                  # Health Care Claim Status Response implementation
│   ├── edi278/                  # Health Care Services Review implementation
│   ├── edi837/                  # Health Care Claim implementation (Professional, Institutional, Dental)
│   ├── helper/                  # Utility functions and shared helpers
│   ├── segments/                # EDI segment definitions and processors
│   ├── error.rs                 # Error handling module
│   ├── transaction_processor.rs # Generic transaction set processor
│   ├── segment_config.rs        # Configuration-driven segment definitions
│   ├── loop_processor.rs        # Enhanced loop detection and processing
│   ├── lib.rs                   # Library exports
│   └── main.rs                  # Application entry point
├── Cargo.toml                   # Rust project configuration and dependencies
└── Cargo.lock                   # Locked dependencies versions
```
