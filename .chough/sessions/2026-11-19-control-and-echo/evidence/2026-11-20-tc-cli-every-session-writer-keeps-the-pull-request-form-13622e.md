---
kind: evidence
id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-13622e
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
duration_seconds: 4
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:06ff64b427728d6bbd123e856e1eb9c69483c0c2b74bbfad947fa8d8c2c8804f
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:11:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:11:14Z. The raw log stays in the machine-local store.
