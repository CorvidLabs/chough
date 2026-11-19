---
kind: evidence
id: 2026-11-19-tc-history-a-kept-entry-is-kept-by-its-file
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
duration_seconds: 16
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:8ab4d600306a4dfe5fffd3188fb4e87d7bcfa181c843bcb03f2c27824558045b
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:11:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:11:56Z. The raw log stays in the machine-local store.
