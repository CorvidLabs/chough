---
kind: evidence
id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-84c2f2
aliases: []
claim: "Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-every-session-writer-keeps-the-pull-request-form]
case_digest: [504be46f991bea14b6c546fc7acf9c1e586375a83adcb47e514a3e4a5589fccc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - checkpoint
  - "--"
  - "--exact"
  - every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:77e5574c07127780939f42640bf9cfbfca57ad7b9cc2b0a919b1acc261d08271
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:38:09Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:38:09Z. The raw log stays in the machine-local store.
