---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: testing
---

# Testing

Functional track only; nothing visible or copy-related changes beyond one status
note.

| Case | Test | Verifies |
|---|---|---|
| TC-cli-0038 | `tests/stages.rs::large_native_status_still_gates_by_its_summary_state` | A stubbed archived response of more than 16 KiB lets accept advance to done with `native:archived`; status reports `observed` with `native_summary.state` archived; a response over 4 MiB still refuses as `native-unknown` (REQ-cli-008) |
| TC-cli-0039 | `tests/stages.rs::clear_flags_apply_before_new_entries` | `--clear-awaiting-human` and `--clear-blockers` combined with new entries keep the new entries (REQ-cli-007) |

Regression: the full suite, `fledge lanes run verify` (including the exercise) and
hosted CI at the tip. Real-state check: rerun the BUG-0004 reproduction procedure in
a disposable copy of `1a108d9`, whose checkpoint is at accept for the archived
change, and record a passing observation; record a passing observation for
BUG-0005 from the regression test run; mark both bugs verified.
