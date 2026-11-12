---
kind: evidence
id: 2026-11-12-tc-records-migrate-preview-is-text-with-json-behind-a-flag-b09962
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
duration_seconds: 1
revision: c6859854aa67f42db56cc283144390bd8928eeff
content: 2de52b4a5f7557f7543ac1a0a29f81b08d913318ccb4b2bf534e9bb3f3ce0614
environment: local-macos-arm64
fingerprint: sha256:ed5c565814517e85dbcc6507fba923297e22c5da205d3b49c103fbac52e41718
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:41:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision c6859854aa67, observed 2026-11-12T22:41:47Z. The raw log stays in the machine-local store.
