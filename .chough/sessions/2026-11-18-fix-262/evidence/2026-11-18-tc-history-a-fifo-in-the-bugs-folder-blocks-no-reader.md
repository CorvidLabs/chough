---
kind: evidence
id: 2026-11-18-tc-history-a-fifo-in-the-bugs-folder-blocks-no-reader
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
duration_seconds: 1
revision: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
content: e55fa100450dd1aeb9d67eddd7f012e4b6367cda9338a9a63fef31dbe2911481
environment: local-macos-arm64
fingerprint: sha256:5876290d8b4c5690ad5c7c233b02e55ce59eae46a3200034af9b685bdaf9e78b
criteria: [DRIVE-14]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T00:37:05Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test special_files -- --exact a_fifo_in_the_bugs_folder_blocks_no_reader for TC-history-a-fifo-in-the-bugs-folder-blocks-no-reader: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8abb5bb89b52, observed 2026-11-18T00:37:05Z. The raw log stays in the machine-local store.
