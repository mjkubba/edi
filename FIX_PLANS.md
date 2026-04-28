# EDI Parser — Fix Plans for All Review Issues

**Date:** 2026-04-27
**Dependency order matters.** Issues are numbered by review severity but should be executed in the order shown in the Execution Roadmap at the bottom.

---

## CRITICAL

### C1: Eliminate panics on untrusted input
**Scope:** 910 direct-index `[n]` accesses, 87 `.unwrap()` calls, 5 `process::exit()` — ~60 files
**Risk:** DoS on any malformed input

**Plan:**
1. Add `get_element(parts, idx) -> String` and `get_required_element(parts, idx, seg, field) -> EdiResult<String>` to `edihelper.rs`
2. Convert all ~45 segment parsers: change `get_xxx(content: String) -> XXX` to `get_xxx(content: &str) -> EdiResult<XXX>`, replace `[n]` with safe helpers
3. Convert controllers and `main.rs`: propagate `Result` with `?`, replace `process::exit()` with `Result` returns
4. Add malformed-input tests (truncated, empty, garbage)

**Commit sequence:** 4 commits (helpers → segments → controllers → tests)

---

### C2: Fix `stiuational_element()` output corruption
**Scope:** 51 call sites across 9 files (cas.rs=17, moa.rs=10, ak9.rs=6, clp.rs=6, n4.rs=4, n1.rs=3, n3.rs=2, st.rs=2)
**Risk:** Silent field-position shift in output — violates X12 §B.1.1.3.10

**Plan:**
1. Delete `stiuational_element()` from `edihelper.rs`
2. Replace all 51 call sites with `build_segment()` which already exists and correctly handles trailing suppression while preserving middle empties
3. Each `write_xxx()` function becomes: `build_segment(&["SEG", &field1, &field2, ...])`
4. Add round-trip test: parse demo file → write → parse again → assert equal

**Commit sequence:** 1 commit per segment file, or batch all in 1 commit

---

### C3: Standardize segment ID handling
**Scope:** All segment parsers + `get_segment_contents()` in `edihelper.rs`
**Risk:** Off-by-one field mapping = silent data corruption

**Plan:**
1. Establish contract: `get_segment_contents(key, contents)` always strips the segment ID and leading `*`. Callers receive elements starting at index 0 = first data element (not the segment ID).
2. Audit every `get_xxx()` parser — verify index 0 maps to the first data element per X12 spec
3. Fix any parser that assumes index 0 is the segment ID
4. Add assertion tests: for each segment, verify field mapping against X12 spec element positions

**Commit sequence:** 1 commit (audit + fix + tests)

---

### C4: Add file size limits
**Scope:** `helper/helper.rs` — `get_file_contents()`
**Risk:** OOM on large batch files

**Plan:**
1. Add a configurable max file size constant (default: 256MB — covers large batches while preventing abuse)
2. Check file metadata size before reading; return `EdiError` if exceeded
3. Use `BufReader` instead of `read_to_string` for future streaming support
4. Add test with a file exceeding the limit

**Commit sequence:** 1 commit

---

### C5: Eliminate O(n²) `contents.clone()`
**Scope:** 201 `contents.clone()` calls across 49 files
**Risk:** Quadratic memory usage — each segment parse clones the entire remaining content

**Plan (Option A — implemented):**
1. Change all controller/loop functions from `fn parse(contents: String)` to `fn parse(contents: &str)`
2. Remove `contents.clone()` at call sites — pass `&contents` instead
3. Internal `content_trim()` still returns `String` (allocates), but callers no longer clone just to pass in

**Future optimization (Option B — zero-copy with offsets):**
Instead of `content_trim()` creating new Strings, track a byte offset into the original:
```rust
pub fn get_loop_2000(contents: &str) -> (Loop2000, usize) {
    // returns bytes consumed; caller does: contents = &contents[consumed..];
}
```
This eliminates ALL allocations during parsing but requires rethinking `content_trim()` to return
slice offsets instead of new Strings. Deferred until after the full fix plan is complete.

