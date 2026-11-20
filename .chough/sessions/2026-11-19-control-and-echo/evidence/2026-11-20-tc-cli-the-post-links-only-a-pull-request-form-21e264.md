---
kind: evidence
id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-21e264
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
duration_seconds: 4
revision: cdc0d2e2f85a6d09567b8780662656ccec551526
content: ea8b493cbeb2bc46835fda1e38ebf881b15f161e91716180c4aec5d9bd3abb01
environment: local-macos-arm64
fingerprint: sha256:82437c435460e226dfa10ccf6e6cec0ff7152257cbfe701c452f6fdd1507a9cb
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:33:24Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cdc0d2e2f85a, observed 2026-11-20T04:33:24Z. The raw log stays in the machine-local store.
