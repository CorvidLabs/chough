---
kind: evidence
id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-fa2c8f
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:c6516337aae3fd186bf7f1d61f666bae1722046bca82eea2e094ee2345d17219
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:11:50Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:11:50Z. The raw log stays in the machine-local store.
