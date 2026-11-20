---
kind: evidence
id: 2026-11-20-tc-cli-bug-adds-on-two-branches-merge-cleanly
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
revision: 2f99dcfb42f130a32cdf751435dcf85d3aab4cca
content: 9ff84979f8f49ab9838777226bdbb55e976f6789d5c595e146addbcee9b9151e
environment: local-macos-arm64
fingerprint: sha256:731c6f2548394ab08961bb7e9fc991155b9aad1b2061a9b64f32a0a481b76586
criteria: [LIFE-5, DRIVE-12]
requirements: [REQ-cli-012, REQ-cli-054]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:48:26Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test bugs -- --exact bug_adds_on_two_branches_in_one_session_merge_cleanly for TC-cli-bug-adds-on-two-branches-merge-cleanly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2f99dcfb42f1, observed 2026-11-20T15:48:26Z. The raw log stays in the machine-local store.
