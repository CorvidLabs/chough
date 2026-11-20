---
kind: evidence
id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-c10aa7
aliases: []
claim: "Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-pull-request-shows-in-every-view]
case_digest: [7421baf7173e5da7c67af953856d93dd122202a6087889f3881bbd4be21f5332]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - checkpoint
  - "--"
  - "--exact"
  - the_pull_request_shows_in_next_status_context_and_the_checkpoint_view
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:7a4bec3f3c7135aef9449c4cbc0af15c207fdb42952478efa5cddf834c8fd94e
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:11Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:11Z. The raw log stays in the machine-local store.
