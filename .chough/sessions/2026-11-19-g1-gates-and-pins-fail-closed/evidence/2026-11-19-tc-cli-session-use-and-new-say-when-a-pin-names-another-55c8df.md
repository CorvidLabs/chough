---
kind: evidence
id: 2026-11-19-tc-cli-session-use-and-new-say-when-a-pin-names-another-55c8df
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
revision: 08399615be5c89eeecf91690cad2bfa741c2685c
content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
environment: local-macos-arm64
fingerprint: sha256:48a0480025db721f4b953ed6e3ced6404851bd05bb42ee3f0d79d7ad43e006be
criteria: [DRIVE-3]
requirements: [REQ-cli-037]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:04:17Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test session_pin -- --exact session_use_and_new_say_when_a_pin_still_names_another_session for TC-cli-session-use-and-new-say-when-a-pin-names-another-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08399615be5c, observed 2026-11-19T17:04:17Z. The raw log stays in the machine-local store.
