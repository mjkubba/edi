# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] — refactor/v0.3.0-architecture

### Refactored
- Migrated 276, 277, 820 to shared ISA/GS/GE/IEA structs (unified interchange envelope)
- Removed scattered `#[allow(dead_code)]` — uses crate-level allow instead
- Deduplicated 834 loop2100b-h into generic parameterized module

## [0.2.0] - 2026-04-27

### Security & Reliability (Production-Safe Milestone)
- Eliminated 998 panic paths — all segment parsers use safe `get_element()` access, all production `unwrap()` calls replaced with proper error handling
- Fixed `stiuational_element()` output corruption — migrated all segment writers to `build_segment()` per X12 §B.1.1.3.10 (preserves empty middle elements, strips trailing)
- Added 256MB file size limit to prevent OOM on large batch files
- Eliminated 172 unnecessary `contents.clone()` calls by changing function params from `String` to `&str`
- Fixed 837 P/I/D subtype detection in write path — now uses version identifiers (X222/X223/X224) instead of fragile JSON field sniffing
- Fixed 90 bare segment ID matches (`.find("SE")` etc.) that could false-match inside data values — now uses boundary-aware `"SE*"` patterns
- Added X12 envelope validation — checks ST02==SE02, GS06==GE02, ISA13==IEA02 per spec
- Restored CLP10 (patient_status_code) and fixed off-by-one field mapping in CLP segment
- Fixed hardcoded segment ID length of 3 — now uses dynamic `key.len()`

### New Features
- Added numeric/decimal field validation per X12 §B.1.1.3.1 (warns on invalid monetary values in BPR, CLP)
- Envelope validation module (`helper/envelope_validation.rs`)
- Restored interchange acknowledgment (TA1) segment parser and writer with tests

### Code Quality
- Fixed typos in public API names (`segement`, `adjustsment`, `numbner`, `scv04`)
- Removed unused `once_cell` dependency
- Removed commented-out dead code, cleaned up segment files
- Audited segment ID handling — confirmed consistent contract, documented
- Documented `clean_contents()` delimiter replacement as correct per X12 §B.1.1.2

### Tests
- 253 tests (up from 238), all passing
- All 12 demo files round-trip verified (EDI→JSON→EDI→JSON = identical)

## [0.1.0] - 2026-04-25

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
