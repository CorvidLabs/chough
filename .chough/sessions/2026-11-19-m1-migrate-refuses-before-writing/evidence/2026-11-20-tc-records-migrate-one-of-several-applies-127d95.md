---
kind: evidence
id: 2026-11-20-tc-records-migrate-one-of-several-applies-127d95
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-one-of-several-applies]
case_digest: [2612e544ae7561ba82fc9ec204edec2b97fa28464c621b5a215132c5d2bc0b18]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - of_three_applies_of_one_digest_only_one_applies
exit_code: 0
tests_run: 1
duration_seconds: 14
revision: 37c5c146ce00ca337d68c77162cb616f6d50e257
content: 8be334f77be8e9003dc6ebc2f8c942de8447f5cbbffa4a36fc769354f985c996
environment: local-macos-arm64
fingerprint: sha256:a4367c0389310f0e497f50f0ccd59b371d5b8e82b3e4be980424152093bbe042
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:36:18Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 37c5c146ce00, observed 2026-11-20T02:36:18Z. The raw log stays in the machine-local store.
