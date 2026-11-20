---
kind: evidence
id: 2026-11-20-tc-records-an-explicit-evidence-id-is-named-by-what-it-holds-a244fe
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
revision: f34dcc8391b47cd2b32c04a89933af75497bc177
content: 9c46ada16719ee7f5bc1b93a1452cb6032aad8f285116a7b70082427b6284a88
environment: local-macos-arm64
fingerprint: sha256:299ffab539094e8a1ae8821f435ebb0ad346aa33da097db5ff1c882251399a75
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:15:38Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact an_explicit_evidence_id_is_named_by_what_it_holds for TC-records-an-explicit-evidence-id-is-named-by-what-it-holds: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f34dcc8391b4, observed 2026-11-20T22:15:38Z. The raw log stays in the machine-local store.
