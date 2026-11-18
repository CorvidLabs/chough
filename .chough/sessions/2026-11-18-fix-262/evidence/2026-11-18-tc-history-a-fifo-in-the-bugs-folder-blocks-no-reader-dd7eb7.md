---
kind: evidence
id: 2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader-dd7eb7
aliases: []
claim: "Ran cargo test --locked --test special_files -- --exact a_fifo_in_the_bugs_folder_blocks_no_reader for TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader]
case_digest: [eae0cd2501c35daaf8720a403c42fcaa9314452db9ba3643504bcb5ae835d9e2]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - special_files
  - "--"
  - "--exact"
  - a_fifo_in_the_bugs_folder_blocks_no_reader
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: af8d04991c50e7ddae3616a645495c1f1b7c306b
content: b3158149ff06f564ddcb10a8acfe7e6c62b32c3f5e8e64c059b6d29279a32860
environment: local-macos-arm64
fingerprint: sha256:de12789a32fc5721039ee1b49ac8eb0503fa2b45a45ce1642f533d934ede1fb8
criteria: [DRIVE-14]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T01:15:06Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test special_files -- --exact a_fifo_in_the_bugs_folder_blocks_no_reader for TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision af8d04991c50, observed 2026-11-18T01:15:06Z. The raw log stays in the machine-local store.
