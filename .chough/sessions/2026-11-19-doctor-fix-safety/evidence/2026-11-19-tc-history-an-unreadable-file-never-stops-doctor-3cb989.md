---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3cb989
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-unreadable-file-never-stops-doctor]
case_digest: [623dd39576587f705f31f4fe1d7e0752b8b2357215804119aedac7c392922313]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_unreadable_file_in_an_archived_folder_never_stops_doctor
exit_code: 0
tests_run: 1
duration_seconds: 10
revision: 3b6c5dd6189ce15a3a661e9de7624cc7b857d3e4
content: ac34b5cc9df791042d6ad0443889821d5af8e9fb1cc92fe3084bc8c43b6482e8
environment: local-macos-arm64
fingerprint: sha256:e489a5089346d751b4363b896686119328530bba084b713b5da3a8b79a18fda3
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:49:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3b6c5dd6189c, observed 2026-11-19T14:49:24Z. The raw log stays in the machine-local store.
