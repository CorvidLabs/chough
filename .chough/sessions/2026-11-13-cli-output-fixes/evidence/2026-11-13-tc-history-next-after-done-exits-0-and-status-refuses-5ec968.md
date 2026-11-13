---
kind: evidence
id: 2026-11-13-tc-history-next-after-done-exits-0-and-status-refuses-5ec968
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
revision: bc275212d81eb8ac9c8498f189809431ab75340c
content: 289ed18f7eeb791ead6c5c6ddb8da209cbcde473ed6022b7b1aa7c4b4dbb7082
environment: local-macos-arm64
fingerprint: sha256:5dd4912480a0029ff98ebd3986541ac4f6f790689d49ded4dc017f146d561bfd
criteria: [DRIVE-13, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:01:01Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact after_the_move_into_done_next_exits_0_and_status_refuses for TC-history-next-after-done-exits-0-and-status-refuses: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision bc275212d81e, observed 2026-11-13T03:01:01Z. The raw log stays in the machine-local store.
