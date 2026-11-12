---
kind: evidence
id: 2026-11-12-tc-history-relay-fields-are-labelled-lines
aliases: []
claim: "Ran cargo test --locked --test relay -- --exact a_relayed_decision_is_written_as_labelled_lines_and_reads_back for TC-history-relay-fields-are-labelled-lines: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-relay-fields-are-labelled-lines]
case_digest: [7debd07a0eb61ea6cdb4dfbad84fc037f44a3ecfa21f3452a633ae39ee4ccd17]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - relay
  - "--"
  - "--exact"
  - a_relayed_decision_is_written_as_labelled_lines_and_reads_back
exit_code: 101
tests_run: 1
duration_seconds: 5
revision: da98f439e0a8db2a2c9cde127a83426d96730f49
content: 936e291c15b0e4e6a4c5e2b1f42d6ba1a17ecca8018baad29bbbaba7196270f8
environment: local-macos-arm64
fingerprint: sha256:fa729d2e8bd974f283e18fab6840938132611d0e706e2305d533cf053d9fbe78
criteria: [LIFE-2]
requirements: [REQ-history-002]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:47:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test relay -- --exact a_relayed_decision_is_written_as_labelled_lines_and_reads_back for TC-history-relay-fields-are-labelled-lines: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision da98f439e0a8, observed 2026-11-12T16:47:54Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_relayed_decision_is_written_as_labelled_lines_and_reads_back stdout ----
- thread 'a_relayed_decision_is_written_as_labelled_lines_and_reads_back' (919497028) panicked at tests/relay.rs:438:5:
- ---
- kind: session
- id: 2026-11-12-relay-decisions
- aliases: [relay-decisions]
- objective: Relay decisions
- intent_scope: ""
- modules: [cli]
- stage: intent
- stage_status: active
- iteration: 1
- callers: []
- native_change: ""
- pr: ""
- criteria: []
- requirements: []
- cases: []
- design: []
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 2.71s
