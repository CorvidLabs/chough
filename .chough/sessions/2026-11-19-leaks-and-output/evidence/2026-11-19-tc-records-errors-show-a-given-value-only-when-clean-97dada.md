---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-given-value-only-when-clean-97dada
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
revision: ccb57abff508d187593c07849872a17c37036014
content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
environment: local-macos-arm64
fingerprint: sha256:b3367108614fe34ee5ebbb2a1163516ac641da9b587385d59be77dabe169f83b
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:39:02Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact errors_show_a_given_value_only_when_clean for TC-records-errors-show-a-given-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ccb57abff508, observed 2026-11-19T03:39:02Z. The raw log stays in the machine-local store.
