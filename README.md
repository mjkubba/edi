# EDI Parser and Processor for Healthcare X12 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 formats. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837P/I/D (Claims).

## Project Status

| Transaction Set | Status | Description |
|----------------|--------|-------------|
| EDI835 (Payment/Remittance Advice) | ✅ Complete | Fully functional with all segments correctly processed and generated |
| EDI270 (Health Care Eligibility Benefit Inquiry) | ✅ Complete | Core functionality working, REF segments included in output, DTP segments fixed |
| EDI271 (Health Care Eligibility Benefit Response) | ✅ Complete | Core functionality working, PER/REF/DTP segments included in output, line breaks added |
| EDI999 (Implementation Acknowledgment) | ✅ Complete | Core functionality working, CTX segment formatting fixed, trailer segments fixed, line breaks added |
| EDI276/277 (Health Care Claim Status) | ✅ Complete | Basic structure implemented, controller functions added, parsing working, generation improved, segment ID fixes implemented, functional tests added |
| EDI837P (Health Care Claim Professional) | ✅ Complete | Basic structure created, TransactionSet trait implemented, parsing for all major loops implemented, tests added, main.rs updated to support 837P |
| EDI837I (Health Care Claim Institutional) | ✅ Complete | Basic structure created, TransactionSet trait implemented, parsing for all major loops implemented, tests added, main.rs updated to support 837I, specialized handling for CL1 segment |
| EDI837D (Health Care Claim Dental) | ✅ Complete | Basic structure created, TransactionSet trait implemented, parsing for all major loops implemented, main.rs updated to support 837D, specialized handling for TOO segment |

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

- **Multiple Transaction Set Support**: 835, 999, 270/271, 276/277, 837P/I/D
- **Configuration-Driven Architecture**: Segment and loop definitions are configurable
- **Robust Error Handling**: Comprehensive error types and validation
- **Bidirectional Conversion**: EDI to JSON and JSON to EDI
- **Extensible Design**: Easy to add new transaction sets and segments
- **Special Format Handling**: Support for complex CTX segments and other special formats
- **Variant-Specific Components**: Specialized handling for format-specific segments like TOO in 837D and CL1 in 837I

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
- ✅ Transaction Set 837P/I/D (Health Care Claim)
  - ✅ Implemented variant-specific components
  - ✅ Added specialized handling for TOO segment in 837D
  - ✅ Added specialized handling for CL1 segment in 837I
  - ✅ Improved format detection logic

### Planned
- Performance Optimization
  - Optimize parsing algorithms
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
