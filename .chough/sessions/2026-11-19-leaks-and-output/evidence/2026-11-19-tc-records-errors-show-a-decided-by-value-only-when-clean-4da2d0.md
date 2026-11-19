---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-4da2d0
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-decided-by-value-only-when-clean]
case_digest: [aea609b56c0229da7fa925443dc2e4a1eb5108fa065ae3086e1c67eafb5a85be]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_decided_by_value_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: ccb57abff508d187593c07849872a17c37036014
content: c3500bb96540c9a5aa130e951210dcae975d696e99737aa39f1c00dd06161e22
environment: local-macos-arm64
fingerprint: sha256:2b018f6aa686755dae58ed6610b1cfcbe5ca40ce13d645ec974fde6759df8aaf
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:39:00Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ccb57abff508, observed 2026-11-19T03:39:00Z. The raw log stays in the machine-local store.
