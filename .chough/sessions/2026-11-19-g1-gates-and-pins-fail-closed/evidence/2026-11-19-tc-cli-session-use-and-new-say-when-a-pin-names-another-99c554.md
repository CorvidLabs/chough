---
kind: evidence
id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-99c554
aliases: []
claim: "Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-session-use-and-new-say-when-a-pin-names-another-session]
case_digest: [d9c6fb3e3e8febc59bbef3b3e30266bcc5f4fb4c09b851462740d2c4fe8ba75c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - session_pin
  - "--"
  - "--exact"
  - session_use_and_new_say_when_a_pin_still_names_another_session
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
environment: local-macos-arm64
fingerprint: sha256:b9eb6860dcfbf4016eb2efd88d68cdac190ffbee99bb0130ad93361f3b320827
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:25:27Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0c31209bb80, observed 2026-11-19T15:25:27Z. The raw log stays in the machine-local store.
