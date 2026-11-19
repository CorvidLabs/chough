---
kind: evidence
id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another
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
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:f2bbc2358d390a362c5d6ff80bfe370e45dd431fb597b0a944fa1981fec87d2f
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:22Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:22Z. The raw log stays in the machine-local store.
