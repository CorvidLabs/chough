---
kind: evidence
id: 2026-11-19-tc-cli-a-hand-edited-no-native-change-cannot-unlock-the-899356
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
revision: 753a022557083e0873d048ced9f5be2b40f686fd
content: c568b52cbc16fcf912257376b09e0a6ba9e32cadf694df0f0e218b8028feb920
environment: local-macos-arm64
fingerprint: sha256:0414ba00a8333672387081a92eaa307f2c6dfa0472fa6d1893c74e9266e0bab5
criteria: [DRIVE-3, DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T17:17:28Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_hand_edited_no_native_change_cannot_unlock_the_ungated_move for TC-cli-a-hand-edited-no-native-change-cannot-unlock-the-ungated-move: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 753a02255708, observed 2026-11-19T17:17:28Z. The raw log stays in the machine-local store.
