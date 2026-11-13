---
kind: evidence
id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-a6f491
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 4001748c5694f643883219e37d690c4271bfc064
content: 489d14fd5c396a13d756cc137bef855dfad477971358d696dc797684d4e06219
environment: local-macos-arm64
fingerprint: sha256:a36d80a35890677ef203302c5e2931e259b74c64e4cf9f0a18fa27fd2a2e847b
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:30:59Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4001748c5694, observed 2026-11-13T03:30:59Z. The raw log stays in the machine-local store.
