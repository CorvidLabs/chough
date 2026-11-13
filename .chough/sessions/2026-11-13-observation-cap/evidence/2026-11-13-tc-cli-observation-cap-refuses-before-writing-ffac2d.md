---
kind: evidence
id: 2026-11-13-tc-cli-observation-cap-refuses-before-writing-ffac2d
aliases: []
claim: "Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-observation-cap-refuses-before-writing]
case_digest: [922a8c1ea9a4708af23602a00ee5e79158af7a0df3cdb98152c5da944e2b5f33]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: d247ac6b17a0a48cfa00ece86730ed758d212389
content: f1693f112aee648629016053eeb1da4f43148a8d7945c6bee3912127fc705702
environment: local-macos-arm64
fingerprint: sha256:61141cd0e00a22a3797451bb23f85e8f3c21edf387bfd051b6bad3d9eac990f7
criteria: [DRIVE-14, FLOW-13]
requirements: [REQ-cli-032]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:59:54Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision d247ac6b17a0, observed 2026-11-13T02:59:54Z. The raw log stays in the machine-local store.
