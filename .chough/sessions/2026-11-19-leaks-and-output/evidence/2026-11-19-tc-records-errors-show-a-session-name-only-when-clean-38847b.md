---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-38847b
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-errors-show-a-session-name-only-when-clean]
case_digest: [5ce8025ff6d02f2ecc35c72d88260d2115be8d43d6231b25821e8adf62802fdc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - a_session_name_is_shown_only_when_clean
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 7db0083b5b70c6a2a9adeaecc2106b28dd5178df
content: 80d2a57529b9c2f201e7623078e3827ba4f5e432658025d46cc324a1fb743b09
environment: local-macos-arm64
fingerprint: sha256:422d0c227543587d1e2f05a2e5365f7a9982d78bc8652fb0ca8a1e09f0df1c95
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:53:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7db0083b5b70, observed 2026-11-19T04:53:47Z. The raw log stays in the machine-local store.
