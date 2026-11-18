---
kind: evidence
id: 2026-11-18-tc-cli-checkpoint-pr-sets-the-pull-request-1a789c
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
revision: 7a87c4933b41576b899d8eec6c6cf8c4a69e1195
content: 2e9367917400ff6e52fd320623d1ec301d6ed1dd9efe5c3fd8ebdacc8d7b3c5c
environment: local-macos-arm64
fingerprint: sha256:e88b1e780e7b893b42754c2fd9aa77f2b989a291145d22ebb0891ff1ab43031b
criteria: [DRIVE-24]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:42:38Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact checkpoint_pr_sets_and_clears_the_pull_request for TC-cli-checkpoint-pr-sets-the-pull-request: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 7a87c4933b41, observed 2026-11-18T23:42:38Z. The raw log stays in the machine-local store.
