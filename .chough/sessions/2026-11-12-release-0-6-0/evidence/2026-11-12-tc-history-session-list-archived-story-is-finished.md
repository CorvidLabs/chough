---
kind: evidence
id: 2026-11-12-tc-history-session-list-archived-story-is-finished
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact session_list_reports_an_archived_story_as_finished for TC-history-session-list-archived-story-is-finished: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-session-list-archived-story-is-finished]
case_digest: [a63202993d3780985a1745d25108fff21f3a3a1b774bb68ae73fe3b878b93c4e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - session_list_reports_an_archived_story_as_finished
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 731aba24c9db412aaa2f1b570043ee4c1f16cc31
content: 09ee87f20f595f31728e706969a0b939c75f822cf7699e157fdb40dda5f964d0
environment: local-macos-arm64
fingerprint: sha256:058a090c46692f92cc21cfddb6fddb3290c9351cc20ad2aebf54dbcbc138e78d
criteria: [DRIVE-16]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:08:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact session_list_reports_an_archived_story_as_finished for TC-history-session-list-archived-story-is-finished: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 731aba24c9db, observed 2026-11-12T20:08:46Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- session_list_reports_an_archived_story_as_finished stdout ----
- thread 'session_list_reports_an_archived_story_as_finished' (922358188) panicked at tests/sessions.rs:472:9:
- assertion `left == right` failed: alpha: {"active":false,"aliases":["alpha"],"archived":true,"file":".chough/archive/sessions/2026-11-12-alpha.md","finished":false,"fix":null,"id":"2026-11-12-alpha","inbox_unfolded":0,"native_change":null,"objective":"Ship alpha","stage":"intent","status":"archived"}
-   left: Bool(false)
-  right: true
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.87s
