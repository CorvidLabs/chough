---
kind: evidence
id: 2026-11-12-tc-history-story-receipts-in-time-order-4f9404
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact a_storys_receipts_are_in_observation_time_order for TC-history-story-receipts-in-time-order: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-story-receipts-in-time-order]
case_digest: [97981b6f9194e7d29f049d42a022a9248c3077fc8234481a7e3511a64f9c7b62]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - a_storys_receipts_are_in_observation_time_order
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 0af25ea1480d1d0ce7eadd189f3a8b0c8b520fd2
content: 5511f9a2ec985a55c1f2ae5978bc45fe1b06f0386f884799ca8962145d552f86
environment: local-macos-arm64
fingerprint: sha256:64c1a1a9fc8e3c90fcce530054b3e03926a98090de4c5553022d1052dbfc81d3
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:19:38Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact a_storys_receipts_are_in_observation_time_order for TC-history-story-receipts-in-time-order: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0af25ea1480d, observed 2026-11-12T17:19:38Z. The raw log stays in the machine-local store.
