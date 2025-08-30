# Project Structure

## Root Directory Layout
```
edi/
├── src/                    # Source code
├── demo/                   # Test files and examples
├── target/                 # Cargo build artifacts (generated)
├── .git/                   # Git repository data
├── .kiro/                  # Kiro IDE configuration
├── .amazonq/               # Amazon Q configuration
├── Cargo.toml              # Rust project configuration
├── Cargo.lock              # Locked dependency versions
├── README.md               # Project documentation
├── LICENSE                 # License file
├── config.json             # Application configuration
└── *.md                    # Additional documentation
```

## Source Code Organization (`src/`)
```
src/
├── main.rs                 # CLI application entry point
├── lib.rs                  # Library exports and re-exports
├── error.rs                # Error types and handling
├── transaction_processor.rs # Generic transaction processing
├── segment_config.rs       # Configuration-driven segments
├── loop_processor.rs       # Loop detection and processing
├── helper/                 # Utility functions
├── segments/               # Common EDI segment definitions
└── edi{XXX}/               # Transaction-specific modules
    ├── mod.rs              # Module exports
    ├── controller.rs       # Main processing logic
    ├── table{N}.rs         # Table/header structures
    ├── loop{NNNN}.rs       # Loop-specific implementations
    └── *.rs                # Other format-specific components
```

## EDI Module Pattern
Each EDI transaction set follows a consistent structure:
- **controller.rs** - Main parsing/generation logic and public API
- **mod.rs** - Module exports and organization
- **table{N}.rs** - Header tables and control structures
- **loop{NNNN}.rs** - Hierarchical loop implementations
- **interchangecontrol*.rs** - ISA/GS envelope handling

## Demo Directory (`demo/`)
Contains test files organized by transaction set:
- **{transaction_set}/** - Folders for each supported format
- **test{XXX}-new.{edi|json}** - Test files for validation
- **test_results/** - Processing results and comparisons

## Key Files
- **Cargo.toml** - Dependencies, build configuration, binary/library setup
- **src/lib.rs** - Public API exports for library usage
- **src/main.rs** - CLI interface with format detection and routing
- **src/error.rs** - Centralized error handling with `EdiError` enum
- **src/transaction_processor.rs** - `TransactionSet` trait definition

## Naming Conventions
- **Modules**: lowercase with underscores (e.g., `edi835`, `loop_processor`)
- **Files**: snake_case (e.g., `transaction_processor.rs`)
- **Structs**: PascalCase (e.g., `Edi835`, `TransactionSet`)
- **Functions**: snake_case (e.g., `get_835`, `write_999`)
- **Constants**: SCREAMING_SNAKE_CASE

## Module Dependencies
- Each EDI module is self-contained with minimal cross-dependencies
- Common functionality shared through `helper/` and `segments/`
- Error handling centralized in `error.rs`
- Generic processing logic in `transaction_processor.rs`