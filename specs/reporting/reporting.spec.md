---
module: reporting
version: 5
status: active
files:
  - src/timing.rs
  - tests/timing.rs

db_tables: []
depends_on:
  - cli
---

# Chough timing and reporting reads

## Purpose

Own Chough's read-only timing report and the request-local reads behind its reports:
how long stages, tests, bug fixes and human gates took, derived from existing
records; bounded, deduplicated native reads within one timing report; and at most
one case-suite load per report request. This module was split from cli on
2026-11-04 so each requirements companion stays within the 65,536-byte record
bound; REQ-reporting-001 to REQ-reporting-003 keep the exact wording of
REQ-cli-020, REQ-cli-024 and REQ-cli-028 (mapping in context.md and docs/FRD.md).
See requirements.md.

## Public API

The binary exposes CLI commands; it does not export a Rust library API. This module
covers the `timing` command and the request-local reporting reads that `next` and
`status` use, as documented in docs/cli.md. The cli module owns the command parser,
the checkpoint, stage moves and every other command group.

## Invariants

1. The timing report writes no record; every Chough and SpecSync record stays
   byte-identical.
2. Durations are elapsed wall-clock time, not effort; a missing, unparseable or
   out-of-order timestamp is unknown, never zero.
3. Native reads within one report are reused only within that request and are
   bounded by one whole-report budget; stage moves, `next` and `status` keep their
   single native read and their gates.
4. Reporting reads cache nothing across requests and introduce no persistent cache,
   new runtime or duplicate lifecycle state.

## Behavioral Examples

```gherkin
Given fixture transitions with fixed timestamps
When chough timing runs
Then it reports stage segments, waiting-on-human time apart from active time, and writes no record

Given the checkpoint change also present in the archive
When an unfiltered timing report runs
Then specsync change status runs once for that change and at most ten archived reads in total

Given several observations and referenced labels
When one next or status request runs
Then the case-definition suite is loaded at most once
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Invalid native budget | `CHOUGH_TIMING_NATIVE_BUDGET_SECONDS` is not a whole number from 1 to 3600 | Refuse before any native read |
| Budget exhausted | No budget remains for a native read | Report it `not-read` with reason `budget-exhausted` and unknown times |
| Missing or out-of-order timestamp | A record's time is missing or ends before it starts | Report the duration unknown, never zero |
| Input beyond its bound | A timing input exceeds its bound | Report the result partial |

## Dependencies

Depends on the cli module for the command parser, checkpoint, records, case
definitions, bounded reads and native status reader, and on the history module for
transition events. Git and SpecSync supply revisions and native lifecycle times.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-11-04 | Split from cli with REQ-cli-020, REQ-cli-024 and REQ-cli-028 renamed REQ-reporting-001 to REQ-reporting-003, wording unchanged |
| 2 | 2026-11-04 | split-the-cli-spec-move-the-timing-and-reporting-requirements-req-cli-020-req-cli-024-and-req-cli-028-to-a-new: Split the cli spec: move the timing and reporting requirements REQ-cli-020, REQ-cli-024 and REQ-cli-028 to a new reporting module as REQ-reporting-001 to REQ-reporting-003 with wording unchanged and a documented ID mapping, so the cli requirements companion stays under the 65536-byte record bound |
| 3 | 2026-11-04 | read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an: Read HI exports and native SpecSync files at full size for every gate and validation so Chough bounds never push an agent to edit human intent or specs (issue 100) |
| 4 | 2026-11-06 | make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive: Make the timing trace test fail only when tracing changes the report, never because its runs straddle a second (169, DRIVE-19) |
| 5 | 2026-11-07 | chough-timing-skips-direct-native-write-events-without-marking-its-report-partial-189-drive-21: chough timing skips direct-native-write events, and chough change new passes --no-spec-change (189, DRIVE-21, DRIVE-20) |
