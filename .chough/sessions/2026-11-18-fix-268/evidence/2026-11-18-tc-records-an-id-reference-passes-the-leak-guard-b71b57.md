---
kind: evidence
id: 2026-11-18-tc-records-an-id-reference-passes-the-leak-guard-b71b57
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact a_token_in_an_id_reference_is_refused_even_under_redact for TC-records-an-id-reference-passes-the-leak-guard: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-an-id-reference-passes-the-leak-guard]
case_digest: [0330525c31d3b09041ab42ce3d87ef4e68ea9b5d62848ff657d97ad28a495041]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - a_token_in_an_id_reference_is_refused_even_under_redact
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6f5347343890308fc96ff050d004404d5c35bbe9
content: 33fab87e35740e5ff5f96d0121f4200d560a55a78ba7ed398d0cbf5795120a1a
environment: local-macos-arm64
fingerprint: sha256:82a7224125811ff7191ce8833cfa98ba534af3425831f9cd4944cfcf7405800e
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T16:09:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_token_in_an_id_reference_is_refused_even_under_redact for TC-records-an-id-reference-passes-the-leak-guard: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6f5347343890, observed 2026-11-18T16:09:14Z. The raw log stays in the machine-local store.
