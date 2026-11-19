---
kind: evidence
id: 2026-11-19-tc-records-a-redacting-save-replaces-a-stored-leak
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
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:ea9a3e82badc070829009b02464158adf827e2b7e20233a03e8e9489a5ed5ec8
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:16Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact a_redacting_save_replaces_a_stored_leak_in_place for TC-records-a-redacting-save-replaces-a-stored-leak: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:16Z. The raw log stays in the machine-local store.
