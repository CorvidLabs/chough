---
kind: evidence
id: 2026-11-12-tc-cli-next-with-no-native-change-says-only-none-recorded
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-next-with-no-native-change-says-only-none-recorded]
case_digest: [ff9ea039288d96b4c88d8ecbe32f2d0826b193e7e17585e2056f78a79036fb4e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_with_no_native_change_says_only_that_none_is_recorded
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 5195bd6774255b7b090d51e37ac362f57fdda2e3
content: ab5a1023f46b5eb925e66eb0451bfdf545f677a3592abbfceebad0d2f81c46c0
environment: local-macos-arm64
fingerprint: sha256:81d252fcd57882da9f5d2a5c7e380b02441f22d3ba9569d8e42e3a105c84b004
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:01:26Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact next_with_no_native_change_says_only_that_none_is_recorded for TC-cli-next-with-no-native-change-says-only-none-recorded: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 5195bd677425, observed 2026-11-12T21:01:26Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- next_with_no_native_change_says_only_that_none_is_recorded stdout ----
- thread 'next_with_no_native_change_says_only_that_none_is_recorded' (924173877) panicked at tests/next.rs:2522:9:
- assertion `left == right` failed: Objective: Ship a focused change
- Stage: intent (active, iteration 1)
- Native change: none recorded (read not-configured)
-   Approved deltas: not checked: no native lifecycle
- Next: Capture or read the intent: run `hi check` to see the human-intent criteria and their state, then `chough context` for this session's bounded context, and reference the criteria the work covers with `chough checkpoint --criterion <id>`.
-   Why: No criteria are referenced yet; the work starts from the human's intent in HI.
-   Run: hi check
- Gate: none for the next stage
- Human gates: none
- Blockers: none
- Open questions: none
- Blocking bugs: none (0 other open; chough bug list)
- Observations needing attention: none
-   0 referenced current, 0 unreferenced.
- Decisions: none
- Last failure: none
- Lesson sources (1 of 1 modules):
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.98s
