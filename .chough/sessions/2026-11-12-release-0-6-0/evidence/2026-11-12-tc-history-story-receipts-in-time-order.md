---
kind: evidence
id: 2026-11-12-tc-history-story-receipts-in-time-order
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact a_storys_receipts_are_in_observation_time_order for TC-history-story-receipts-in-time-order: failed (exit 101, tests run 1)."
result: failed
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
exit_code: 101
tests_run: 1
duration_seconds: 11
revision: da98f439e0a8db2a2c9cde127a83426d96730f49
content: 81b75050bbbfd8a1c97dd22da05b11e7992001acc0ee690ded978dcde6e4b3ff
environment: local-macos-arm64
fingerprint: sha256:17aa15072bf86be6679d81fd615e417393390e5cb90e705f8d048fbd07246406
criteria: [LIFE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:48:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact a_storys_receipts_are_in_observation_time_order for TC-history-story-receipts-in-time-order: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da98f439e0a8, observed 2026-11-12T16:48:21Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_storys_receipts_are_in_observation_time_order stdout ----
- thread 'a_storys_receipts_are_in_observation_time_order' (919504991) panicked at tests/journal.rs:1389:5:
- receipts are not in observation-time order: ---
- kind: session
- id: 2026-11-12-ship-a-focused-change
- aliases: [ship-a-focused-change]
- objective: Ship a focused change
- intent_scope: LIFE
- modules: [cli]
- stage: done
- stage_status: active
- iteration: 1
- callers: []
- native_change: c1
- pr: https://example.invalid/SYNTHETIC-org/SYNTHETIC-repo/pull/7
- criteria: []
- requirements: []
- cases: []
- design: []
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 8.48s
