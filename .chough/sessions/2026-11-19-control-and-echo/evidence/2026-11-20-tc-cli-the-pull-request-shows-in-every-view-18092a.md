---
kind: evidence
id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-18092a
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
duration_seconds: 0
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:3e01a1f30d5f4c7c70796580d3773d4b254b9cf1ae57e9ecb009fc441d625f84
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:11:44Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:11:44Z. The raw log stays in the machine-local store.
