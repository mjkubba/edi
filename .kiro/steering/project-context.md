---
inclusion: always
---

# EDI Parser Project Context

## Project Overview
This project implements a comprehensive Electronic Data Interchange (EDI) parser and processor for healthcare X12 formats in Rust. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837P/I/D (Claims).

## Key Features
- **Multiple Transaction Set Support**: 835, 999, 270/271, 276/277, 837P/I/D
- **Configuration-Driven Architecture**: Segment and loop definitions are configurable
- **Robust Error Handling**: Comprehensive error types and validation
- **Bidirectional Conversion**: EDI to JSON and JSON to EDI
- **Extensible Design**: Easy to add new transaction sets and segments
- **Special Format Handling**: Support for complex CTX segments and other special formats
- **Variant-Specific Components**: Specialized handling for format-specific segments

## Current Implementation Status

### Completed Transaction Sets
- ✅ EDI835 (Payment/Remittance Advice) - Fully functional
- ✅ EDI999 (Implementation Acknowledgment) - Fully functional
- ✅ EDI270 (Health Care Eligibility Benefit Inquiry) - Fully functional
- ✅ EDI271 (Health Care Eligibility Benefit Response) - Fully functional
- ✅ EDI276 (Health Care Claim Status Request) - Functional with differences
- ✅ EDI277 (Health Care Claim Status Response) - Functional with differences
- ✅ EDI278 (Health Care Services Review) - Functional with minor differences
- ✅ EDI837P (Health Care Claim Professional) - Functional with differences
- ✅ EDI837I (Health Care Claim Institutional) - Functional with differences
- ✅ EDI837D (Health Care Claim Dental) - Functional
- ✅ EDI820 (Health Insurance Exchange Related Payments) - Partially functional

### Not Implemented
- ❌ EDI834 (Benefit Enrollment and Maintenance) - Not implemented

## Project Structure
The project follows a modular structure with separate modules for each transaction set:

```
edi/
├── src/
│   ├── edi835/                  # Healthcare Claim Payment/Advice format
│   ├── edi999/                  # Implementation Acknowledgment format
│   ├── edi270/                  # Eligibility Benefit Inquiry format
│   ├── edi271/                  # Eligibility Benefit Response format
│   ├── edi276/                  # Health Care Claim Status Request
│   ├── edi277/                  # Health Care Claim Status Response
│   ├── edi837/                  # Health Care Claim implementation
│   ├── helper/                  # Utility functions and shared helpers
│   ├── segments/                # EDI segment definitions and processors
│   ├── error.rs                 # Error handling module
│   ├── transaction_processor.rs # Generic transaction set processor
│   ├── segment_config.rs        # Configuration-driven segment definitions
│   ├── loop_processor.rs        # Enhanced loop detection and processing
│   ├── lib.rs                   # Library exports
│   └── main.rs                  # Application entry point
```

Each transaction set module contains:
- Controller for parsing and generating EDI content
- Loop structures for organizing segments
- Segment definitions and processors
- Unit tests for validation