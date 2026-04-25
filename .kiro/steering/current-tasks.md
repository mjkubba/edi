---
inclusion: manual
---

# Current Development Tasks

## Completed
- [x] All transaction sets implemented (835, 999, 270/271, 276/277, 278, 837P/I/D, 820, 834)
- [x] Fix all test failures (237/237 pass)
- [x] Fix memory crash in edi999 CTX parsing
- [x] Fix NM1 id_code_qualifier field across all modules
- [x] Fix UM prefix detection (removed — AR/HS are um01 values)
- [x] Fix PRV parser off-by-one
- [x] Fix edi837 loop boundary detection and write functions
- [x] Fix edi276/277 HL/NM1/BHT parsing
- [x] Fix edi278 loop2010f to accept NM1*1P

## Remaining Work

### High Priority
- [ ] Implement Loop2000C/D parsing for EDI276/277
  - TRN, REF, DMG segments not parsed/written for Provider and Subscriber levels
- [ ] Enhance EDI820 implementation
  - Fix missing segments (N1, ENT, NM1, RMR, DTM)
- [ ] Verify EDI834 against real files

### Medium Priority
- [ ] Clean up compiler warnings (unused imports, dead code, unused mut)
- [ ] Improve EDI837P/I segment coverage on round-trip

### Low Priority
- [ ] Performance optimization for large files
- [ ] Add support for custom delimiters
- [ ] Add schema validation
