---
kind: evidence
id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-d489f3
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-the-post-links-only-a-pull-request-form]
case_digest: [6980ad481e65f7315ce6603bd014029ff29b35141e9a9b50e8cc52c48e3cdd11]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - a_stored_pr_that_is_neither_form_is_left_out_of_the_post
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:0fc7d4bf3bbb4f568e7ba3ec3bbd62d23a28d84017351d99aa9185f765653e2d
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:25:53Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:25:53Z. The raw log stays in the machine-local store.
