---
kind: evidence
id: 2026-11-17-tc-cli-change-ship-prints-a-short-summary-and-its-reports
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
duration_seconds: 0
revision: 5cb7e11a28236e4254d6d94a5f98f0b68e846b14
content: 9952521b4170d0b90075273964e9202e3561dd5d2d8e153e6df5516034b93734
environment: local
fingerprint: sha256:a909de55e2fc71b5c82a531a67df4bef63c73cadffde3cdce07a4a6aae13cf35
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-17T14:22:18Z
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
**Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1).**

Recorded as passed in local at revision 5cb7e11a2823, observed 2026-11-17T14:22:18Z. The raw log stays in the machine-local store.
