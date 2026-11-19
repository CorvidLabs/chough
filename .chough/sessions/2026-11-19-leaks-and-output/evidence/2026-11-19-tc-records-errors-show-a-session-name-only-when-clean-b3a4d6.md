---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-b3a4d6
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
duration_seconds: 0
revision: dad03318e83ea9e93b14547cdf5f3dd1f4978ccd
content: 5beee9dcb4c059df64f32c6edb22a87fc208406ace0dc20c3d4567357dfd20ea
environment: local-macos-arm64
fingerprint: sha256:0dac13c68005aa163d354909073555bf37e256de633df920734573353177e55d
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:04:20Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dad03318e83e, observed 2026-11-19T03:04:20Z. The raw log stays in the machine-local store.