---

### C6: Fix `clean_contents()` delimiter corruption
**Scope:** `helper/helper.rs` — `clean_contents()`
**Risk:** Global `replace()` changes delimiters inside data values

**Plan:**
1. Parse ISA as fixed-length (positions 3 and 105) to detect custom delimiters — this part is already correct
2. Instead of global `replace()`, walk the content segment-by-segment: split on the detected segment terminator, then within each segment split on the detected element separator, and rejoin with `*` and `~`
3. This preserves data values that happen to contain the delimiter character
4. Add test: ISA with `|` delimiter where a data value also contains `|`

**Commit sequence:** 1 commit

---

### C7: Fix 837 P/I/D subtype detection
**Scope:** `main.rs` routing + 837 controller `detect()` methods
**Risk:** Wrong parser applied to wrong subtype

**Plan:**
1. Use `GS08` (Version/Release/Industry Identifier Code) or `ST03` to distinguish subtypes:
   - `005010X222` → 837P (Professional)
   - `005010X223` → 837I (Institutional)
   - `005010X224` → 837D (Dental)
2. `main.rs` already does this for the read path (confirmed in code). Fix the write path which uses fragile JSON field sniffing.
3. Remove duplicate `detect()` methods from 837 controllers
4. Add test: feed each demo file through detection, assert correct subtype

**Commit sequence:** 1 commit

---

## HIGH

### H1: Fix HL loop detection (substring matching)
**Scope:** 837 controller, 276/277 controllers
**Risk:** Matches HL content inside unrelated segments

**Plan:**
1. Replace `contents.contains("HL*")` and `contents.find("HL")` with `find_next_segment_start("HL", contents, pos)` which already exists in `edihelper.rs` and checks segment boundaries
2. Apply same fix to all segment-finding logic in controllers
3. Add test: content where "HL" appears inside a data value

**Commit sequence:** 1 commit

---

### H2: Fix 276/277 `find("SE")` false matches
**Scope:** `edi276/controller.rs`, `edi277/controller.rs`
**Risk:** Premature content truncation

**Plan:**
1. Replace `contents.find("SE")` with `find_next_segment_start("SE", contents, 0)` — boundary-aware search
2. Same for any other bare `find()` calls on segment IDs
3. Add test: content with "SERVICES" in a data value before the actual SE segment

**Commit sequence:** 1 commit (combine with H1)

---

### H3: 837 flat loop structure loses HL parent-child
**Scope:** `edi837/controller.rs`, loop2000 files
**Risk:** Multi-subscriber batches lose claim-to-subscriber association

**Plan:**
1. Parse HL segments into a tree: each HL has `id`, `parent_id`, `level_code`, `has_children`
2. Build the tree first, then walk it to associate claims with the correct subscriber/patient
3. Update the 837 output structs to nest patients under subscribers under billing providers
4. Test with a multi-subscriber demo file (create one based on the 837P spec example)

**Commit sequence:** 2 commits (tree parser + struct refactor)

---

### H4: Monetary/numeric fields are all `String`
**Scope:** All segment structs — CLP03/04, AMT02, BPR02, SVC fields, etc.
**Risk:** No validation, no arithmetic, no overflow protection

**Plan:**
1. **Phase 1 (validation only):** Add a `validate_numeric(value: &str, field: &str) -> EdiResult<()>` helper that checks the value matches X12 numeric format (N0, R, etc.) during parsing. Don't change struct types yet.
2. **Phase 2 (future):** Change monetary fields to `Option<Decimal>` using the `rust_decimal` crate. This is a large breaking change — defer until after C1-C6 are done.

**Commit sequence:** 1 commit for Phase 1

---

### H5: Add X12 envelope validation
**Scope:** All controllers + new validation module
**Risk:** Silently accepts structurally invalid files

