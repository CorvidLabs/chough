---
kind: evidence
id: 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-3d2da5
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
revision: 8ead84204edc55fcf5c30db35fb0beb6b7290cb2
content: 6880a79a758fb134e06561e2e779a9d6a9025ddc5484976b625f5c52cc4b2dea
environment: local-macos-arm64
fingerprint: sha256:10e78f27f7822bf115bf76829defebaaba62d0cdcb9c8a68bb24b6b5992eb56c
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:34:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8ead84204edc, observed 2026-11-20T17:34:00Z. The raw log stays in the machine-local store.
