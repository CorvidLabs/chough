---
kind: evidence
id: 2026-11-13-tc-records-migrate-preview-is-text-with-json-behind-a-flag-d7e86b
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
revision: fd5397bb391b4a98db91f150e3979ebec4844955
content: baed51ebb0520a68d7f3eb166740be8bbf34626ce353e71d6c5482806b515a19
environment: local-macos-arm64
fingerprint: sha256:0d709f00924196c4a036404c498515c5b846b09950c903aef0c390e54c03218f
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T01:16:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision fd5397bb391b, observed 2026-11-13T01:16:22Z. The raw log stays in the machine-local store.
