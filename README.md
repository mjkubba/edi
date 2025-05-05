# EDI Parser and Processor for Healthcare X12 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 formats. It supports multiple transaction sets including 835 (Payment/Remittance Advice), 999 (Implementation Acknowledgment), 270/271 (Eligibility), 276/277 (Claim Status), and 837 (Claims).

## Project Status

- **Phase 1**: ✅ Complete - Fixed CTX segment implementation, improved error handling, addressed Table 1 content placement
- **Phase 2**: ✅ Complete - Implemented common infrastructure and additional transaction sets (270/271, 276/277)
- **Phase 3**: 🔄 In Progress - Implementing 837 transaction sets and performance optimizations

The parser implements support for EDI X12 segment handling, including interchange control, functional groups, and transaction sets. It features specialized modules for processing healthcare-specific loops and segments, making it particularly valuable for healthcare claims processing systems and medical billing applications. The implementation follows strict EDI standards while providing a developer-friendly API for parsing and generating EDI documents.

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

- **Multiple Transaction Set Support**: 835, 999, 270/271, 276/277, with 837 in progress
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

### Phase 2: ✅ Complete
- Common Infrastructure Updates
  - Generic transaction set processor
  - Configuration-driven segment definitions
  - Enhanced loop detection and processing
  - Standardized error handling
- Transaction Set 270 (Health Care Eligibility Inquiry)
  - Fixed REF segments not being included in output
  - Fixed DTP segment parsing
  - Added line breaks between segments
- Transaction Set 271 (Health Care Eligibility Response)
  - Fixed PER, REF, and DTP segments in output
  - Enhanced logging and error handling
- Transaction Set 276 (Health Care Claim Status Request)
  - Implemented full parsing and generation
  - Added support for all required loops and segments
- Transaction Set 277 (Health Care Claim Status Response)
  - Implemented full parsing and generation
  - Added support for STC segments
  - Enhanced nested loop handling

### Phase 3: 🔄 In Progress
- Transaction Set 837 (Health Care Claim)
  - Created directory structure for 837P, 837I, and 837D variants
  - Implemented common segments and basic loop structure
  - Implemented parsing for Loop2000A (Billing Provider)
  - Implemented parsing for Loop2010AA, Loop2010AB, Loop2010AC
  - In progress: Implementing remaining loops and segments
- Performance optimization
  - Planned: Optimize parsing algorithms
  - Planned: Implement caching for frequently used segments
- Additional features
  - Planned: Custom delimiters
  - Planned: Pretty printing
  - Planned: Schema validation

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
