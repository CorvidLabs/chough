---
kind: evidence
id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-5f558b
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-acts-on-the-pinned-session]
case_digest: [416c0ce58c7d0afd7055a2c22b7cab4af4fdc66d1201a4a655ff53b53448b1ef]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:c28fd66f1b023f58315b1e752572f6d074669aab60f46b4995ae46b740a82c02
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:23:51Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:23:51Z. The raw log stays in the machine-local store.
