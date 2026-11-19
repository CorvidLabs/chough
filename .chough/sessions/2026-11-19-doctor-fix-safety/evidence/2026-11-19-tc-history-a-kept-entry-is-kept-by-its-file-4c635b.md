---
kind: evidence
id: 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file-4c635b
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-kept-entry-is-kept-by-its-file]
case_digest: [768ee8a528719bfdbba66e84cc11491e82351b770ade2896855af81ee75fa3fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_kept_entry_is_kept_by_its_file_whatever_its_id
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:c5d8d3aabded3fdfb3e35dbf28c401be33be61258ac3e5a839e0cb997309e352
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:33Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:33Z. The raw log stays in the machine-local store.
