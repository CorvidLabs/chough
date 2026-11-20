---
kind: evidence
id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-dd29b8
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-an-unattended-done-needs-a-reason]
case_digest: [9b7afffc63069734d84ed6c521a86bc957cd0b7fd4327fef7f0f55a2004d92c3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - without_specsync_a_done_across_plan_and_accept_needs_a_reason
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:45c57fce3c351ce48a63c37736e22a419e01e4e0fb4b39514b44083b0b9c7b23
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:57:04Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:57:04Z. The raw log stays in the machine-local store.
