# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-04-27

### Security & Reliability (Production-Safe Milestone)
- **C1**: Eliminated 998 panic paths — all segment parsers use safe `get_element()` access, all production `unwrap()` calls replaced with proper error handling
- **C2**: Fixed `stiuational_element()` output corruption — migrated all segment writers to `build_segment()` per X12 §B.1.1.3.10 (preserves empty middle elements, strips trailing)
- **C4**: Added 256MB file size limit to prevent OOM on large batch files
- **C5**: Eliminated 172 unnecessary `contents.clone()` calls by changing function params from `String` to `&str`
- **C7**: Fixed 837 P/I/D subtype detection in write path — now uses version identifiers (X222/X223/X224) instead of fragile JSON field sniffing
- **H1+H2**: Fixed 90 bare segment ID matches (`.find("SE")` etc.) that could false-match inside data values — now uses boundary-aware `"SE*"` patterns
- **H5**: Added X12 envelope validation — checks ST02==SE02, GS06==GE02, ISA13==IEA02 per spec
- **H6**: Restored CLP10 (patient_status_code) and fixed off-by-one field mapping in CLP segment
- **H7**: Fixed hardcoded segment ID length of 3 — now uses dynamic `key.len()`

### New Features
- **H4**: Added numeric/decimal field validation per X12 §B.1.1.3.1 (warns on invalid monetary values in BPR, CLP)
- **H5**: Envelope validation module (`helper/envelope_validation.rs`)
- **TA1**: Restored interchange acknowledgment segment parser and writer with tests

### Code Quality
- **L1**: Fixed typos in public API names (`segement`, `adjustsment`, `numbner`, `scv04`)
- **L4**: Removed unused `once_cell` dependency
- **L5**: Removed commented-out dead code, cleaned up segment files
- **C3**: Audited segment ID handling — confirmed consistent contract, documented
- **C6**: Documented `clean_contents()` delimiter replacement as correct per X12 §B.1.1.2

### Tests
- 253 tests (up from 238), all passing
- All 12 demo files round-trip verified (EDI→JSON→EDI→JSON = identical)

## [Unreleased]

### Fixed
- **edi837P/I/D**: Rewrote `parse_loop2300` with sequential segment processing — eliminates `find()` ordering bugs that caused segments to be skipped when they appeared in non-standard order
- **edi837**: Fixed `parse_loop2000a` PRV boundary — was consuming claim-level PRV*PE from far ahead in content
- **edi837**: Fixed envelope segment double-tilde in `write_837p`/`write_837i`/`write_837d` — raw segments already contain `~`, write functions no longer add another
- **edi837**: Removed redundant CL1/TOO parsing from 837I/D controllers — now handled by sequential `parse_loop2300`
- **edi835**: Fixed output formatting — segments now written one-per-line instead of all on single line
- **edi835**: Fixed TS3 parser element position mapping — TS306-TS312 are NOT USED per TR3, TS313 now correctly at position 12
- **edi835**: Added `build_segment()` helper for X12 §3.7 compliant trailing separator suppression — preserves empty middle fields, trims trailing empties
- **edi835**: Converted TS3, TS2, MIA, SVC, PLB writers to use `build_segment()`
- **edi834**: Fixed Loop2320/2330 — moved from Loop2000 level to inside Loop2300 per spec; Loop2330 corrected from DSB (disability) to NM1 (COB insurer)
- **edi276**: AMT/DTP segments at subscriber level now captured on round-trip
- **main.rs**: Fixed `clean_contents` result being discarded (was stored in unused `_clean_contents` variable)

### Added
- **edi837 Loop2000B**: NM1*PR (payer name), DMG after NM1*IL, payer N3/N4/PER/REF
- **edi837 Loop2000C**: DMG parsing after NM1*QC (handles both before-NM1 and after-NM1 orderings)
- **edi837 Loop2300**: NM1*82/71/72 (rendering/attending/operating provider) and associated PRV*PE
- **edi837 Loop2400**: TOO segment parsing for 837D dental claims
- **edi834 Loop2320**: COB segment + REF + DTP with nested Loop2330 (NM1 + N3 + N4)
- **Custom delimiter support**: `clean_contents()` detects ISA element separator (position 3) and segment terminator (position 105), normalizes to standard `*` and `~`

### Changed
- 238 tests passing, 0 compiler warnings (was 237 tests, 26 warnings)
- Removed dead code: `loop2010ba.rs`, `loop2010bb.rs`, `loop2010ca.rs`, unused parse/write functions from `table1.rs`, `interchangecontrol.rs`, `interchangecontroltrailer.rs`
- Removed unused generic infrastructure: `segment_config.rs`, `loop_processor.rs`, `TransactionProcessor`
- Suppressed warnings on public API (`get_278`, `get_820`) and infrastructure kept for future use (`EdiError` variants, `has_segment`)
- All 12 transaction sets now round-trip identical or trailing-newline-only

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
