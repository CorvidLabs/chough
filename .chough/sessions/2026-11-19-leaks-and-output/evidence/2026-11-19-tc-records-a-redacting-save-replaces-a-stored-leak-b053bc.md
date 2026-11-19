---
kind: evidence
id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-b053bc
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-redacting-save-replaces-a-stored-leak]
case_digest: [44004ef165e948a2f6f1a5c6eeae7ffffd09c4656ed449a43d7454aebe4eb288]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_redacting_save_replaces_a_stored_leak_in_place
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 75f43060e66b768d48ca01e931aa2c076709e1b7
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:3dd65eb8dadbdb534620fc3b511de29ddda62f2682170be20798909e3b0a150a
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:23:28Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 75f43060e66b, observed 2026-11-19T14:23:28Z. The raw log stays in the machine-local store.
