---
kind: evidence
id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-27266b
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
revision: 08399615be5c89eeecf91690cad2bfa741c2685c
content: fef31d171b05da7c4cb171040b1e6cba36ff7502441fa8e2da97c04abe321a6d
environment: local-macos-arm64
fingerprint: sha256:c42d2b9052273bb3f1ae34a15e8777de68e68b119441d10de5758eda9ee84630
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:04:15Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 08399615be5c, observed 2026-11-19T17:04:15Z. The raw log stays in the machine-local store.
