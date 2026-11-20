---
kind: evidence
id: 2026-11-20-tc-cli-g2-an-unattended-done-needs-a-reason-d31721
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
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:83fba77d849958d305146f80766df4d90c568559cf35cbb979d560261f1df70d
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:49Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_done_across_plan_and_accept_needs_a_reason for TC-cli-g2-an-unattended-done-needs-a-reason: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:49Z. The raw log stays in the machine-local store.
