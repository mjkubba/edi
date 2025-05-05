# Contributing to EDI Parser

Thank you for considering contributing to the EDI Parser project! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:
- A clear, descriptive title
- Steps to reproduce the bug
- Expected behavior
- Actual behavior
- Any relevant logs or error messages
- EDI file examples (with sensitive information removed)

### Suggesting Enhancements

For feature requests, please create an issue with:
- A clear, descriptive title
- A detailed description of the proposed feature
- Any relevant examples or use cases
- If applicable, sample EDI files demonstrating the need

### Pull Requests

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes
4. Run tests to ensure your changes don't break existing functionality
5. Submit a pull request

## Development Workflow

### Setting Up the Development Environment

```bash
# Clone the repository
git clone [repository-url]
cd edi

# Build the project
cargo build

# Run tests
cargo test
```

### Coding Standards

- Follow Rust best practices
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for common mistakes
- Write comprehensive tests for new functionality
- Document public API with rustdoc comments

### Commit Messages

All commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools

Example:
```
feat(837): Add support for TOO segment in dental claims

Implement specialized handling for TOO segments in 837D format.
Update Loop2300 to include fields for TOO segments.
Update parse_loop2300 to handle TOO segments.
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name
```

### Testing Methodology

1. Parse EDI files to JSON and verify structure
2. Generate EDI files from JSON and verify structure
3. Compare original and generated EDI files
4. Identify unprocessed segments and structural differences

## License

By contributing to this project, you agree that your contributions will be licensed under the project's license.
