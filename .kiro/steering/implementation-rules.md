---
inclusion: always
---

# Rust Implementation Rules

When you implement new code, keep the current structure, mimic what was already created, don't create a new structure.
Make sure the changes to an existing code or file will not impact the existing functionality and keep the code backward compatible.

## X12 Reference Documentation

Before implementing a new transaction set, segment, or loop — or when something about the X12 structure is unclear — **search the knowledge base first**. It contains the full X12 implementation guides for all supported transaction sets:

- 270/271 (Eligibility) — `270-271v5010X279.md`
- 276/277 (Claim Status) — files with `005010X212` in the name
- 278 (Services Review) — `278v5010X217.md`
- 820 (Premium Payment) — `820v5010X218.md`
- 834 (Enrollment) — `834v5010X220.md`
- 835 (Payment/Remittance) — `835v5010X221.md`
- 837P (Claim Professional) — `837Pv5010X222.md`
- 837I (Claim Institutional) — files with `005010X223` in the name
- 837D (Claim Dental) — `837Dv5010X224.md`
- 999 (Acknowledgment) — `999v5010X231.md`

These guides contain segment element definitions, loop hierarchies, situational rules, and valid code values. Use them to verify correct field mappings and segment ordering.

## EDI Segment Parsing Rules

1. **`get_segment_contents(key, contents)` strips the segment ID prefix** (e.g., `get_segment_contents("NM1", ...)` returns content AFTER `NM1*`). When callers split the result on `*`, index 0 = first data element (e.g., NM101). This contract is consistent across the entire codebase (audited 2026-04-27).

2. **The edi837 module stores raw segment strings WITH the `~` terminator** (e.g., `"HL*2*1*22*0~"`). This is by design — the write functions output the stored value directly followed by `\n`. All other modules use parsed structs (NM1, HL, etc.) and the write functions add `~`. Do not mix these patterns.

3. **Loop boundary detection must include ALL segment types that start a new loop or section.** When parsing segments in a loop (e.g., REF, DTP), the boundary check must include not just `HL*` and `NM1*` but also `CLM*`, `LX*`, `SE*`, and any other segment that signals the current loop has ended. Missing boundaries causes the parser to consume segments belonging to the next loop.

4. **Segment parse order must match the expected X12 segment order in the data.** If REF appears before AMT in the EDI data, parse REF before AMT. Using `find()` to locate a segment skips over everything between the current position and the found segment — any unparsed segments in between are lost when `remaining_content` advances.

5. **Use `count_segment_starts()` and `find_next_segment_start()` from edihelper** when counting or finding segments. Naive `contents.matches("IK3").count()` can match segment IDs inside other segment data values (e.g., "IK3" inside a CTX value), causing infinite loops or incorrect splitting.

6. **Prefer sequential segment processing over `find()` for loops with many segment types.** The `parse_loop2300` rewrite proved that iterating segment-by-segment (split on `~`, match prefix) is more reliable than calling `find()` for each segment type. `find()` skips over unrecognized segments between the cursor and the match, silently losing data.

7. **NOT USED elements still occupy their position in the segment.** Per X12 §B.1.1.3.10, only **trailing** empty element separators may be suppressed. Middle empty elements must keep their `*` separators to preserve positional meaning. Use `build_segment()` from edihelper which handles this correctly — it joins all elements with `*` then trims trailing empties.

8. **NOT USED elements in the spec do NOT remove the position from the segment.** If TS306 is NOT USED and TS313 is SITUATIONAL, TS313 is still at position 12 (not position 5). The parser must read from the correct X12 position index, not skip NOT USED positions.

9. **Demo files are AI-generated and may contain errors.** Always verify segment content against the X12 spec in the knowledge base before assuming the demo file is correct. If a round-trip diff occurs, check whether the demo or the code is wrong.

10. **Write functions must output segments in spec-defined position order.** The X12 implementation guide defines a position number for each segment within a loop (e.g., TRN at pos 0200, NM1 at pos 0300). Writers must follow this order even if the parser stored them differently.

11. **Use boundary-aware segment matching.** Never use bare `.find("SE")` or `.contains("HL")` — these match inside data values (e.g., "SERVICES" contains "SE"). Always append the element separator: `.find("SE*")`, `.contains("HL*")`. For robust matching, use `find_next_segment_start()` from edihelper which checks for `~` or start-of-content before the segment ID.

