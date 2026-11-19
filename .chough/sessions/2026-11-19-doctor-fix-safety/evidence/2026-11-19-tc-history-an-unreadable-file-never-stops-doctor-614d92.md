---
kind: evidence
id: 2026-11-19-tc-history-an-unreadable-file-never-stops-doctor-614d92
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
duration_seconds: 25
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:109d562b39e456780c041caca78c61fa13a4029df768e778cf53565402bf950e
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:10:22Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_unreadable_file_in_an_archived_folder_never_stops_doctor for TC-history-an-unreadable-file-never-stops-doctor: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:10:22Z. The raw log stays in the machine-local store.
