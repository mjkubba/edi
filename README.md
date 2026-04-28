# EDI Parser — Healthcare X12 to JSON Converter

A fast, single-binary tool for converting healthcare EDI X12 files to JSON and back. Supports all major transaction sets used in US healthcare.

## Supported Formats

| Transaction Set | Description |
|----------------|-------------|
| 835 | Health Care Claim Payment/Remittance Advice |
| 999 | Implementation Acknowledgment |
| 270/271 | Health Care Eligibility Benefit Inquiry & Response |
| 276/277 | Health Care Claim Status Request & Response |
| 278 | Health Care Services Review |
| 837P | Health Care Claim — Professional |
| 837I | Health Care Claim — Institutional |
| 837D | Health Care Claim — Dental |
| 820 | Health Insurance Exchange Related Payments |
| 834 | Benefit Enrollment and Maintenance |

## Quick Start

### Download

Grab the latest binary from [Releases](https://github.com/mjkubba/edi/releases).

### Or build from source

```bash
git clone https://github.com/mjkubba/edi.git
cd edi
cargo build --release
# Binary at target/release/edi (or edi.exe on Windows)
```

## Usage

### Convert EDI to JSON
```bash
edi -f claim.edi -o claim.json
```

### Convert JSON back to EDI
```bash
edi -f claim.json -o claim.edi -w -j
```

### Options
```
-f <file>     Input file (EDI or JSON)
-o <file>     Output file (defaults to out.json or out.edi)
-w            Write mode — generate EDI from JSON
-j            Input is JSON (use with -w)
-h, --help    Show help
```

### Examples

```bash
# Parse an 835 payment file
edi -f remittance.edi -o remittance.json

# Parse a 270 eligibility inquiry
edi -f eligibility_request.edi -o eligibility.json

# Generate an 837P claim from JSON
edi -f professional_claim.json -o claim.edi -w -j

# Round-trip test (parse then regenerate)
edi -f original.edi -o parsed.json
edi -f parsed.json -o regenerated.edi -w -j
diff original.edi regenerated.edi
```

The parser auto-detects the transaction set type from the content — no need to specify which format you're working with.

## Features

- **Auto-detection** — Identifies transaction set type from ST segments and implementation guide references
- **Bidirectional** — EDI → JSON and JSON → EDI
- **Round-trip safe** — Parse and regenerate with identical output for all supported formats
- **Custom delimiters** — Automatically detects non-standard element separators and segment terminators from the ISA segment
- **Single binary** — No runtime dependencies, no config files needed
- **Demo files included** — Sample EDI files for all 12 transaction sets in the `demo/` directory

---

## Development

### Prerequisites
- Rust toolchain (1.56.0 or later)

### Build & Test
```bash
cargo build
cargo test    # 253 tests
```

### Testing with demo files
```bash
# Parse a demo file
cargo run -- -f ./demo/edi835-demo-005010X221.edi -o ./demo/test835.json

# Regenerate and compare
cargo run -- -f ./demo/test835.json -o ./demo/test835.edi -w -j
diff ./demo/edi835-demo-005010X221.edi ./demo/test835.edi
```

Demo files are AI-generated based on public X12 implementation guides, intended for testing only.

### Project Structure
```
src/
├── edi835/    edi999/    edi270/    edi271/     # Transaction set modules
├── edi276/    edi277/    edi278/    edi837/     # Each has controller, loops, segments
├── edi820/    edi834/                           
├── segments/              # 58 shared segment parsers (NM1, CLM, REF, etc.)
├── helper/                # CLI args, file I/O, content cleaning
├── lib.rs                 # Library exports
└── main.rs                # CLI entry point
```

### Roadmap
- Deduplicate 837 P/I/D shared loop code into common module
- 837 HL parent-child tree structure for multi-subscriber batches
- Consolidate architectural patterns (trait-based routing)
- `Option<String>` for optional X12 fields
- Web UI for browser-based EDI processing
- Schema validation
- Performance optimization for large files

## Contributing

Contributions welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
