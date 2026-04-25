# Project Status

Last updated: 2026-04-25

## Build & Tests

- **237/237 tests passing**, zero failures
- **52 compiler warnings** (25 unused functions, 16 unused imports, 4 unused mut, misc)
- Build toolchain: `cargo.exe` via WSL (project lives on Windows filesystem)

## Transaction Set Status

| Transaction Set | Status | Round-Trip Fidelity | Known Issues |
|---|---|---|---|
| 835 (Payment/Remittance) | ✅ Complete | Identical output | None |
| 999 (Acknowledgment) | ✅ Complete | Minor diffs | GE/IEA control numbers differ on round-trip |
| 270 (Eligibility Inquiry) | ✅ Complete | Line break diffs only | Formatting only |
| 271 (Eligibility Response) | ✅ Complete | Line break diffs only | Formatting only |
| 276 (Claim Status Request) | ⚠️ Functional | Data loss | Loop2000C/D not parsed — TRN, REF, DMG missing at Provider/Subscriber levels |
| 277 (Claim Status Response) | ⚠️ Functional | Data loss | Same Loop2000C/D gap; STC values differ |
| 278 (Services Review) | ⚠️ Functional | Minor diffs | DTP, SV2, PRV segments missing in output |
| 837P (Claim Professional) | ⚠️ Functional | Significant gaps | Missing: NM1*41, PER, NM1*40, DMG, NM1*PR, CR1, CRC, NM1*PW, NM1*45, LX, SV1, QTY, NTE. Dead code: `write_loop2010ba/bb/ca` exist but aren't called |
| 837I (Claim Institutional) | ⚠️ Functional | Significant gaps | Missing: NM1*41, PER, NM1*40, DMG, NM1*PR, CL1, NM1*71, SBR, OI, LX, SV2, DTP |
| 837D (Claim Dental) | ✅ Functional | Minor diffs | Core functionality working |
| 820 (Premium Payment) | ⚠️ Partial | Major gaps | Missing segments: N1, ENT, NM1, RMR, DTM |
| 834 (Enrollment) | ❌ Unverified | Unknown | Code exists but was not recognized in May 2025 testing. Loop2320/2330 are stubs. No demo files to test against |

## What Needs Work

### High Priority

1. **EDI834 — Debug and verify**
   - Code exists in `src/edi834/` with controller, loops, segments, and `main.rs` wiring
   - May 2025 test: "format is not recognized" — detection or parsing bug
   - `loop2320` and `loop2330` are stub implementations (empty write functions, unused variables)
   - `loop1000b` detection logic looks fragile (`N1*IN*` checked for both 1000A and 1000B)
   - Need demo 834 files to test against
   - KB has full spec: `834v5010X220.md`

2. **EDI820 — Fix missing segments**
   - N1, ENT, NM1, RMR, DTM not preserved on round-trip
   - `loop2000`/`loop2100` parsers need to capture all child segments
   - KB has full spec: `820v5010X218.md`

3. **EDI276/277 — Implement Loop2000C/D**
   - TRN, REF, DMG segments at Provider and Subscriber HL levels not parsed/written
   - Data loss on round-trip for multi-level HL hierarchies

### Medium Priority

4. **EDI837P/I — Wire up dead code and add missing segments**
   - `write_loop2010ba`, `write_loop2010bb`, `write_loop2010ca` exist but are never called
   - Many segments missing on round-trip (see table above)

5. **Clean up 52 compiler warnings**
   - Most are auto-fixable: `cargo fix --lib -p edi` handles unused imports/mut
   - 25 unused functions — some are dead code, some are from the generic `transaction_processor`/`segment_config` infrastructure that isn't fully wired in

### Low Priority

6. **Generic infrastructure partially unused**
   - `transaction_processor.rs`, `segment_config.rs`, `loop_processor.rs` have functions (`register_common_segments`, `parse_isa`, `parse_gs`, etc.) that are dead code
   - Decide: wire them in or remove them

7. **Performance optimization** for large files
8. **Custom delimiter support**
9. **Schema validation**

## Completed Work

- All transaction set modules created (835, 999, 270/271, 276/277, 278, 837P/I/D, 820, 834)
- 237 unit tests passing
- Fixed: memory crash in edi999 CTX parsing
- Fixed: NM1 id_code_qualifier field across all modules
- Fixed: UM prefix detection (AR/HS are um01 values, not prefixes)
- Fixed: PRV parser off-by-one
- Fixed: edi837 loop boundary detection and write functions
- Fixed: edi276/277 HL/NM1/BHT parsing
- Fixed: edi278 loop2010f to accept NM1*1P

## Recommended Next Steps

1. Get demo 834 files → debug and verify edi834 parsing
2. Fix edi820 segment coverage (N1, ENT, NM1, RMR, DTM)
3. Implement edi276/277 Loop2000C/D parsing
4. Wire up edi837 dead code (loop2010ba/bb/ca write functions)
5. Clean up compiler warnings
