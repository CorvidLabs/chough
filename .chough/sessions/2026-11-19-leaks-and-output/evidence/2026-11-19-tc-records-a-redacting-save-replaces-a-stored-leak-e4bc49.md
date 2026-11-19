---
kind: evidence
id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak-e4bc49
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
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:c4e2b838c31970252de65dd26a77cc0f764bfd2cbf825cdde788e75458d585b1
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:43Z. The raw log stays in the machine-local store.
