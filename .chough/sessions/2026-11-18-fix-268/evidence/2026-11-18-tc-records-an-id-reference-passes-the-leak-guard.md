---
kind: evidence
id: 2026-11-18-tc-records-an-id-reference-passes-the-leak-guard
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
revision: 0ec498884fa5a68bf0c9528b4dc627a0fdf28751
content: 7433f506277b409e303be381d1401a0aba6095c5ff2812ba85b260fd89102238
environment: local-macos-arm64
fingerprint: sha256:3680443d0bc9bff022d54cdea68fc3339ecfdabf1d0db12580ed316102f564a2
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T15:54:59Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4g
---
**Ran cargo test --locked --test leaks -- --exact a_token_in_an_id_reference_is_refused_even_under_redact for TC-records-an-id-reference-passes-the-leak-guard: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0ec498884fa5, observed 2026-11-18T15:54:59Z. The raw log stays in the machine-local store.
