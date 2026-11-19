---
kind: evidence
id: 2026-11-19-tc-cli-ship-checks-the-whole-history-a7a49d
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-ship-checks-the-whole-history]
case_digest: [bfea68e596414a467d4884dc903a45af60f1029df4eeebaabbae5d1ddd968304]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_checks_the_whole_history_and_goes_on_without_a_base
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:7db894284a9f29d4b6865b7e3b463784c139e85d4576c31f56ab171df8d927a1
criteria: [DRIVE-3]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:32Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test change -- --exact ship_checks_the_whole_history_and_goes_on_without_a_base for TC-cli-ship-checks-the-whole-history: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:32Z. The raw log stays in the machine-local store.
