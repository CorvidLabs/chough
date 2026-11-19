---
kind: evidence
id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-eb55fa
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
duration_seconds: 2
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:fd16353c69bfbcd439b039d70bdb7ea8f8489616548a97f80730319d13f73962
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:44:07Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:44:07Z. The raw log stays in the machine-local store.
