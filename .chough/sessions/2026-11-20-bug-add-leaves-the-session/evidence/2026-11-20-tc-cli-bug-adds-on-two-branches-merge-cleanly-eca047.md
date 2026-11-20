---
kind: evidence
id: 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly-eca047
aliases: []
claim: "Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-bug-adds-on-two-branches-merge-cleanly]
case_digest: [9a381267c6fa4636c79c97b4c76bf43849335d876d2b49bcac4fa41ab826696b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - bugs
  - "--"
  - "--exact"
  - bug_adds_on_two_branches_in_one_session_merge_cleanly
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 04c4b86f441ccdc2ac65197c8a41abade68a9fee
content: be2ed9fc738750c49bb3f21a747855ecc4ca71132349d92e132695841ead9ac4
environment: local-macos-arm64
fingerprint: sha256:e40d3ffc1025d23648c0ee7a1dc1d048d49c8c4b9e7e650761586302c45872ff
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012, REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T16:34:41Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 04c4b86f441c, observed 2026-11-20T16:34:41Z. The raw log stays in the machine-local store.
