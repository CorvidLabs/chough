---
kind: evidence
id: 2026-11-12-tc-cli-stage-move-prints-a-short-summary-a207e8
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-move-prints-a-short-summary]
case_digest: [7473a8983adb456754af04271c4b35af8b83d62bd6c38d629f07cf83e76d7f0e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_stage_move_prints_a_short_summary_and_its_full_report_under_json
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:f345bba3649dc990300dd54f59d7bebfec3a0f1f1c1bf8eb5d8ee2e484253b5a
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:13Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:13Z. The raw log stays in the machine-local store.
