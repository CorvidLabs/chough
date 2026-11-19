---
kind: evidence
id: 2026-11-19-tc-records-errors-show-a-session-name-only-when-clean-134730
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
revision: b84728117b64a7ed3ac049540fde93c0228c58cb
content: 8df2cfe1b49bf070691ae617bb23edaf03f46cde732ce4deca4fc7b08e702412
environment: local-macos-arm64
fingerprint: sha256:e33471e1cc8954933b904b42b35e649a0d53b9494d07cca1bed0a89ba1417627
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T04:16:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test session_pin -- --exact a_session_name_is_shown_only_when_clean for TC-records-errors-show-a-session-name-only-when-clean: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b84728117b64, observed 2026-11-19T04:16:06Z. The raw log stays in the machine-local store.
