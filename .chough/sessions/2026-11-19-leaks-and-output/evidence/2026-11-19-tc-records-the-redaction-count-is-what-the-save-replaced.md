---
kind: evidence
id: 2026-11-19-tc-records-the-redaction-count-is-what-the-save-replaced
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-the-redaction-count-is-what-the-save-replaced]
case_digest: [64420d75b9474d13ef66f57850ee92362ed4092a98b5fd39f02439bb2ed6110d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - the_redaction_count_is_what_the_save_replaced
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:ed46ca7f1aa2c8b484d24bcd22e4307387927e9f87e0608be890529aa106e877
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:54:23Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact the_redaction_count_is_what_the_save_replaced for TC-records-the-redaction-count-is-what-the-save-replaced: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:54:23Z. The raw log stays in the machine-local store.
