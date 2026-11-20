---
kind: evidence
id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-a61c25
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:2e17c7a3adaa84b9b3a97be29cf11ee89dfef1b5bfdeb5f5ac15eac78837ad63
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:41:49Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:41:49Z. The raw log stays in the machine-local store.
