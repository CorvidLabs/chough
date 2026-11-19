---
kind: evidence
id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-3ad8cc
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
duration_seconds: 5
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:8bc87b03bb3d2cb2529c2abbe359ee11bf643d4ac6aa6dc9be3a4ff81dc29a50
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:54:09Z. The raw log stays in the machine-local store.
