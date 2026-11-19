---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-baf262
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
revision: 908d3ace4545d270125d13706f6b012cdc276e21
content: 13c35b153507f4923f42fab829e5df58a452a1ead1714f1ee5f1ce62976ccfe1
environment: local-macos-arm64
fingerprint: sha256:92eca1becfbaf5f01c1b9aa6932a1341310b7e554bd9ad7a217fd117b18adf63
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:11:48Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 908d3ace4545, observed 2026-11-19T05:11:48Z. The raw log stays in the machine-local store.
