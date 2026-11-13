---
kind: evidence
id: 2026-11-13-tc-history-story-receipts-last-and-bugs-named-282afe
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-story-receipts-last-and-bugs-named]
case_digest: [bcd5cb75ed64eb357408b3541926b37728275d7998b0c7a4fb696fbc519cfba7]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_puts_its_receipts_last_and_names_its_bugs
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:956359f2b9302d17bfc241542872b093543e5324c35c4a4b6375f9e71ffbf209
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:51:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:51:49Z. The raw log stays in the machine-local store.
