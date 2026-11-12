---
kind: evidence
id: 2026-11-12-tc-cli-stage-move-prints-a-short-summary-6637e4
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
revision: 0ad3ad56cf7f5a714f73689a12eabff5ff8f0c31
content: 1c36fe8e52a5538a8198aff1428542f41b658e09f2d3847acd086cdac11cdac0
environment: local-macos-arm64
fingerprint: sha256:4c129c1c2cffa5938d08aa8bbdf0ace5177dc1e69f84a53a9d17aa3d9b476c3c
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:34:38Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0ad3ad56cf7f, observed 2026-11-12T22:34:38Z. The raw log stays in the machine-local store.
