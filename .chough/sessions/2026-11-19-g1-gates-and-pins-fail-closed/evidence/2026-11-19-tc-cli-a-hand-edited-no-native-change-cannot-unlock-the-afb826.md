---
kind: evidence
id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-afb826
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
duration_seconds: 8
revision: b0c31209bb806dcb68d2eb311d032d52bd2671e6
content: c85945fd930cfde7add26c2ac46953d495d8e167e04d29ea35fb56538a652aed
environment: local-macos-arm64
fingerprint: sha256:29e3ace83c6c7b154ad01838eed2b2ab304ea3c363875d1cb35294cad45adf32
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:25:13Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0c31209bb80, observed 2026-11-19T15:25:13Z. The raw log stays in the machine-local store.
