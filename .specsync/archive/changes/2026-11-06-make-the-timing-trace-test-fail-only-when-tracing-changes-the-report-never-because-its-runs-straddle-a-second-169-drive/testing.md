---
change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
artifact: testing
---

# Testing

- **Failing first:** the fixture's native stub gains an optional `.stub-delay-<id>`. The test sets a 1.1 s delay on the current change, so every `timing` run takes over a second and the plain and traced runs always straddle a second boundary. With today's comparison the test fails every time ("traced and untraced reports differ", on Linux at the definition commit).
- **Fix:**
  - The test runs each report once, with no retry.
  - It requires the traced report to equal the plain one, except that each time (`unix_seconds`) and span (`seconds`) may be later by exactly `shift`, the gap between the two `generated_at` values. Labels, reasons, unknowns, keys, array lengths and every other field must be equal.
  - Its other assertions (stderr, trace lines, native read count, phases, text mode, the refused budget) stay unchanged.
- **Strictness:** the test also checks that the comparison rejects a report with one non-time field changed, and one compared with the wrong shift.
- `fledge lanes run verify` passes on Linux.

The test checks REQ-reporting-002's reworded criterion: identical except for the clock, with each time and span that runs to now later by exactly the gap between the two runs.

`tests/spec_split.rs` pins a digest of each requirement moved in the reporting split. REQ-reporting-002's digest is updated to the reworded text, with a comment naming this change, as #77 did for its modified requirement. The guard still catches any change that is not approved.
