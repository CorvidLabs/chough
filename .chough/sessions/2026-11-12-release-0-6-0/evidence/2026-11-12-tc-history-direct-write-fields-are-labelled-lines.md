---
kind: evidence
id: 2026-11-12-tc-history-direct-write-fields-are-labelled-lines
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_is_written_as_labelled_lines for TC-history-direct-write-fields-are-labelled-lines: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-direct-write-fields-are-labelled-lines]
case_digest: [ba6a5b94f5f8fca8e9c069f8b5c873a659ced3051402ef9cdb98b545a3a5e576]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - direct_writes
  - "--"
  - "--exact"
  - a_direct_write_is_written_as_labelled_lines
exit_code: 101
tests_run: 1
duration_seconds: 9
revision: da98f439e0a8db2a2c9cde127a83426d96730f49
content: 936e291c15b0e4e6a4c5e2b1f42d6ba1a17ecca8018baad29bbbaba7196270f8
environment: local-macos-arm64
fingerprint: sha256:dbc7d698d18e4033892e2c4b81303c919fa2a95d15404e910c0eba05bdf35f6a
criteria: [LIFE-2]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:48:07Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_is_written_as_labelled_lines for TC-history-direct-write-fields-are-labelled-lines: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da98f439e0a8, observed 2026-11-12T16:48:07Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_direct_write_is_written_as_labelled_lines stdout ----
- thread 'a_direct_write_is_written_as_labelled_lines' (919501231) panicked at tests/direct_writes.rs:370:5:
- ---
- kind: session
- id: 2026-11-12-direct-d1
- aliases: [direct-d1]
- objective: Watch d1
- intent_scope: ""
- modules: []
- stage: intent
- stage_status: active
- iteration: 3
- callers: []
- native_change: d1
- pr: ""
- criteria: []
- requirements: []
- cases: []
- design: []
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 6.97s
