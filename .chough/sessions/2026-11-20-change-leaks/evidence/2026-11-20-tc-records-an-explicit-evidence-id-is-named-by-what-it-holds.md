---
kind: evidence
id: 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-an-explicit-evidence-id-is-named-by-what-it-holds]
case_digest: [be619fb26aa65e64c22c3b6d3b4d56c6dca7992b9fa4a38af8fcba8517e0d527]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - an_explicit_evidence_id_is_named_by_what_it_holds
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 38a01b22378e70a778b05f5da6891e6d6b41e13f
content: 6938808c10085e6558eb80e4f8523360fcb44ba436559dcd6187620cc9cd6a65
environment: local-macos-arm64
fingerprint: sha256:0d0ecd152d2f7ff0aa5e4143364152b8914569b9ea5a0b2101089d5c6dd885fd
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:06:46Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 38a01b22378e, observed 2026-11-20T17:06:46Z. The raw log stays in the machine-local store.
