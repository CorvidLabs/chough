---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-96adc3
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-a-move-past-plan-says-whether-a-human-decided]
case_digest: [2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - without_specsync_a_move_past_plan_says_whether_a_human_decided
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:4a01c0eadbdd98c6f12ba69a8c1c56856bc8a3506f4021864673fbd9f35755c6
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:47Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:47Z. The raw log stays in the machine-local store.
