---
kind: evidence
id: 2026-11-12-tc-cli-status-gives-the-next-action-whole
aliases: []
claim: "Ran cargo test --locked --test next -- --exact status_gives_the_next_valid_action_whole for TC-cli-status-gives-the-next-action-whole: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-status-gives-the-next-action-whole]
case_digest: [b684c5e4208964f55d65a715d31ef8bedf9bd03cebd5d42b444f909ac0176acb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - status_gives_the_next_valid_action_whole
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
content: 6287c845d27db39c417052a4608cf898a2c1017c06b5e7c461b069727288236f
environment: local-macos-arm64
fingerprint: sha256:9351a0853c1d60851d3cb2f895d460804fd60319166345e2742109d9adee2589
criteria: [FLOW-2, LIFE-3]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:42:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact status_gives_the_next_valid_action_whole for TC-cli-status-gives-the-next-action-whole: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision dd3e2ab5df32, observed 2026-11-12T17:42:40Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- status_gives_the_next_valid_action_whole stdout ----
- thread 'status_gives_the_next_valid_action_whole' (921047143) panicked at tests/next.rs:2362:5:
- assertion `left == right` failed: {
-   "blockers": {
-     "items": [],
-     "total": 0
-   },
-   "gates": [],
-   "native_change": null,
-   "next_valid_action": {
-     "action": "Continue build: Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against the scope before moving on. Check each SYNTHETIC part of the plan against  [cut]",
-     "command": null,
-     "reason": "No gate, human decision or blocking bug stops the current stage; this is the checkpoint's next action."
-   },
-   "stage": {
-     "current": "build",
-     "iteration": 1,
-     "status": "active"
-   }
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.69s
