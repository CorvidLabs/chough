---
kind: evidence
id: 2026-11-12-tc-history-story-receipts-last-and-bugs-named-bca1ad
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:7ec191c6811e5c7f2585295a3970d4e892d31a53aa9af3788cb78734358574de
criteria: [LIFE-7, LIFE-3]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:26:53Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test sessions -- --exact a_story_puts_its_receipts_last_and_names_its_bugs for TC-history-story-receipts-last-and-bugs-named: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:26:53Z. The raw log stays in the machine-local store.
