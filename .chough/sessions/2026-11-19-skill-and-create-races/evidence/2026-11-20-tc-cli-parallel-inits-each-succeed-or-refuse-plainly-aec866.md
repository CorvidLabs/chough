---
kind: evidence
id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-aec866
aliases: []
claim: "Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-parallel-inits-each-succeed-or-refuse-plainly]
case_digest: [59a8124b5e60791583eb75d311612c77d169ec54345a474522cd6a70b531d0e3]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - parallel
  - "--"
  - "--exact"
  - parallel_inits_each_succeed_or_refuse_plainly
exit_code: 0
tests_run: 1
duration_seconds: 8
revision: 29fb39131c6435612182f52929b94e69ab0dcba3
content: 96d7762eff66c97a902e624035a7af9cb23d58368e612eba154b1b05115db813
environment: local-macos-arm64
fingerprint: sha256:07ad4746ad5276845264e943270df8ed383a18bc87f27d287d875c41ff8dbc8d
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:19:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 29fb39131c64, observed 2026-11-20T14:19:40Z. The raw log stays in the machine-local store.
