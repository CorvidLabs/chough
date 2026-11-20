---
kind: evidence
id: 2026-11-20-tc-cli-parallel-inits-each-succeed-or-refuse-plainly-a11c2d
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
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:a97898fe82e526e44251a06fd5e0b4be0b9f1f5f2cd92750dbfa7f70497bfda3
criteria: [LIFE-4, LIFE-6]
requirements: [REQ-cli-001]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:00:08Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test parallel -- --exact parallel_inits_each_succeed_or_refuse_plainly for TC-cli-parallel-inits-each-succeed-or-refuse-plainly: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T15:00:08Z. The raw log stays in the machine-local store.
