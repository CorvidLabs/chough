---
kind: evidence
id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json]
case_digest: [76aff66faf917a27b83922b3af28cfb3cca4f54d8298905dc1fd0bf54d26f3e6]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - change
  - "--"
  - "--exact"
  - ship_prints_a_short_summary_after_specsync_and_every_report_under_json
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: c29cf1e653a31e63748d144c4bf1e22942d57a61
content: 0875b1cc72bf9eb8374f1ccf008ceb008770a5be3c01bf0dcae51c961b4c1f1d
environment: local-macos-arm64
fingerprint: sha256:44c704b7ed2479967990e480c81b83d5c5fc45d4daf477704004e8685db7873e
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:32:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c29cf1e653a3, observed 2026-11-13T02:32:47Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- ship_prints_a_short_summary_after_specsync_and_every_report_under_json stdout ----
- thread 'ship_prints_a_short_summary_after_specsync_and_every_report_under_json' (931167994) panicked at tests/change.rs:654:5:
- the summary is over 12 lines: {
-   "checkpoint": {
-     "awaiting_human": [],
-     "blockers": [],
-     "bugs": [],
-     "completed": [],
-     "decisions": [
-       "SYNTHETIC-human decided \"Implementation review of a5\": \"pass\" (a test); recorded by SYNTHETIC-agent; a relayed claim, not authenticated; ran specsync change review a5 as SYNTHETIC-reviewer with verdict pass"
-     ],
-     "intent_scope": "",
-     "modules": [],
-     "native_change": "a5",
-     "next_action": "Gate \"Implementation review of a5\" closed (review pass); the change shipped, so run `chough stage advance --to done`.",
-     "objective": "Ship a5",
-     "observations": [],
-     "open_questions": [],
-     "references": {
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out; finished in 1.36s
