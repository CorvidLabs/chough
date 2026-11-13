---
kind: evidence
id: 2026-11-13-tc-cli-observation-cap-refuses-before-writing-578f54
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
duration_seconds: 3
revision: 3df0102c1be7e9ea9e55af5aea6039b1562c1a3b
content: 08f9767107c068c9ef20761ffab965cbf0c69f2f26fe50f576742e92619c9082
environment: local-macos-arm64
fingerprint: sha256:20cef9eb058216430a0cb56fc597cb2db524e8c87697f35799be0f70771cb4e4
criteria: [DRIVE-14, FLOW-13]
requirements: [REQ-cli-032]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:41:18Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test next -- --exact at_the_observation_cap_writes_refuse_before_running_and_next_reads_only_referenced_runs for TC-cli-observation-cap-refuses-before-writing: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3df0102c1be7, observed 2026-11-13T02:41:18Z. The raw log stays in the machine-local store.
