---
kind: evidence
id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-a90344
aliases: []
claim: "Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 22568b99de030da4067b1accfac83e2c91d9e993
content: 9a72d59c509bca86133978ea0ed478c12eea1e036835535143167fc9eaabbbc3
environment: local-macos-arm64
fingerprint: sha256:b948e881caf978de10fe5b323d8d5af5204d7a0a13915c0571296eba4d40a8bb
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:33:25Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 22568b99de03, observed 2026-11-13T02:33:25Z. The raw log stays in the machine-local store.
