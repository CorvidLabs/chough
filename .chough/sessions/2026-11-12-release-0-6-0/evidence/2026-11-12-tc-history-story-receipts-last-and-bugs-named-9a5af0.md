---
kind: evidence
id: 2026-11-12-tc-history-story-receipts-last-and-bugs-named-9a5af0
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
duration_seconds: 3
revision: 421a7034dfce0c052d48c4d1fb5abe041c272a46
content: 9e16bb2234c3aadbf27905868186d8ca01d209615750221de4f1f7201e15af54
environment: local-macos-arm64
fingerprint: sha256:956359f2b9302d17bfc241542872b093543e5324c35c4a4b6375f9e71ffbf209
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:06:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 421a7034dfce, observed 2026-11-12T23:06:03Z. The raw log stays in the machine-local store.
