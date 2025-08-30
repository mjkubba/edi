# Technology Stack

## Language & Runtime
- **Rust** (Edition 2021) - Primary language for performance and memory safety
- Minimum supported Rust version: 1.56.0

## Core Dependencies
- **serde** (1.0) - Serialization/deserialization framework with derive features
- **serde_json** (1.0) - JSON parsing and generation
- **log** (0.4) - Logging facade
- **env_logger** (0.10) - Environment-based logger implementation
- **once_cell** (1.18) - Thread-safe lazy static initialization

## Build System
- **Cargo** - Standard Rust package manager and build tool
- Dual target configuration:
  - Library: `edi_parser` (src/lib.rs)
  - Binary: `edi` (src/main.rs)

## Common Commands

### Development
```bash
# Build the project
cargo build

# Build optimized release version
cargo build --release

# Run the application
cargo run -- [args]

# Run tests
cargo test

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy linter
cargo clippy
```

### Usage Examples
```bash
# Convert EDI to JSON
cargo run -- -f input.edi -o output.json

# Convert JSON to EDI
cargo run -- -f input.json -o output.edi -w -j

# Parse specific transaction set
cargo run -- -f ./demo/test835-new.edi -o ./demo/output.json
```

## Architecture Patterns
- **Trait-based design** - `TransactionSet` trait for consistent behavior
- **Module-per-format** - Each EDI format has its own module (edi835/, edi999/, etc.)
- **Configuration-driven** - Segment definitions use configuration files
- **Error handling** - Custom `EdiError` enum with `EdiResult<T>` type alias
- **Logging** - Structured logging throughout the application