---
kind: evidence
id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-1b762e
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:55d0c72e4345963ff068a23cee62187f9e6b50a9ffec44722fb97e1898523cd7
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:41:42Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:41:42Z. The raw log stays in the machine-local store.
