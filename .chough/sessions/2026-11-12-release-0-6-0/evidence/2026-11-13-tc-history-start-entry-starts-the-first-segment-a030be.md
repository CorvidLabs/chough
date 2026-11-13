---
kind: evidence
id: 2026-11-13-tc-history-start-entry-starts-the-first-segment-a030be
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:5d2747706a81db4cfe9bdbfad4e1ae165c8ca5d21bfb07e0f1763128045ef6cd
criteria: [LIFE-7, DRIVE-3]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:14:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:14:37Z. The raw log stays in the machine-local store.
