---
kind: evidence
id: 2026-11-20-tc-cli-the-pull-request-shows-in-every-view-363716
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
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:1c42bd82cb58905c3481e0a9e9f3604acc3828e94168d28a17ea2250450d623c
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:38:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact the_pull_request_shows_in_next_status_context_and_the_checkpoint_view for TC-cli-the-pull-request-shows-in-every-view: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:38:06Z. The raw log stays in the machine-local store.
