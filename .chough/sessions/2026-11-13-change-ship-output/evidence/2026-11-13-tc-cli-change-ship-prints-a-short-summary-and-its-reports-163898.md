---
kind: evidence
id: 2026-11-13-tc-cli-change-ship-prints-a-short-summary-and-its-reports-163898
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
duration_seconds: 2
revision: d0fc7e970adb7ed2f55cc8c23c2bbb5d6112fcd9
content: b98eab13fdc77afc8782e0d29cb271be64181bed096c9014b27bebff794e3366
environment: local-macos-arm64
fingerprint: sha256:b4e03cee984ed03e5614e3ca8ab943d90a0b903991cf28ae3da2a9f7eb637a39
criteria: [DRIVE-20, FLOW-16]
requirements: [REQ-cli-059]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:41:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test change -- --exact ship_prints_a_short_summary_after_specsync_and_every_report_under_json for TC-cli-change-ship-prints-a-short-summary-and-its-reports-under-json: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d0fc7e970adb, observed 2026-11-13T02:41:36Z. The raw log stays in the machine-local store.
