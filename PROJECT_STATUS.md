# Project Status

Last updated: 2026-04-25

## Build & Tests

- **238/238 tests passing**, zero failures
- **26 compiler warnings** (edi837 dead code, EdiError variants, infrastructure kept for future use)
- Build toolchain: `cargo.exe` via WSL (project lives on Windows filesystem)

## Transaction Set Status

| Transaction Set | Status | Round-Trip Fidelity | Known Issues |
|---|---|---|---|
| 835 (Payment/Remittance) | ✅ Complete | Identical output | None |
| 999 (Acknowledgment) | ✅ Complete | Minor diffs | GE/IEA control numbers differ on round-trip |
| 270 (Eligibility Inquiry) | ✅ Complete | Line break diffs only | Formatting only |
| 271 (Eligibility Response) | ✅ Complete | Line break diffs only | Formatting only |
| 276 (Claim Status Request) | ✅ Complete | Minor gaps | AMT, DTP at subscriber level not captured |
| 277 (Claim Status Response) | ✅ Complete | Identical output | None |
| 278 (Services Review) | ⚠️ Functional | Minor diffs | DTP, SV2, PRV segments missing in output |
| 837P (Claim Professional) | ✅ Complete | Trailing newline only | All segments preserved on round-trip |
| 837I (Claim Institutional) | ✅ Complete | Trailing newline only | All segments preserved on round-trip including CL1 |
| 837D (Claim Dental) | ✅ Complete | Trailing newline only | All segments preserved on round-trip including TOO |
| 820 (Premium Payment) | ✅ Complete | Identical output | None |
| 834 (Enrollment) | ⚠️ Functional | Partial | Loop1000B boundary fixed; Loop2320/2330 are stubs |

## What Needs Work

### High Priority

1. **EDI834 — Verified and partially fixed**
   - Code exists in `src/edi834/` with controller, loops, segments, and `main.rs` wiring
   - Loop1000B boundary detection fixed — no longer consumes INS/REF/DTP belonging to Loop2000
   - `loop2320` and `loop2330` are stub implementations (empty write functions, unused variables)
   - Demo file available: `demo/edi834-demo-005010X220.edi`
   - KB has full spec: `834v5010X220.md`

2. **EDI820 — Fix missing segments**
   - N1, ENT, NM1, RMR, DTM not preserved on round-trip
   - `loop2000`/`loop2100` parsers need to capture all child segments
   - KB has full spec: `820v5010X218.md`

3. **EDI276/277 — Implement Loop2000C/D**
   - TRN, REF, DMG segments at Provider and Subscriber HL levels not parsed/written
   - Data loss on round-trip for multi-level HL hierarchies

### Medium Priority

4. **Clean up remaining 26 compiler warnings**
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
- 238 unit tests passing
- Fixed: memory crash in edi999 CTX parsing
- Fixed: NM1 id_code_qualifier field across all modules
- Fixed: UM prefix detection (AR/HS are um01 values, not prefixes)
- Fixed: PRV parser off-by-one
- Fixed: edi837 loop boundary detection and write functions
- Fixed: edi276/277 HL/NM1/BHT parsing
- Fixed: edi278 loop2010f to accept NM1*1P
- Fixed: edi820 off-by-one indexing in all 7 parser files — round-trip now identical
- Fixed: edi834 Loop1000B boundary detection consuming INS/REF/DTP from Loop2000
- Fixed: edi834 controller premature break after first member
- Fixed: edi834 NM1 offset in all 8 loop2100 files
- Fixed: edi834 cross-member segment consumption with in_current_member()
- Fixed: edi276/277 missing GS/GE segment handling
- Fixed: edi276/277 Loop2000C (Service Provider) and Loop2000D (Subscriber) parsing
- Fixed: edi277 hardcoded TRN/STC/REF segments replaced with parsed data
- Reduced compiler warnings from 52 to 26
- Created demo files for all 12 transaction sets
- Fixed: edi837P/I/D round-trip — all segments now preserved
  - Rewrote parse_loop2300 with sequential segment processing (eliminates find() ordering bugs)
  - Added NM1*PR (payer) and DMG parsing to Loop2000B
  - Added DMG parsing after NM1*QC in Loop2000C
  - Added NM1*82/71/72 (rendering/attending/operating provider) and PRV*PE to Loop2300
  - Added TOO segment parsing to Loop2400 for 837D
  - Fixed envelope segment double-tilde in write_837p/i/d
  - Fixed parse_loop2000a PRV boundary (was consuming claim-level PRV)
  - Removed redundant CL1/TOO parsing from 837I/D controllers

## Recommended Next Steps

1. ~~Get demo 834 files → debug and verify edi834 parsing~~ ✅ Done
2. ~~Fix edi820 segment coverage (N1, ENT, NM1, RMR, DTM)~~ ✅ Done
3. ~~Implement edi276/277 Loop2000C/D parsing~~ ✅ Done
4. ~~Wire up edi837 dead code (loop2010ba/bb/ca write functions)~~ ✅ Done — rewrote with inline parsing
5. Clean up compiler warnings

## Demo Files

All demo files are in `demo/`. These files are AI-generated based on public X12 implementation guides and EDI specification documentation. They are intended for testing and development purposes only.

| File | Transaction Set |
|------|----------------|
| `edi835-demo-005010X221.edi` | 835 Payment/Remittance |
| `edi999-demo-005010X231.edi` | 999 Acknowledgment |
| `edi270-demo-005010X279.edi` | 270 Eligibility Inquiry |
| `edi271-demo-005010X279.edi` | 271 Eligibility Response |
| `edi276-demo-005010X212.edi` | 276 Claim Status Request |
| `edi277-demo-005010X212.edi` | 277 Claim Status Response |
| `edi278-demo-005010X217.edi` | 278 Services Review |
| `edi837P-demo-005010X222.edi` | 837P Professional Claim |
| `edi837I-demo-005010X223.edi` | 837I Institutional Claim |
| `edi837D-demo-005010X224.edi` | 837D Dental Claim |
| `edi820-demo-005010X218.edi` | 820 Premium Payment |
| `edi834-demo-005010X220.edi` | 834 Enrollment |
