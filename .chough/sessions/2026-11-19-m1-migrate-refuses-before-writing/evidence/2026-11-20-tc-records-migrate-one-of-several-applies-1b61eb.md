---
kind: evidence
id: 2026-11-20-tc-records-migrate-one-of-several-applies-1b61eb
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
duration_seconds: 18
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:e006612157217d0f7db6226e2aa83b064f1a1786b2ee18b4f564407a21e0729c
criteria: [LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T03:59:36Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact of_three_applies_of_one_digest_only_one_applies for TC-records-migrate-one-of-several-applies: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T03:59:36Z. The raw log stays in the machine-local store.
