---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-331ecd
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
duration_seconds: 3
revision: dfc30a73ea6b78ffa1c305f1d4727a4a15927f82
content: b6aac05392959ac230a3e8cf88776c731d15e36bef91028874a7ed2bc16afef2
environment: local-macos-arm64
fingerprint: sha256:d280142d6d64f78596520ec6c381b6a6c2dd383841214e8ce7aa7ce26b3d5e78
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:39:39Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dfc30a73ea6b, observed 2026-11-19T04:39:39Z. The raw log stays in the machine-local store.
