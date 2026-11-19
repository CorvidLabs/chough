---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-657ce6
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
duration_seconds: 16
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:cf77d6cb8bade5801de5aecb0b842bde8da5ee621d33efe89fc1f54b787c086d
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:14:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:14:12Z. The raw log stays in the machine-local store.
