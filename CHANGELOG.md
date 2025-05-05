# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-05-05

### Added
- Initial release with support for multiple EDI transaction sets:
  - EDI835 (Payment/Remittance Advice)
  - EDI999 (Implementation Acknowledgment)
  - EDI270/271 (Eligibility Inquiry/Response)
  - EDI276/277 (Claim Status Request/Response)
  - EDI837P/I/D (Healthcare Claims)
- Configuration-driven segment definitions
- Generic transaction set processor
- Standardized error handling
- Bidirectional conversion (EDI to JSON and JSON to EDI)
- Special format handling for complex segments
- Variant-specific components for 837 formats
- Comprehensive unit tests
- Command-line interface for file processing

### Fixed
- CTX segment formatting in 999 format
- Table 1 content placement issues
- REF segment handling in 270/271 formats
- PER/DTP segment handling in 271 format
- Trailer segment values in 999 format
- Line breaks in generated output for better readability
- Format detection logic for 837 variants
