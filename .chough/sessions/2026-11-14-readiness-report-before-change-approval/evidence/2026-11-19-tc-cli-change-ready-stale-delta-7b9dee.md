---
kind: evidence
id: 2026-11-19-tc-cli-change-ready-stale-delta-7b9dee
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-stale-delta]
case_digest: [3b14f5b1df7c6ea1a96739442f5903068bca246e377c9352cb040829c9b6ae16]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - a_stale_delta_is_named_until_the_body_is_rewritten
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: f42ce614db0e702d88e13c1aa59324c57661d8fd
content: d1da5619d2366cd32410ea839862e73f5ad74e0ba901201bc753f622084c0677
environment: local-macos-arm64
fingerprint: sha256:6b3dbe60f2da9a66d02b92fcd1ff8b599d750c9ecbc3d0544c907f069dd9da22
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T03:50:00Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact a_stale_delta_is_named_until_the_body_is_rewritten for TC-cli-change-ready-stale-delta: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f42ce614db0e, observed 2026-11-19T03:50:00Z. The raw log stays in the machine-local store.
