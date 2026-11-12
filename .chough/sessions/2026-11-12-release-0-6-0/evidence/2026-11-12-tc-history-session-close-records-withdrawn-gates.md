---
kind: evidence
id: 2026-11-12-tc-history-session-close-records-withdrawn-gates
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact session_close_records_the_gates_it_withdraws for TC-history-session-close-records-withdrawn-gates: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-session-close-records-withdrawn-gates]
case_digest: [c4977c8214f983edfae431e9f8aa07532b708b053ed2d102ab5ee83385de500f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - session_close_records_the_gates_it_withdraws
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 95ba89c6bd2551a68f061cf7b821ffef07797de8
content: e9362dd93c20ea6a68dfe117857f15c18e59e55b18573097c69aa4303d3d5087
environment: local-macos-arm64
fingerprint: sha256:d37d213736f6867f1fe42981d3c13244af126bad9aaf835541dee91570625b7c
criteria: [DRIVE-16, FLOW-13]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:05:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact session_close_records_the_gates_it_withdraws for TC-history-session-close-records-withdrawn-gates: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 95ba89c6bd25, observed 2026-11-12T22:05:55Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- session_close_records_the_gates_it_withdraws stdout ----
- thread 'session_close_records_the_gates_it_withdraws' (925327715) panicked at tests/sessions.rs:737:5:
- assertion `left == right` failed: {"archived":".chough/archive/sessions/2026-11-12-alpha.md","closed":"2026-11-12-alpha","journal_post":{"post":"knowledge/journal/2026-11-12-ship-alpha.md","status":"written"},"kept":[],"librarian_index":{"index":"knowledge/index.md","status":"refreshed"},"reason":"Abandoned for beta","schema_version":1}
-   left: Null
-  right: Array [String("Approve the alpha intent")]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.92s
