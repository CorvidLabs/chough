---
change: make-chough-timing-read-each-native-change-at-most-once-skip-archived-history-the-requested-report-does-not-show-bound
artifact: testing
---

# Testing

This is the verification plan, not a record that these checks have run.

| Requirement | Evidence |
|---|---|
| REQ-cli-023, TC-cli-0089 (extended) | `tests/timing.rs`: a stub `specsync` on PATH logs each argv. Unfiltered report: one read of the checkpoint change, at most ten archived reads, identical current and history entries. Each of `--cycle current`, `--stage test`, `--case`, `--bug`: one read, `history` null, `history_selection` `skipped-by-filter` with the filter named, text line present, estimates identical to the unfiltered report. |
| REQ-cli-023, TC-cli-0100 (extended) | Stub modes sleep, hang (with a child holding the pipes), exit 1 and print malformed JSON, with `CHOUGH_TIMING_NATIVE_BUDGET_SECONDS=2`: bounded elapsed time, explicit states, `not-read`/`budget-exhausted` entries with unknown times, no surviving stub descendants, and invalid budget values refused with zero stub invocations. |
| REQ-cli-023, named regressions in tests/timing.rs | Trace on: one stderr line per read and per phase, stdout byte-identical. Trace off: both streams byte-identical. Stage move with a failing/timing-out stub and budget 1 is refused. `next` and `status` each log exactly one stub read. Records byte-identical after timing. |
| Real native | Release build against SpecSync 6.1.2 on a clone of the same base: completed versus timed-out reads reported as observed; no claim beyond the measured inputs. |

Environments: local-macos-arm64 observed; ci-linux and ci-windows only when observed.

No new case number is allocated. Observation IDs are explicit, for example `tc-cli-0089-native-dedupe-b35acaf-pre-fix`, so they do not depend on the local second-based generator that #30 addresses.
