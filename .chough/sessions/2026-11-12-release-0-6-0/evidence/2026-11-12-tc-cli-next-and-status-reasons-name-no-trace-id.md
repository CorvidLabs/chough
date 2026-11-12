---
kind: evidence
id: 2026-11-12-tc-cli-next-and-status-reasons-name-no-trace-id
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-and-status-reasons-name-no-trace-id]
case_digest: [c1b8719fd99cd1139582c7205c4fda82f746a4fdfcb338cbc3694e73381b3ee9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_and_status_reasons_name_no_trace_id
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:c6db9e4605981f314aab1da546fcd7f0d9f3ae00890de4bf2c918863696cf307
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:30Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_and_status_reasons_name_no_trace_id for TC-cli-next-and-status-reasons-name-no-trace-id: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:30Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- next_and_status_reasons_name_no_trace_id stdout ----
- thread 'next_and_status_reasons_name_no_trace_id' (924177027) panicked at tests/next.rs:2572:9:
-   Why: Native change is a complete draft waiting for its definition approval, which only the human gives (FLOW-23).
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 29 filtered out; finished in 1.71s
