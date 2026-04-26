# Project Status

Last updated: 2026-04-25

## Build & Tests

- **235/235 tests passing**, zero failures
- **0 compiler warnings**
- Build toolchain: `cargo.exe` via WSL (project lives on Windows filesystem)

## Transaction Set Status

| Transaction Set | Status | Round-Trip Fidelity | Known Issues |
|---|---|---|---|
| 835 (Payment/Remittance) | ✅ Complete | Trailing newline only | None |
| 999 (Acknowledgment) | ✅ Complete | Minor diffs | AK9 segment reordered, trailing newline |
| 270 (Eligibility Inquiry) | ✅ Complete | Minor diffs | REF segment reordered, trailing newline |
| 271 (Eligibility Response) | ✅ Complete | Minor diffs | TRN/DTP reordered, LS/LE envelope added |
| 276 (Claim Status Request) | ✅ Complete | Minor gaps | AMT, DTP at subscriber level not captured |
| 277 (Claim Status Response) | ✅ Complete | Identical output | None |
| 278 (Services Review) | ✅ Complete | Trailing newline only | None |
| 837P (Claim Professional) | ✅ Complete | Trailing newline only | All segments preserved on round-trip |
| 837I (Claim Institutional) | ✅ Complete | Trailing newline only | All segments preserved on round-trip including CL1 |
| 837D (Claim Dental) | ✅ Complete | Trailing newline only | All segments preserved on round-trip including TOO |
| 820 (Premium Payment) | ✅ Complete | Trailing newline only | None |
| 834 (Enrollment) | ✅ Complete | Identical output | Loop2320/2330 are stubs |

## What Needs Work

### High Priority

1. **EDI834 — Loop2320/2330 stubs**
   - `loop2320` and `loop2330` are stub implementations (empty write functions)
   - Coordination of benefits data not preserved
   - KB has full spec: `834v5010X220.md`

### Medium Priority

2. **EDI276 — AMT/DTP at subscriber level**
   - AMT, DTP segments at subscriber HL level not captured on round-trip

### Low Priority

3. **Generic infrastructure partially unused**
   - `transaction_processor.rs`, `segment_config.rs`, `loop_processor.rs` have functions that are dead code
   - Decide: wire them in or remove them

4. **Performance optimization** for large files
5. **Custom delimiter support**
6. **Schema validation**

## Completed Work

- All transaction set modules created (835, 999, 270/271, 276/277, 278, 837P/I/D, 820, 834)
- 235 unit tests passing
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
- Eliminated all compiler warnings (26 → 0)
  - Removed dead loop2010ba/bb/ca files and unused parse/write functions
  - Suppressed warnings on public API and infrastructure kept for future use
- Fixed: edi835 segment serialization
  - Added build_segment() helper for X12 §3.7 compliant trailing separator suppression
  - Fixed TS3 parser element position mapping (TS306-TS312 are NOT USED per TR3)
  - Converted TS3, TS2, MIA, SVC, PLB writers to use build_segment()
  - Fixed demo file TS3 line with data at NOT USED positions
  - 835 round-trip now trailing-newline-only

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
