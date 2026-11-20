---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided
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
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:3fed28a399110b77c89c4308bfe315e173a5513a02e1ea1c3f9b5c0be62e7991
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:25Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:25Z. The raw log stays in the machine-local store.
