---
kind: evidence
id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session
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
duration_seconds: 1
revision: 8482f2025741f35d4b56afae8df77248c26f55f3
content: ff833c9c6d490b304d520e0ac627541e69e3bfcd47395f2d8e43501b3bb5ddcd
environment: local-macos-arm64
fingerprint: sha256:ae2726fc58ee6ad7ba6778818a2621de55969d0a54582c51453f2d0e0ed3fc98
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:12:19Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8482f2025741, observed 2026-11-19T15:12:19Z. The raw log stays in the machine-local store.
