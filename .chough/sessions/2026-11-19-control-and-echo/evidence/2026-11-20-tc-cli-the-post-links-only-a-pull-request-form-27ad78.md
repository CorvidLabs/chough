---
kind: evidence
id: 2026-11-20-tc-cli-the-post-links-only-a-pull-request-form-27ad78
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
duration_seconds: 2
revision: 92b89b970299939dd38635eab1c13fb1952b539a
content: 263b028bb1eb18088c6dd4ce935a7a391ed48e79086e38b4b949047414f1b1c2
environment: local-macos-arm64
fingerprint: sha256:a6673e8136a2ddf17a4db235ce1af48763d3704decc0a16be4a20a3d872d9599
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:23:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_stored_pr_that_is_neither_form_is_left_out_of_the_post for TC-cli-the-post-links-only-a-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 92b89b970299, observed 2026-11-20T03:23:18Z. The raw log stays in the machine-local store.
