---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-3b0644
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
duration_seconds: 3
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:00214c384f66173b39eb210f966c01bde4aaf9ec55dd9d3adc0bc6c34b57bf03
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:04Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:04Z. The raw log stays in the machine-local store.
