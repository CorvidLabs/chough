---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-7daf52
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
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:fd4335e1d7a4ad070c43421a612956fdf5f7c8a82f4294c4dea4b66b700e4e41
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:57:06Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:57:06Z. The raw log stays in the machine-local store.
