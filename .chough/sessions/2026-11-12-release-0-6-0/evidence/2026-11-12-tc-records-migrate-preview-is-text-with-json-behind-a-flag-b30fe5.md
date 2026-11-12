---
kind: evidence
id: 2026-11-12-tc-records-migrate-preview-is-text-with-json-behind-a-flag-b30fe5
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
revision: 67c851b9e28401894bddb7dd318026ff10274858
content: 95e94594ff41857662ceb749ae31317d4d0435443a173f10ab76dd036a6f8075
environment: local-macos-arm64
fingerprint: sha256:7942ba8777e545d28491c594f0b764658a2d03275f9221133c053a67cc8d510b
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:25:20Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_prints_the_plan_as_text_and_the_same_plan_as_json for TC-records-migrate-preview-is-text-with-json-behind-a-flag: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 67c851b9e284, observed 2026-11-12T23:25:20Z. The raw log stays in the machine-local store.
