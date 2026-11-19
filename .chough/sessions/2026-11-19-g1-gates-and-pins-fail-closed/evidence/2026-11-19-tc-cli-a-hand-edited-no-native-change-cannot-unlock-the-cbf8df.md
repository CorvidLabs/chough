---
kind: evidence
id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-cbf8df
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
duration_seconds: 1
revision: 9cafd0c50bda1f3bcee094f58707ef3cd4dbabac
content: 2ef0a47060f35b5a16547d697d71941f5032d1c7e578555b0ce3e742957c95ff
environment: local-macos-arm64
fingerprint: sha256:8219dde5fb6b610a6c134a107b129bf9baf85b4e696be5d9b66ef5ba08651f72
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:23:46Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9cafd0c50bda, observed 2026-11-19T19:23:46Z. The raw log stays in the machine-local store.
