---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-decided-by-value-only-when-clean-8fb099
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
duration_seconds: 2
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:66d46d639b08c4b79bab8cf459f122389d6691a75867e8f6fd4db9a1fea7559e
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:53:56Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test humans -- --exact a_decided_by_value_is_shown_only_when_clean for TC-records-errors-show-a-decided-by-value-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:53:56Z. The raw log stays in the machine-local store.
