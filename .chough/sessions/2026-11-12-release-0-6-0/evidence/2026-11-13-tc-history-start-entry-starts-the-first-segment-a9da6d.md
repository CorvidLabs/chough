---
kind: evidence
id: 2026-11-13-tc-history-start-entry-starts-the-first-segment-a9da6d
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:5d2747706a81db4cfe9bdbfad4e1ae165c8ca5d21bfb07e0f1763128045ef6cd
criteria: [LIFE-7, DRIVE-3]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:47:44Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test timing -- --exact init_and_session_new_write_a_start_entry_that_timing_reads for TC-history-start-entry-starts-the-first-segment: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:47:44Z. The raw log stays in the machine-local store.
