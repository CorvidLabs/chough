---
kind: evidence
id: 2026-11-13-tc-cli-stage-move-prints-a-short-summary-da0dde
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
duration_seconds: 2
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:8520d578d3c53c2544d98d5510bb646aa08b8b65f6da9f7313ceec9e3951b9cd
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:48:55Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:48:55Z. The raw log stays in the machine-local store.
