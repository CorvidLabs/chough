---
kind: evidence
id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-f72a5c
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
duration_seconds: 12
revision: 49401eb44ee9f38f3a5d2b01dc85a23b6a49ce2b
content: c49862bde5f5323cf7bc123c229c342bb37e821e64b3e5978db49c0c0abacb02
environment: local-macos-arm64
fingerprint: sha256:24ac40ba3e7fbfb4e821ec9d834a41cc05cd1c9fb4c0d41fdc321a0f6dc07e43
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:43:48Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 49401eb44ee9, observed 2026-11-19T16:43:48Z. The raw log stays in the machine-local store.
