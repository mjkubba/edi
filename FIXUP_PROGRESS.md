# EDI Test Fix Progress — wipfis837 branch

**Date**: 2026-04-25
**Branch**: wipfis837 (commit b6f995b)
**State**: 28 test failures, build passes, memory crash fixed

## Completed
- [x] Aborted stale rebase on main
- [x] Fixed 4 NM1 test compilation errors (committed as b6f995b)
  - src/edi270/loop2000a.rs — added id_code_qualifier
  - src/edi271/loop2115c.rs — added id_code_qualifier
  - src/edi276/controller.rs — added id_code_qualifier (2 places)
- [x] Memory crash fix already in place (count_segment_starts/find_next_segment_start in edihelper.rs, used by edi999 loops)

## Remaining Fixes (28 failures)

### Fix 1: write_nm1 trailing empty fields (affects ~5 tests)
**File**: `src/segments/nm1.rs` — `write_nm1()`
**Problem**: Outputs trailing `*` before `~` for empty fields (e.g., `NM1*PR*2*NAME*****PI*12345*~`)
**Fix**: Before the final `push_str("~")`, trim trailing `*`:
```rust
while nm1_content.ends_with('*') {
    nm1_content.pop();
}
nm1_content.push('~');
```
Also remove the hardcoded SMITH*MARY special case (lines 49-59) — the generic trim handles it.

### Fix 2: edi837 `..=end` off-by-one (14 tests)
**Files**: All edi837 loop files + controller.rs
**Problem**: Every segment extraction uses `remaining_content[pos..=end]` which includes the `~` terminator. Tests expect values without `~`.
**Fix**: Change `..=end` to `..end` in every segment extraction across:
- src/edi837/controller.rs (~14 occurrences across 3 parse impls)
- src/edi837/loop2000a.rs (2)
- src/edi837/loop2000b.rs (10)
- src/edi837/loop2000c.rs (10)
- src/edi837/loop2010aa.rs (5)
- src/edi837/loop2010ab.rs (4)
- src/edi837/loop2010ac.rs (5)
- src/edi837/loop2300.rs (12)
- src/edi837/loop2400.rs (10)

### Fix 3: PRV parser off-by-one (3 tests)
**File**: `src/segments/prv.rs` — `get_prv()`
**Problem**: Assigns `elements[0]` to `segment_id`, but `get_segment_contents("PRV",...)` already strips the `PRV*` prefix. So elements[0] is actually PRV01, causing all fields to shift.
**Fix**: Hardcode `segment_id = "PRV"` and use elements[0] for prv01, elements[1] for prv02, etc.

### Fix 4: UM prefix logic (3 tests)
**File**: `src/segments/um.rs` — `UM::new()`
**Problem**: Treats UM01 values like "AR"/"HS" as prefixes instead of request category codes. `get_segment_contents` already strips `UM*`, so parts[0] IS um01.
**Fix**: Remove prefix detection logic. Always put parts[0] into `um01_request_category_code`.
Also fix test `test_get_um_with_prefix`: change input from `AR*I*2*21:B*****Y` to `AR*I*2*21:B****Y` (one fewer `*`).

### Fix 5: Test assertion updates for NM1 field shift (4 tests)
These tests assert old field names after the id_code_qualifier addition:

- `src/edi271/loop2115c.rs` test_get_loop_2115c: change `id_code == "SV"` → `id_code_qualifier == "SV"`, `member_number == "0202034"` → `id_code == "0202034"`
- `src/edi278/loop2010a.rs` test_get_loop2010a: change `id_code == "PI"` → `id_code_qualifier == "PI"`, `member_number == "12345"` → `id_code == "12345"`
- `src/edi835/loop2100.rs` test_get_loop_2100: change `member_number == "98765432111"` → `id_code == "98765432111"`

### Fix 6: BHT test data / parser issues (2 tests)
- `src/edi276/controller.rs` test_parse_and_generate_276: Add bht06 to test BHT segment: `BHT*0010*13*12345*20230501*1200~` → `BHT*0010*13*12345*20230501*1200*13~`
- `src/edi277/table1.rs`: BHT parser requires `>= 7` elements for ALL fields — change to individual bounds checks
- `src/edi277/controller.rs` test: Add bht06: `BHT*0010*08*12345*20230501*1200~` → `BHT*0010*08*12345*20230501*1200*08~`

## After All Fixes
- [ ] Run `cargo.exe build` — verify no errors
- [ ] Run `cargo.exe test` — verify 0 failures
- [ ] Run `cargo.exe +nightly fmt` (if available) or format manually
- [ ] Clean up compiler warnings (49 warnings — mostly unused imports/dead code)
- [ ] Commit and verify ready for merge to main
