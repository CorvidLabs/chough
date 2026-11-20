---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-on-the-default-branch-names-own-files
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-on-the-default-branch-names-own-files]
case_digest: [0cb9bc7a1551cc2ca552005b982baf6dcb01712bb51e8fbfd1c12d8a3b0a2f12]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - on_the_default_branch_itself_the_change_s_files_are_still_named
exit_code: 0
tests_run: 1
duration_seconds: 5
revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
environment: local-macos-arm64
fingerprint: sha256:0176451202936e5de908baf71868ba0a2f30f8dde7707d8637b889b73fd35f6f
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:32:49Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact on_the_default_branch_itself_the_change_s_files_are_still_named for TC-cli-change-ready-on-the-default-branch-names-own-files: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 874d3a4e8cb9, observed 2026-11-20T04:32:49Z. The raw log stays in the machine-local store.
