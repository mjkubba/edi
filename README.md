# EDI Parser and Processor for Healthcare X12 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 formats. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837 (Claims).

## Project Status

- **Phase 1**: ✅ Complete - Fixed CTX segment implementation, improved error handling, addressed Table 1 content placement
- **Phase 2**: 🔄 In Progress - Implementing common infrastructure and additional transaction sets

The parser implements support for EDI X12 segment handling, including interchange control, functional groups, and transaction sets. It features specialized modules for processing healthcare-specific loops and segments, making it particularly valuable for healthcare claims processing systems and medical billing applications. The implementation follows strict EDI standards while providing a developer-friendly API for parsing and generating EDI documents.

## Repository Structure
```
edi/
├── src/                         # Source code directory
│   ├── edi835/                  # Healthcare Claim Payment/Advice format implementation
│   ├── edi999/                  # Functional Acknowledgment format implementation
│   ├── edi270/                  # Eligibility Benefit Inquiry format implementation
│   ├── edi271/                  # Eligibility Benefit Response format implementation
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

- **Multiple Transaction Set Support**: 835, 999, 270/271, with 276/277 and 837 planned
- **Configuration-Driven Architecture**: Segment and loop definitions are configurable
- **Robust Error Handling**: Comprehensive error types and validation
- **Bidirectional Conversion**: EDI to JSON and JSON to EDI
- **Extensible Design**: Easy to add new transaction sets and segments
- **Special Format Handling**: Support for complex CTX segments and other special formats

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

## Run application
cargo run
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

## Development Roadmap

### Phase 1: ✅ Complete
- Fixed CTX segment implementation in 999 format
- Improved error handling for malformed input files
- Addressed Table 1 content placement issues
- Added comprehensive unit tests

### Phase 2: 🔄 In Progress
- Common Infrastructure Updates
  - Generic transaction set processor
  - Configuration-driven segment definitions
  - Enhanced loop detection and processing
  - Standardized error handling
- Transaction Set 270 (Health Care Eligibility Inquiry)
- Transaction Set 271 (Health Care Eligibility Response)
- Enhanced 999 Implementation
  - Fixed CTX segment handling for special formats
  - Improved IK4 segment generation
  - Enhanced input file processing

### Phase 3: 🔄 Planned
- Transaction Set 276/277 (Health Care Claim Status)
- Transaction Set 837 (Health Care Claim)
- Performance optimization
- Schema validation
- Additional features (custom delimiters, pretty printing, etc.)

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
