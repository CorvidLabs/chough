---
kind: evidence
id: 2026-11-19-tc-cli-ship-keeps-the-session-it-started-from
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves for TC-cli-ship-keeps-the-session-it-started-from: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-keeps-the-session-it-started-from]
case_digest: [8be99612cc944962c998cf5e285bab715040c47559fb21ad08eac0c6e0e735ac]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_keeps_the_session_it_started_from_when_the_pointer_moves
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:c9412dda04f3e9b1c0d53b0f4264be47f9a2d750b1cd143016871b805d4f109e
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:24:03Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_keeps_the_session_it_started_from_when_the_pointer_moves for TC-cli-ship-keeps-the-session-it-started-from: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:24:03Z. The raw log stays in the machine-local store.
