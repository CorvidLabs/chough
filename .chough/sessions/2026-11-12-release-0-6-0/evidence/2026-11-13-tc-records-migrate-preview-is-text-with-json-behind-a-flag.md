---
kind: evidence
id: 2026-11-13-tc-records-migrate-preview-is-text-with-json-behind-a-flag
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-preview-is-text-with-json-behind-a-flag]
case_digest: [a94aad04371868914b182bd9fcaac5a8a6b8cc85a1683bbc206be23e8eacbd7b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_prints_the_plan_as_text_and_the_same_plan_as_json
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: dbbcedb9687d91c2eb4328b00edf282d81ff87a0
content: a562346fa153186af8a4211c9872f7d8112016b51c75614d53068c64ee3f4d6a
environment: local-macos-arm64
fingerprint: sha256:a5682cba93898eaf645b5e8687aa31254e379b39583bb25bfa1d2b6e6b5bfe0c
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:22:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision dbbcedb9687d, observed 2026-11-13T00:22:46Z. The raw log stays in the machine-local store.
