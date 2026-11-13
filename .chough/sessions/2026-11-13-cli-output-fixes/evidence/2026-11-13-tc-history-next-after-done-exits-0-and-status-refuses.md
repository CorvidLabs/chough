---
kind: evidence
id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-next-after-done-exits-0-and-status-refuses]
case_digest: [670778eb5f1bd0d61fd514d67b3ab56c48492a1b0ddc15f7321937a81f540712]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - after_the_move_into_done_next_exits_0_and_status_refuses
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
environment: local-macos-arm64
fingerprint: sha256:8f4abf71a0b70d530dc4f62ec40b47d57f496bbd8dc63b67ae1c17df87f971f3
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:56:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 1a57d6f323ee, observed 2026-11-13T02:56:55Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- after_the_move_into_done_next_exits_0_and_status_refuses stdout ----
- thread 'after_the_move_into_done_next_exits_0_and_status_refuses' (932270956) panicked at tests/common/mod.rs:191:9:
- ["next"] failed: Error: no open session; run `chough session new <key>`
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.93s
