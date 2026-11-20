---
kind: evidence
id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly
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
duration_seconds: 1
revision: 63057710d3223425b0db8d988717f2c657e5035b
content: 281e5715af06fc2baf07d3d98900c36403acb009d6f346d6428c18b86fd0cd52
environment: local-macos-arm64
fingerprint: sha256:00f52a16a89f9e48103a158b0b3416d8793e44a87d0dbd0b2756db1d15aaac32
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T09:20:43Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 63057710d322, observed 2026-11-20T09:20:43Z. The raw log stays in the machine-local store.
