# Fix Plan: C1 — Eliminate Panics on Untrusted Input

## Problem
910 direct-index accesses (`[n]`) and 87 `.unwrap()` calls across 107 files. Any malformed EDI input crashes the process. This is a DoS vector for a healthcare parser that will inevitably receive garbage.

## Scope
- **910** array index accesses (panic on out-of-bounds)
- **87** `.unwrap()` calls (panic on None/Err)
- **5** `process::exit()` calls in `helper/helper.rs`
- **~60** source files affected

## Strategy: Bottom-Up, Three Phases

### Phase 1: Safe Element Access Helper (foundation)
**Files:** `src/helper/edihelper.rs`
**Effort:** Small

Add a single helper function that all segment parsers will use:

```rust
/// Safely get element at index, returning empty string if missing
pub fn get_element(parts: &[&str], index: usize) -> String {
    parts.get(index).unwrap_or(&"").to_string()
}

/// Safely get required element, returning Err if missing
pub fn get_required_element(parts: &[&str], index: usize, segment: &str, field: &str) -> EdiResult<String> {
    parts.get(index)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .ok_or_else(|| EdiError::MissingField(format!("{} {} at position {}", segment, field, index)))
}
```

This is the single change that makes everything else mechanical.

### Phase 2: Segment Parsers (bulk of the work)
**Files:** All `src/segments/*.rs` (~45 files)
**Effort:** Large but mechanical

Two patterns exist in the codebase. Convert both:

**Pattern A — direct index (panics):**
```rust
// BEFORE: panics if < 2 elements
amt01: amt_parts[0].to_string(),
amt02: amt_parts[1].to_string(),
```
```rust
// AFTER: returns error or empty string
amt01: get_required_element(&parts, 0, "AMT", "amt01")?,
amt02: get_required_element(&parts, 1, "AMT", "amt02")?,
```

**Pattern B — .get() with fallback (safe but inconsistent):**
```rust
// BEFORE: safe but verbose
if n4_parts.get(1).is_some() {
    payee_state = n4_parts[1].to_string();
}
```
```rust
// AFTER: clean
payee_state: get_element(&parts, 1),
```

**Conversion order** (by panic density, highest first):
1. `stc.rs` (40), `ins.rs` (34) — worst offenders
2. `eb.rs`, `hi.rs`, `mia.rs` (24 each)
3. `hd.rs` (22), `sv2.rs`, `um.rs` (20 each)
4. `cas.rs`, `ts2.rs` (19 each), `dmg.rs` (18)
5. `bgn.rs`, `bpr.rs`, `dsb.rs`, `hsd.rs`, `isa.rs` (16 each)
6. All remaining segments

**Each segment parser change:**
1. Change return type: `get_xxx(content: String) -> XXX` → `get_xxx(content: &str) -> EdiResult<XXX>`
2. Replace `[n]` with `get_element()` / `get_required_element()`
3. Update callers

### Phase 3: Controllers & Main (propagate Result)
**Files:** `src/main.rs`, all `controller.rs`, loop files, `helper/helper.rs`
**Effort:** Medium

1. **`helper/helper.rs`**: Replace 5 `process::exit()` calls with `Result` returns
2. **Controllers**: Change parse functions to return `EdiResult<T>`, propagate `?`
3. **`main.rs`**: Replace 25 `.unwrap()` calls with match/`?`, print user-friendly errors on Err

### Phase 4: Verify
1. `cargo test` — all existing tests pass
2. Test with each demo file — same output as before
3. Test with truncated/malformed files — graceful error instead of panic
4. Test with empty file, file with only ISA, file with garbage

## File Change Estimates

| Phase | Files | Index fixes | Unwrap fixes |
|-------|-------|-------------|--------------|
| 1     | 1     | 0           | 0            |
| 2     | ~45   | ~750        | ~30          |
| 3     | ~15   | ~160        | ~57          |
| 4     | 0     | 0           | 0            |
| **Total** | **~61** | **~910** | **~87** |

## Commit Strategy
One commit per phase. Each phase compiles and tests pass before moving on.

- `fix(core): add safe element access helpers`
- `fix(segments): replace panics with Result in all segment parsers`
- `fix(controllers): propagate Result through controllers and main`
- `test: add malformed input tests`

## Risk
Phase 2 is the largest but most mechanical — every file follows the same pattern. The real risk is Phase 3 where controller logic has to thread `Result` through complex loop-parsing state machines. Take extra care with the 837 controller (39KB, most complex).
