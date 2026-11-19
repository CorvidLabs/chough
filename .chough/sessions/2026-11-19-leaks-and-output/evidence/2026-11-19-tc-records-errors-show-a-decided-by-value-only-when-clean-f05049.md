---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-f05049
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
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:716567b226fedf140347ad1a48a3f4ab19145a44c2e4e67ea86a8f7843c9d1a5
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:11Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:11Z. The raw log stays in the machine-local store.
