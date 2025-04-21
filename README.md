# EDI Parser and Processor for Healthcare X12 835/999 Formats in Rust

This project provides a robust Electronic Data Interchange (EDI) parser and processor specifically designed for healthcare X12 

Make it work <= we are here   
Make it right   
Make it fast   

I'm using Amazon Q Developer to help me with this new endeavour if there are any code references I'll include them here.

The parser implements support for EDI X12 segment handling, including interchange control, functional groups, and transaction sets. It features specialized modules for processing healthcare-specific loops and segments, making it particularly valuable for healthcare claims processing systems and medical billing applications. The implementation follows strict EDI standards while providing a developer-friendly API for parsing and generating EDI documents.

## Repository Structure
```
edi/
├── src/                          # Source code directory
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
│   │   ├── edihelper.rs        # Common EDI processing functions
│   │   └── helper.rs           # General helper functions
│   ├── segments/               # EDI segment definitions and processors
│   │   ├── isa.rs             # Interchange Control Header
│   │   ├── gs.rs              # Functional Group Header
│   │   └── [other segments]    # Individual segment implementations
│   └── main.rs                 # Application entry point
├── Cargo.toml                  # Rust project configuration and dependencies
└── Cargo.lock                  # Locked dependencies versions
```

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

### Inputs   
To provide EDI file use `-f` then the file name.   
To specify the output file use `-o` then output the file name.     
`cargo run -f <edifilepath> -o <outputfile>` or the compiled version `./edi -f <edifilepath> -o <outputfile>` for *nix and `.\edi.exe -f <edifilepath> -o <outputfile>` for Windows.   
If no file path provided the demo file will be used as input.

### Outputs:   
If file path is provided in the 2nd place after the file name it will be used to dump the json,     
otherwise json output will be written in `out.json` file
`cargo run -f <edifilepath> -o <outputfile>`


## Data Flow
The EDI processor handles documents through a pipeline of parsing, validation, and processing stages, transforming raw EDI text into structured data objects.

```ascii
Raw EDI Input
     │
     ▼
[Parser Layer]
     │
     ▼
[Validation Layer]
     │
     ▼
[Processing Layer]
     │
     ▼
Structured Output
```

Component interactions:
1. Parser reads raw EDI text and splits into segments
2. Segments are validated against X12 specifications
3. Valid segments are grouped into functional groups and transactions
4. Transaction sets are processed according to their type (835/999)
5. Business rules are applied to transaction data
6. Results are transformed into structured output format
7. Error handling occurs at each stage with appropriate logging

### TODO:
* ~~implement logger~~
* ~~check if the file passed is 835, this can be read from ST*835*~~
* ~~Check against the guide how many of each segment is in each loop~~
* ~~Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle~~
* ~~Adding parameterized input, -f for file -o for output etc.~~
* ~~Adding Write EDI 835 functionality~~
* ~~Finding some mismatches between the standard and the implementation of EDI835!!!~~
* 999 have segment loops, similar to 835, need to write the logic for these.
     * Where I left: fixing CTX, need time to debug this against the standards to see if I coded CTX to 999 standards or something else, what is required and what is situational, it's failing in writing all parts.
* Make it safer when something does not exist
* More cool things


### Artifacts and Components:
Used example file from https://x12.org/examples/005010x221 located at the src dir, will not be included in the repo (gitignored)