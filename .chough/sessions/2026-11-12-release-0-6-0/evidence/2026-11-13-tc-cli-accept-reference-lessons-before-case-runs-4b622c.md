---
kind: evidence
id: 2026-11-13-tc-cli-accept-reference-lessons-before-case-runs-4b622c
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
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:b4f4cdf3f5786f83616d4094880fb5512072225b479301298a7aae36f78f1145
criteria: [FLOW-18, LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:53:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_accept_reference_writes_lessons_before_the_case_runs for TC-cli-accept-reference-lessons-before-case-runs: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:53:40Z. The raw log stays in the machine-local store.
