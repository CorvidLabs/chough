---
kind: evidence
id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-79cb99
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
revision: 26a42a15d428cc7db4805069cc58c0d23330f390
content: ac1d2cb7c652eb32daa894a68a773f001f75e09bcdb2e1a55f8b21299f23ce98
environment: local-macos-arm64
fingerprint: sha256:af851aa711b00af0649b0d8fecf6889928591aa4d2c93375008eaa459265ad93
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:11:46Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26a42a15d428, observed 2026-11-20T04:11:46Z. The raw log stays in the machine-local store.
