---
kind: evidence
id: 2026-11-13-tc-cli-next-with-no-open-session-says-so-and-exits-0
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-with-no-open-session-says-so-and-exits-0]
case_digest: [3b27295d134205f159c1e25e54aac9aedf67971962308394c42ad1aa59335322]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_with_no_open_session_says_so_and_exits_0
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 1a57d6f323ee083aac99cc100762d0169f2a7c2a
content: 4a0504b923b86eb60de2e722b0fa05dcd4ce59cdea6d267a4415927f86ca2f33
environment: local-macos-arm64
fingerprint: sha256:8edd6b839e7113d19e3b5c56c20f6a8bf30e0cf17b6856d43dbfbf6b4fd980e1
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:56:52Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_open_session_says_so_and_exits_0 for TC-cli-next-with-no-open-session-says-so-and-exits-0: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 1a57d6f323ee, observed 2026-11-13T02:56:52Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- next_with_no_open_session_says_so_and_exits_0 stdout ----
- thread 'next_with_no_open_session_says_so_and_exits_0' (932269642) panicked at tests/next.rs:3110:5:
- Error: no open session; run `chough session new <key>`
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.70s
