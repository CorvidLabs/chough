---
kind: evidence
id: 2026-11-20-tc-cli-every-session-writer-keeps-the-pull-request-form-88607b
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:3a88dcf2a57ce081ce51f5599477bdd01d93c64cc192aa91e207523bbd1e7c5d
criteria: [FLOW-13, LIFE-3]
requirements: [REQ-cli-007]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:41:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test checkpoint -- --exact every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out for TC-cli-every-session-writer-keeps-the-pull-request-form: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:41:45Z. The raw log stays in the machine-local store.
