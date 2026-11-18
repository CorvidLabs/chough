---
kind: evidence
id: 2026-11-18-tc-cli-checkpoint-pr-sets-the-pull-request
aliases: []
claim: "Ran cargo test --locked --test checkpoint -- --exact checkpoint_pr_sets_and_clears_the_pull_request for TC-cli-checkpoint-pr-sets-the-pull-request: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-checkpoint-pr-sets-the-pull-request]
case_digest: [d5b6574e6559b6e9414184de1b6be51348b90ff42c741fc80acf4cee692ee67f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - checkpoint
  - "--"
  - "--exact"
  - checkpoint_pr_sets_and_clears_the_pull_request
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
content: 62015fce69c9ab4f104c820953c0c38bb4a4b6d9bf657febd825bffcf55676ea
environment: local-macos-arm64
fingerprint: sha256:a29788f77494829f8d10501b5ff7e951501bba1994e3309f8c5e3b0810abe514
criteria: [DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:34:10Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact checkpoint_pr_sets_and_clears_the_pull_request for TC-cli-checkpoint-pr-sets-the-pull-request: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 66efea4dd777, observed 2026-11-18T20:34:10Z. The raw log stays in the machine-local store.
