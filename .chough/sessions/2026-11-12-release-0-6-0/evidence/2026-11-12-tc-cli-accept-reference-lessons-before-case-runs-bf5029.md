---
kind: evidence
id: 2026-11-12-tc-cli-accept-reference-lessons-before-case-runs-bf5029
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-accept-reference-lessons-before-case-runs]
case_digest: [1131dd1ee910b5059c51b566e991ca0c9244d150b51e6bbcc8dfe5f5a2c542d8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_accept_reference_writes_lessons_before_the_case_runs
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 648ebb942879e453a90874b5e0ba3ea341515301
content: 1e3d4b1dc96fcd222b7b4c3cb1e922d5c0b93aa47190bb61424d2d99c9956761
environment: local-macos-arm64
fingerprint: sha256:b4f4cdf3f5786f83616d4094880fb5512072225b479301298a7aae36f78f1145
criteria: [FLOW-18, LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:55:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 648ebb942879, observed 2026-11-12T23:55:16Z. The raw log stays in the machine-local store.
