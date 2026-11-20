---
kind: evidence
id: 2026-11-20-tc-records-migrate-one-of-several-applies
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
duration_seconds: 7
revision: 3f9f99c0b583daa04c613bbb5f32aa745d38c208
content: 1272150d359cc73b82685776425d73448f613f091c374b7cee06729875f291da
environment: local-macos-arm64
fingerprint: sha256:4cd51ce18eabb365fd00aaca12e814f59ba7f6c647e504ee82aba9235f22a5a6
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T02:24:11Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3f9f99c0b583, observed 2026-11-20T02:24:11Z. The raw log stays in the machine-local store.
