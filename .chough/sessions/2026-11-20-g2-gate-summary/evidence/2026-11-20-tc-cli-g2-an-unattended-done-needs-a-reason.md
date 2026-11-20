---
kind: evidence
id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason
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
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:085c824626e3c5f36a8da2595ba1093483331d2b8c83509483d38339c6e63cbe
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:23Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:23Z. The raw log stays in the machine-local store.
