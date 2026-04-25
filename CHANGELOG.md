# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Fixed
- **NM1 segment**: Added `id_code_qualifier` field (NM108), fixed `write_nm1` to trim trailing empty fields
- **edi837**: Fixed loop boundary detection in loop2000b/c to recognize CLM/LX segments, preventing claim data from being consumed by subscriber parsing
- **edi837**: Fixed `write_837p`, `write_837i`, `write_837d` to write loop2400 (service lines) nested inside each loop2300 (claim) instead of from a separate top-level collection
- **edi837**: Reordered REF parsing before AMT/QTY in loop2300 to match X12 segment order and prevent data loss
- **edi278**: Removed incorrect UM prefix detection logic — "AR" and "HS" are valid UM01 request category codes, not separate prefixes
- **edi278**: Fixed PRV parser `get_prv()` off-by-one error where segment ID was expected but already stripped by caller
- **edi278**: Fixed loop2010f parser to accept NM1 entity ID "1P" (Billing Provider) in addition to "SJ" (Service Provider)
- **edi276/277**: Fixed HL and NM1 segment parsing to strip segment ID prefix before calling `get_hl`/`get_nm1`
- **edi277**: Fixed BHT parser to use individual bounds checks per field instead of requiring all 7 elements
- **edi276/277**: Added bht06 (transaction type code) to test data for complete round-trip testing
- **edi999**: Fixed infinite loop / memory crash caused by segment identifiers (IK3, IK4) appearing inside CTX segment data values
- **edi835**: Fixed NM1 test assertion for corrected patient `id_code` field
- **edi271**: Fixed NM1 test assertions for `id_code_qualifier` and `id_code` fields in loop2115c

### Added
- `count_segment_starts()` and `find_next_segment_start()` helper functions for boundary-aware segment counting and searching
- EDI834 (Benefit Enrollment and Maintenance) basic implementation with all loop structures

### Changed
- All 237 tests now pass (previously 28 failures + memory crash)
- UM segment `um00_request_category_code_prefix` field is no longer populated by the parser; values go directly into `um01_request_category_code`
