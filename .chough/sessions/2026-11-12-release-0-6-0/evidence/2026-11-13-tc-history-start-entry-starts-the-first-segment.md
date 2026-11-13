---
kind: evidence
id: 2026-11-13-tc-history-start-entry-starts-the-first-segment
aliases: []
claim: "Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:71e6074f83b0cddb7d474239d558ef7b31e72cdef7af62d79b655e068b5ad9d6
criteria: [LIFE-7, DRIVE-3]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:21:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:21:40Z. The raw log stays in the machine-local store.