12. **All segment parsers must use safe element access.** Use `get_element(&parts, index)` from edihelper instead of direct `parts[index]`. EDI segments may have fewer elements than the maximum (trailing optionals omitted per §B.1.1.3.10). Direct indexing panics on short segments.

13. **ISA/GS/GE/IEA envelope segments are identical across all transaction types.** The control segments use the same struct definitions, same element names, same required/situational designators. Use the shared structs from `segments/isa.rs`, `segments/gs.rs`, `segments/se.rs`, `segments/ge.rs`, `segments/iea.rs`. Do not create local copies.

14. **X12 delimiter replacement in `clean_contents()` is correct per spec.** Per X12 §B.1.1.2, delimiter characters must not appear inside data element values within the interchange. Global `replace()` of custom delimiters to standard `*` and `~` is safe.

15. **837 P/I/D share structure above Loop 2400.** Loops 1000A/B, 2000A/B/C, 2010AA/AB/AC/BA/BB/CA, 2300, 2320, 2330 are structurally identical across Professional, Institutional, and Dental. Differences: 837P has UPIN REF in 2010AA; 837I has CL1 in 2300; DTP qualifiers vary per subtype; Loop 2400 service lines are completely different (SV1 vs SV2 vs SV3+TOO).

16. **837 uses a single `Edi837` struct with a `subtype` enum.** The old `Edi837P`/`Edi837I`/`Edi837D` types are aliases to `Edi837`. Subtype is auto-detected from the version identifier (X222/X223/X224). Do not create separate structs for subtypes.

17. **837 claims are nested under their subscriber/patient via the HL tree.** `Loop2000b` (subscriber) contains `loop2000c: Vec<Loop2000c>` (patients) and `loop2300: Vec<Loop2300>` (claims when subscriber=patient). `Loop2000c` (patient) contains `loop2300: Vec<Loop2300>` (claims for that patient). Claims are NOT stored as flat vectors on the top-level `Edi837` struct.

## File Operations

When working with file operations in Rust:

1. Prefer using the simpler `fs::read_to_string()` and `fs::write()` functions over verbose `File::open()` + `read_to_string()` or `File::create()` + `write_all()` combinations
2. Avoid the `#[allow(clippy::verbose_file_reads)]` annotation by using the recommended methods
3. Use `serde_json::to_string_pretty()` + `fs::write()` instead of creating a file and then writing to it with `serde_json::to_writer_pretty()`
4. Keep imports organized by functionality (e.g., group path-related imports together)

## Git Best Practices

### Committing Changes

Follow the git best practice of committing early and often. Run `git commit` often, but DO NOT ever run `git push`.

BEFORE committing a change, ALWAYS do the following steps:

1. Run `cargo build` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
2. Run `cargo test` and fix any problems. Prefer running it against just the crate you're modifying for shorter runtimes
3. Run `cargo +nightly fmt` to auto-format the code
4. Commit the changes

### Pushing Changes

The primary remote for pushing is `origin` using HTTPS. Always push to `main`:

```bash
git push origin main
```

**Use `git.exe push` instead of `git push`** — the WSL `git` hangs on auth prompts, but `git.exe` uses Windows credential manager and works. Use `git.exe` for any git operation that needs authentication (push, pull, fetch from private repos).

### Commit Messages

All commit messages should follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]

🤖 Assisted by [Kiro](https://kiro.dev)
```

Types:
- feat: A new feature
- fix: A bug fix
- docs: Documentation only changes
- style: Changes that do not affect the meaning of the code
- refactor: A code change that neither fixes a bug nor adds a feature
- perf: A code change that improves performance
- test: Adding missing tests or correcting existing tests
- chore: Changes to the build process or auxiliary tools
- ci: Changes to CI configuration files and scripts

Best practices:
- Use the imperative mood ("add" not "added" or "adds")
- Don't end the subject line with a period
- Limit the subject line to 50 characters
- Capitalize the subject line
- Separate subject from body with a blank line
- Use the body to explain what and why vs. how
- Wrap the body at 72 characters

Example:
```
feat(lambda): Add Go implementation of DDB stream forwarder

Replace Node.js Lambda function with Go implementation to reduce cold
start times. The new implementation supports forwarding to multiple SQS
queues and maintains the same functionality as the original.

🤖 Assisted by [Kiro](https://kiro.dev)
```