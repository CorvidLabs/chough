---
kind: evidence
id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-69ab28
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:bcbf3dc6318efd55ec4d48fb3f62ec3a16651fd45bf5eb621a507043f70a07cd
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:25:48Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:25:48Z. The raw log stays in the machine-local store.
