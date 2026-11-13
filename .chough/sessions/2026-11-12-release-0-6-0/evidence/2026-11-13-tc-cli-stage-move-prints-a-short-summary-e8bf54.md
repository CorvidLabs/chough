---
kind: evidence
id: 2026-11-13-tc-cli-stage-move-prints-a-short-summary-e8bf54
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:a4e00a9eae1b8ca47e757cd505bc10001c2a67b6efa4f4215b2a1c28ca0cae52
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:16:05Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:16:05Z. The raw log stays in the machine-local store.
