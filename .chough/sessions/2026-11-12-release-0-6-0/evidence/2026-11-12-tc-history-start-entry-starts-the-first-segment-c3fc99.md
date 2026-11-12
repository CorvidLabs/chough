---
kind: evidence
id: 2026-11-12-tc-history-start-entry-starts-the-first-segment-c3fc99
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
duration_seconds: 1
revision: 91c3824f94a9742f127a480d297fc7f2175e631c
content: c2aa3b745dd7a94452e0b4feee5950b8b356056ae629fdf516c4dcf5c7d1e6fd
environment: local-macos-arm64
fingerprint: sha256:639768871bf28c090133abe9f9b9fe1883652a1645aa2e8872b933d3319a00db
criteria: [LIFE-7, DRIVE-3]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:07:39Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 91c3824f94a9, observed 2026-11-12T21:07:39Z. The raw log stays in the machine-local store.
