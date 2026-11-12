---
kind: evidence
id: 2026-11-12-tc-cli-a-close-that-runs-no-native-step-says-closed-once
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-close-that-runs-no-native-step-says-closed-once]
case_digest: [20bad69c45c115e0fe183b0deaaf610064a8c89df019b15543e396d911056e38]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - a_close_that_runs_no_native_step_says_closed_once
exit_code: 101
tests_run: 1
duration_seconds: 12
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:b4d06c9620f97292230fe9f8f627d15e369c75e9b34f7448d6f00842cff3036f
criteria: [FLOW-23]
requirements: [REQ-cli-050]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:21Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact a_close_that_runs_no_native_step_says_closed_once for TC-cli-a-close-that-runs-no-native-step-says-closed-once: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:21Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_close_that_runs_no_native_step_says_closed_once stdout ----
- thread 'a_close_that_runs_no_native_step_says_closed_once' (924157592) panicked at tests/humans.rs:851:9:
- assertion `left == right` failed: Gate "SYNTHETIC go-ahead" closed (closed); run `chough next` for the next action.
-   left: 2
-  right: 1
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 9.33s