**Plan:**
1. Add `validate_envelope()` function:
   - ST02 == SE02 (control numbers match)
   - SE01 == actual segment count between ST and SE
   - GS06 == GE02 (group control numbers match)
   - GE01 == actual transaction set count
   - ISA13 == IEA02 (interchange control numbers match)
   - IEA01 == actual functional group count
2. Call after parsing, before returning the result
3. Return `EdiError::ValidationError` on mismatch
4. Add tests for each validation rule

**Commit sequence:** 1 commit

---

### H6: Fix CLP off-by-one field mapping
**Scope:** `segments/clp.rs`
**Risk:** CLP10 (patient_status_code) is commented out, shifting CLP11-13 indices

**Plan:**
1. Add `clp10_patient_status_code: String` back to the struct
2. Fix index mapping: `clp_parts[9]` → CLP10, `clp_parts[10]` → CLP11, etc.
3. Currently `clp_parts[9]` maps to `clp11` — this is wrong per the X12 835 spec
4. Update test with a CLP segment that has all 14 elements

**Commit sequence:** 1 commit

---

### H7: Fix hardcoded segment ID length of 3
**Scope:** `edihelper.rs` — `get_segment_contents()`, `content_trim()`
**Risk:** 2-char segments (ST, SE, GS, GE, HL, N3, N4) may be mishandled

**Plan:**
1. Audit `get_segment_contents()` — it uses `key.len() + 1` to skip the ID, which is already dynamic. Verify this works for 2-char and 3-char segments.
2. Audit `get_loop_contents()` which hardcodes `contents.get(3..)` — fix to use `key.len() + 1`
3. Add tests for 2-char segment extraction

**Commit sequence:** 1 commit

---

## MEDIUM

### M1: Deduplicate 837 P/I/D code
**Scope:** `edi837/` — controller.rs, loop files
**Risk:** Maintenance burden, divergent bugs

**Plan:**
1. Extract shared 837 logic into `edi837/common.rs`: HL parsing, loop2000a/b, loop2010aa/ab/ac, loop2300, loop2400
2. P/I/D controllers call shared functions, only overriding subtype-specific segments (SV1 vs SV2 vs SV3, CL1, TOO)
3. Delete duplicate loop files

**Commit sequence:** 2 commits (extract common + delete duplicates)

---

### M2: Unify InterchangeHeader
**Scope:** 10 separate `interchangecontrol.rs` files
**Risk:** Inconsistent parsing across transaction types

**Plan:**
1. Create `src/segments/interchange.rs` with a single `InterchangeEnvelope` struct (ISA, GS, ST, SE, GE, IEA)
2. All transaction types use this shared struct
3. Delete 10 duplicate `interchangecontrol.rs` and `interchangecontroltrailer.rs` files

**Commit sequence:** 1 commit

---

### M3: Consolidate architectural patterns
**Scope:** Project-wide
**Risk:** Confusion, inconsistent behavior

**Plan:**
1. Pick one pattern: the `TransactionSet` trait approach (already defined)
2. Implement `TransactionSet` for all 10 transaction types
3. Route from `main.rs` through the trait
4. Delete raw-string and ad-hoc patterns

**Commit sequence:** 1 commit per transaction type

---

### M4: Wire up or delete `TransactionSet` trait
**Scope:** `transaction_processor.rs`, `main.rs`
**Risk:** Dead code confusion

**Plan:**
1. If M3 is done: wire `main.rs` to use the trait for routing
2. If M3 is deferred: delete the trait and `transaction_processor.rs`

**Commit sequence:** 1 commit

---

### M5: Replace `process::exit()` with `Result`
**Scope:** `helper/helper.rs` — 5 calls
**Risk:** Uncleanable process termination

**Plan:** Covered by C1 Phase 3. No separate work needed.

---

