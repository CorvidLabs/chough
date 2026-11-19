---
kind: evidence
id: 2026-11-19-tc-cli-ship-acts-on-the-pinned-session-a1988a
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
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:caba923eaf8f748e9b9585ca4a46f6fde52937a1fccdd2455d852e2d5ac51841
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:35Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_closes_the_gate_and_moves_the_stage_in_the_pinned_session for TC-cli-ship-acts-on-the-pinned-session: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:35Z. The raw log stays in the machine-local store.
