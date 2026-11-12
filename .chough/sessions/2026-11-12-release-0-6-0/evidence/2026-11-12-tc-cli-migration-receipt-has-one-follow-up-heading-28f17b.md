---
kind: evidence
id: 2026-11-12-tc-cli-migration-receipt-has-one-follow-up-heading-28f17b
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-cli-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-migration-receipt-has-one-follow-up-heading]
case_digest: [c0b53bced8467534fb0a3f914e0d32e2705e82699ccb81d1d0f19fbdccf76697]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_receipt_has_one_follow_up_heading_for_every_follow_up
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 34b7cbb75288a57e7249c1bc50eb3354441840b6
content: 1b9f7fae45c3596bc36a084cc2c4b391a1bb119b03fd75d07f671e11dedfaf1d
environment: local-macos-arm64
fingerprint: sha256:e0948b541003b19a9cc931fff2d0b0a0289ce356f36afb67c4089956cc51290d
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:27:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_receipt_has_one_follow_up_heading_for_every_follow_up for TC-cli-migration-receipt-has-one-follow-up-heading: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 34b7cbb75288, observed 2026-11-12T20:27:03Z. The raw log stays in the machine-local store.
