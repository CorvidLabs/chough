---
kind: evidence
id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-12330e
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move]
case_digest: [5a71b02828eacc6a111ef179e060d318d7c9d6d728f159c720082d78bacc2243]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_hand_edited_no_native_change_cannot_unlock_the_ungated_move
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 659dcded2db9d7f9dd9e82fe6d398a132d4d2388
content: ba5fc1619c8f83de2f7ecd06c3c14961bc2644b05cdff88cafe9fc58e2108935
environment: local-macos-arm64
fingerprint: sha256:fca2f56efe38456ca5e8a190d73d212b515103ee201dcd436f446bcf06b284f5
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:33:29Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 659dcded2db9, observed 2026-11-19T17:33:29Z. The raw log stays in the machine-local store.
