---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-ba13e7
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-given-value-only-when-clean]
case_digest: [1b9e60bd395240e7bada8ab87a5255a98f5c52063ce7aa6466b737437d87f5e4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - errors_show_a_given_value_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 135a7e22079edda12ff28e7c1f3ac227b6aadb99
content: 1af7432d2edb6e67661429932ce5256b156e45d139929ffbf77dfa141b66c47a
environment: local-macos-arm64
fingerprint: sha256:cd20e1a63e620df3a5c71b1d2e748bea44b8ed993a87006c35b283ef59e28723
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:31:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 135a7e22079e, observed 2026-11-19T04:31:40Z. The raw log stays in the machine-local store.
