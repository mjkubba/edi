# EDI Parser Project Overview

## Project Summary

This project implements a comprehensive Electronic Data Interchange (EDI) parser and processor for healthcare X12 formats in Rust. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837P/I/D (Claims).

## Key Features

- **Multiple Transaction Set Support**: 835, 999, 270/271, 276/277, 837P/I/D
- **Configuration-Driven Architecture**: Segment and loop definitions are configurable
- **Robust Error Handling**: Comprehensive error types and validation
- **Bidirectional Conversion**: EDI to JSON and JSON to EDI
- **Extensible Design**: Easy to add new transaction sets and segments
- **Special Format Handling**: Support for complex CTX segments and other special formats
- **Variant-Specific Components**: Specialized handling for format-specific segments

## Implementation Highlights

### Common Infrastructure

- **Generic Transaction Set Processor**: Standardized approach to parsing and generating EDI content
- **Configuration-Driven Segment Definitions**: Flexible configuration for segment structures
- **Enhanced Loop Detection and Processing**: Robust handling of nested loops
- **Standardized Error Handling**: Comprehensive error types and validation

### Transaction Set Implementations

#### EDI835 (Payment/Remittance Advice)
- Complete implementation with proper segment handling
- Fixed REF segment in Table1 to ensure the qualifier (EV) is included
- Rewrote the write_per function to handle all cases correctly
- Reordered segments in write_loop2100 function to match expected order

#### EDI999 (Implementation Acknowledgment)
- Fixed CTX segment formatting for special formats like "CLM01:123456789"
- Fixed CTX segment formatting for empty fields in the middle of the segment
- Added proper values for trailer segments (SE, AK9, GE, IEA)
- Added line breaks between segments in the generated output

#### EDI270/271 (Eligibility Inquiry/Response)
- Fixed REF, PER, and DTP segments not being included in the generated output
- Added line breaks between segments in the generated output
- Enhanced logging to show segment details
- Added tests to verify segment handling

#### EDI276/277 (Claim Status Request/Response)
- Enhanced loop processing for Loop2100A and Loop2100B
- Added comprehensive documentation to functions and structures
- Fixed compilation errors related to field name mismatches
- Added support for STC segments in EDI277
- Fixed JSON to EDI conversion for both formats
- Improved segment ID handling to ensure proper output format

#### EDI837P/I/D (Healthcare Claims)
- Implemented all three variants: Professional, Institutional, and Dental
- Added specialized handling for TOO segment in 837D format
- Added specialized handling for CL1 segment in 837I format
- Improved format detection logic to better distinguish between variants
- Updated Loop2300 to include fields for specialized segments

## Development Roadmap

### Completed
- ✅ All transaction sets implemented (835, 999, 270/271, 276/277, 837P/I/D)
- ✅ Specialized segment handling for variant-specific components
- ✅ Improved format detection logic
- ✅ Comprehensive unit tests
- ✅ Code quality improvements (documentation, error handling, etc.)

### Planned
- Performance optimization (parsing algorithms, caching, memory usage)
- Additional features (custom delimiters, pretty printing, schema validation)
- Web interface for EDI processing

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