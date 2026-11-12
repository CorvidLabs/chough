---
kind: evidence
id: 2026-11-12-tc-history-start-entry-starts-the-first-segment
aliases: []
claim: "Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-start-entry-starts-the-first-segment]
case_digest: [4826f76dc094adbd89739183af30b3a2541b8c1fbe5529be665b7a197ca157cb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - timing
  - "--"
  - "--exact"
  - init_and_session_new_write_a_start_entry_that_timing_reads
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:202517fc34123a3363dfa9337286d456c7d42de090dcc27e2b21adead46b8521
criteria: [LIFE-7, DRIVE-3]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:06:08Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:06:08Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- init_and_session_new_write_a_start_entry_that_timing_reads stdout ----
- thread 'init_and_session_new_write_a_start_entry_that_timing_reads' (924237124) panicked at tests/timing.rs:2425:9:
- assertion `left == right` failed: []
-   left: 0
-  right: 1
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.60s