### M6: Remove `#[allow(dead_code)]` crutch
**Scope:** 77 instances across 54 files
**Risk:** Hides real dead code

**Plan:**
1. Remove all `#[allow(dead_code)]` annotations
2. Compile — fix or delete anything the compiler flags as dead
3. Re-add only for fields that are intentionally unused (e.g., reserved X12 elements)

**Commit sequence:** 1 commit

---

### M7: Deduplicate 834 loop2100b-h stubs
**Scope:** 7 identical 916-byte files in `edi834/`
**Risk:** Copy-paste maintenance

**Plan:**
1. Create a single generic `loop2100_stub.rs` with a parameterized function
2. Delete loop2100b.rs through loop2100h.rs
3. Update `mod.rs` and controller

**Commit sequence:** 1 commit

---

## LOW

### L1: Fix typos in public API names
**Scope:** `stiuational` → `situational`, `segement` → `segment`, `adjustsment` → `adjustment`, `numbner` → `number`, `scv04` → `svc04`
**Plan:** Find-and-replace across codebase. 1 commit.

---

### L2: Use `Option<String>` for optional fields
**Scope:** All segment structs
**Plan:** Change optional X12 elements from `String` (empty = absent) to `Option<String>`. Large change — defer until after C1/C2/C3. Affects serialization format (breaking change for JSON consumers).

---

### L3: Remove glob imports in `main.rs`
**Scope:** `main.rs` — `use crate::edi270::controller::*` etc.
**Plan:** Replace `*` with explicit imports. 1 commit.

---

### L4: Remove dead `once_cell` dependency
**Scope:** `Cargo.toml`
**Plan:** Remove from `[dependencies]`, verify build. 1 commit.

---

### L5: Remove commented-out code
**Scope:** Scattered across files
**Plan:** Delete all commented-out code blocks. If needed later, it's in git history. 1 commit.

---

## Execution Roadmap

Issues are ordered by dependency and impact. Each row can start after its prerequisites are done.

| Priority | Issue | Depends On | Est. Files | Est. Effort |
|----------|-------|------------|------------|-------------|
| 1        | C4: File size limits | — | 1 | Small |
| 2        | C6: Fix clean_contents | — | 1 | Small |
| 3        | H6: Fix CLP off-by-one | — | 1 | Small |
| 4        | H7: Fix segment ID length | — | 1 | Small |
| 5        | C7: Fix 837 detection | — | 2 | Small |
| 6        | L1: Fix typos | — | ~10 | Small |
| 7        | L4: Remove once_cell | — | 1 | Trivial |
| 8        | L5: Remove commented code | — | ~20 | Small |
| 9        | C3: Standardize segment ID | — | ~50 | Medium |
| 10       | C2: Fix stiuational_element | C3 | 9 | Medium |
| 11       | C1: Eliminate panics | C3 | ~60 | Large |
| 12       | C5: Eliminate clones | C1 | ~50 | Large |
| 13       | H1+H2: Fix segment finding | — | 4 | Small |
| 14       | H5: Envelope validation | C1 | 2 | Medium |
| 15       | H4: Numeric validation | C1 | ~45 | Medium |
| 16       | M2: Unify InterchangeHeader | C1 | ~12 | Medium |
| 17       | M7: Dedup 834 stubs | — | 8 | Small |
| 18       | M6: Remove dead_code attrs | C1 | ~54 | Small |
| 19       | M1: Dedup 837 P/I/D | M2 | ~15 | Large |
| 20       | H3: 837 HL tree structure | M1 | ~5 | Large |
| 21       | M3+M4: Consolidate arch | M1, M2 | ~15 | Large |
| 22       | L3: Remove glob imports | M3 | 1 | Trivial |
| 23       | L2: Option<String> fields | C1, C2 | ~50 | Large |

**Total estimated commits:** ~35-40
**Suggested milestone:** After items 1-14, the parser is safe for production input. Items 15+ are quality-of-life.
