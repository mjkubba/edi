# EDI Parser and Processor for Healthcare X12 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 formats. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), 278 (Health Care Services Review), and 837P/I/D (Claims).

## Project Status

| Transaction Set | Status | Description |
|----------------|--------|-------------|
| EDI835 (Payment/Remittance Advice) | ✅ Complete | Fully functional with minor formatting differences in output (missing empty fields in SVC segments) |
| EDI270 (Health Care Eligibility Benefit Inquiry) | ✅ Complete | Fully functional with line breaks in generated output, all segments correctly processed |
| EDI271 (Health Care Eligibility Benefit Response) | ✅ Complete | Fully functional with line breaks in generated output, all segments correctly processed including LS/LE |
| EDI999 (Implementation Acknowledgment) | ✅ Complete | Fully functional with special CTX segment handling for both standard and special formats |
| EDI276/277 (Health Care Claim Status) | ⚠️ Partial | Parsing works correctly but generation has issues with TRN and STC segments in 277 |
| EDI278 (Health Care Services Review) | ✅ Complete | Fully functional with support for AR/HS prefixes in UM segment, some segments missing in output (DTP, SV2, PRV) |
| EDI837P (Health Care Claim Professional) | ⚠️ Partial | Parsing works correctly but generation not yet fully implemented |
| EDI837I (Health Care Claim Institutional) | ⚠️ Partial | Parsing works correctly with specialized handling for CL1 segment, generation not yet implemented |
| EDI837D (Health Care Claim Dental) | ⚠️ Partial | Parsing works correctly with specialized handling for TOO segment, generation not yet implemented |

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

## Features

- **Multiple Transaction Set Support**: 835, 999, 270/271, 276/277, 278, 837P/I/D
- **Configuration-Driven Architecture**: Segment and loop definitions are configurable
- **Robust Error Handling**: Comprehensive error types and validation
- **Bidirectional Conversion**: EDI to JSON and JSON to EDI
- **Extensible Design**: Easy to add new transaction sets and segments
- **Special Format Handling**: Support for complex CTX segments and other special formats
- **Variant-Specific Components**: Specialized handling for format-specific segments like TOO in 837D and CL1 in 837I
- **Prefix Support**: Handling for special prefixes like AR/HS in UM segments for 278 transaction sets

## Usage Instructions
### Prerequisites
- Rust toolchain (1.56.0 or later)
- Cargo package manager
- Environment with logging capabilities for debug output

### Installation
```bash
# Clone the repository
git clone [repository-url]
cd edi

# Build the project
cargo build --release

# Run tests
cargo test
```

### Command Line Options
```
-f <file>     Input file path (EDI or JSON)
-o <file>     Output file path
-w            Write mode (convert JSON to EDI)
-j            Specify input is JSON
-h, --help    Show help information
```

### Examples
```bash
# Convert EDI to JSON
cargo run -- -f input.edi -o output.json

# Convert JSON to EDI
cargo run -- -f input.json -o output.edi -w -j
```

## Testing Methodology
- Parse EDI files to JSON and verify structure
- Generate EDI files from JSON and verify structure
- Compare original and generated EDI files
- Identify unprocessed segments and structural differences

```bash
# Parse EDI to JSON
cargo run -- -f ./demo/edi835-1.edi -o ./demo/test835-new.json

# Generate EDI from JSON
cargo run -- -f ./demo/test835-new.json -o ./demo/test835-new.edi -w -j

# Compare files
diff ./demo/edi835-1.edi ./demo/test835-new.edi
```

## Development Roadmap

### Completed
- ✅ Fixed CTX segment implementation in 999 format
- ✅ Improved error handling for malformed input files
- ✅ Addressed Table 1 content placement issues
- ✅ Added comprehensive unit tests
- ✅ Common Infrastructure Updates
  - ✅ Generic transaction set processor
  - ✅ Configuration-driven segment definitions
  - ✅ Enhanced loop detection and processing
  - ✅ Standardized error handling
- ✅ Transaction Set 270/271 (Health Care Eligibility)
- ✅ Transaction Set 276/277 (Health Care Claim Status)
- ✅ Transaction Set 278 (Health Care Services Review)
  - ✅ Implemented all loops and segments
  - ✅ Added support for AR/HS prefixes in UM segment
  - ✅ Added facility address handling
  - ✅ Added service provider details
- ✅ Transaction Set 837P/I/D (Health Care Claim)
  - ✅ Implemented variant-specific components
  - ✅ Added specialized handling for TOO segment in 837D
  - ✅ Added specialized handling for CL1 segment in 837I
  - ✅ Improved format detection logic

### Planned
- Complete 837 Generation
  - Implement write functionality for 837P/I/D formats
  - Ensure proper handling of variant-specific segments during generation
  - Add comprehensive tests for generation functionality
- Fix 276/277 Generation
  - Address missing segments in 276/277 generation process
  - Implement proper handling of TRN and STC segments in 277
  - Ensure complete segment processing for both formats
- Code Cleanup
  - Address compiler warnings, particularly unused imports and functions
  - Fix unused variable warnings
  - Improve code organization and documentation
- Performance Optimization
  - Optimize parsing algorithms for better performance with large files
  - Implement caching for frequently used segments
  - Reduce memory usage for large files
- Additional Features
  - Add support for custom delimiters
  - Implement pretty printing for output files
  - Add schema validation
  - Create a web interface for EDI processing

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
[Specify your license here]
