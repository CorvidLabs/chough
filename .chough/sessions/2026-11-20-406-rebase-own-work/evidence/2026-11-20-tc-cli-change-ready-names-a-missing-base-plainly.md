---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-names-a-missing-base-plainly
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-names-a-missing-base-plainly]
case_digest: [d6851ef47f5c60f0fba435275e008a048d4eba48aa7aaef9def0095aff34494f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_base_commit_the_repository_lacks_is_named_plainly
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
environment: local-macos-arm64
fingerprint: sha256:e91cc8d6752be72849bfe228c99d8b68397b306980a8ae6caea11ba03698e15f
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:32:55Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_base_commit_the_repository_lacks_is_named_plainly for TC-cli-change-ready-names-a-missing-base-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 874d3a4e8cb9, observed 2026-11-20T04:32:55Z. The raw log stays in the machine-local store.
