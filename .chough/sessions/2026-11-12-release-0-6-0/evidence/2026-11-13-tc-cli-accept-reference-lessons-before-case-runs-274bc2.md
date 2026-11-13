---
kind: evidence
id: 2026-11-13-tc-cli-accept-reference-lessons-before-case-runs-274bc2
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:b4f4cdf3f5786f83616d4094880fb5512072225b479301298a7aae36f78f1145
criteria: [FLOW-18, LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:21:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:21:45Z. The raw log stays in the machine-local store.
