# Lesson bundle: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19)
- **Kind**: BugFix
- **Specs**: reporting
- **Paths**: tests/timing.rs, hi/drive.md, CHANGELOG.md, specs/reporting/requirements.md, tests/spec_split.rs
- **Acceptance**: DRIVE-19 (Leif, relayed by orc 2026-11-06, 'Capture both', then 'Record both, reword the AC'; chough#169). REQ-reporting-002's trace criterion is reworded from 'byte-identical to the untraced run' to 'identical to the untraced run except for the clock'; no other text changes. timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit checks that criterion. It runs plain and traced timing --json once each, with no retry, and they must be equal except that every time and span running to now is later by exactly the gap between the two runs' generated_at. With a native stub that makes each run take over a second, the old comparison fails every time and the new one passes. A mutated field or a wrong shift fails the comparison. No other test, gate, bound or timeout changes.

## Evidence

- Verification commit: `fd4b060e41dbb9516371b8990e7cb4d2bdc6800f`
- Base commit: `167790489735b3e6a88ec21a20bb2530d4c96138`
- Verified by: `specsync check --spec history --spec reporting`

## From the change's context.md

# Context

Issue #169. In the auditor's verify-lane run at 1030252c, `tests/timing.rs::timing_trace_reports_native_reads_and_phases_without_changing_output_or_exit` failed with "traced and untraced reports differ". It passed on replay. The test compares a plain and a traced `timing --json` byte for byte. Each report ends its ongoing segments at its own `generated_at`, so the two match only when both runs land in the same second. Five retries make a mismatch less likely, but cannot rule it out on a loaded host.

Leif captured DRIVE-19 ("Capture both"), in the wording of spec-sync's CI-7: "A failing test always means something is actually wrong, never that it ran at an unlucky moment."

Issue #30 rules out a runtime clock switch, which could forge observation times, so the fix stays in the test.

Leif then asked to reword the criterion ('Record both, reword the AC'), relayed by the orc on 2026-11-06 with a send-back. REQ-reporting-002's acceptance criterion said the traced standard output is "byte-identical to the untraced run", which two runs in different seconds cannot be. It now says "identical to the untraced run except for the clock": this change's one MODIFIED line, in `deltas/reporting.md`.

## From the change's testing.md

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

## Where these lessons go

- `specs/reporting/context.md`
