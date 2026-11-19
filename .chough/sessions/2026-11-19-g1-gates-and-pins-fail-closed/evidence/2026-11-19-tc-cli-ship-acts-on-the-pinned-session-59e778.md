---
kind: evidence
id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-59e778
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
duration_seconds: 3
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:d2f168d716465d10baf1f4861aaeab51f01633decdd9d8356a3f597e6e9407dc
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:44:02Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:44:02Z. The raw log stays in the machine-local store.
