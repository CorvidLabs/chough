---
kind: evidence
id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-94ae64
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
duration_seconds: 6
revision: 0cece9573c0ca14af60b641f9fbb759186684dcd
content: 87d024fbc93e52a7f690b93ad1c207cf85e4077357bd8b05b095f69f72c96e4d
environment: local-macos-arm64
fingerprint: sha256:1c1b80b7c9df97a5de30f679cb6bb0a3772d7df9d4115cc7790e936cef3313e6
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:35:59Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cece9573c0c, observed 2026-11-20T14:35:59Z. The raw log stays in the machine-local store.
