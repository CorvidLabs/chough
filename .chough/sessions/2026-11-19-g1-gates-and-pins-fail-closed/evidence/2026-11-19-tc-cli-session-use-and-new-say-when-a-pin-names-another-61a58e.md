---
kind: evidence
id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-61a58e
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
duration_seconds: 0
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:0b0695893a483e6043a890db5460ce52008bae9f4901d4d3b772a534a54e7428
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:37Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:37Z. The raw log stays in the machine-local store.
