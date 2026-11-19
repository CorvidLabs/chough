---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor
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
duration_seconds: 4
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:61b88e9fc75301e0a40dd0d447d952274cfb5234dd57d01b04326e46af22597f
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:51Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:51Z. The raw log stays in the machine-local store.
