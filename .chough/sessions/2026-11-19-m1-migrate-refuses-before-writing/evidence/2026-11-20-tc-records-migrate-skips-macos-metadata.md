---
kind: evidence
id: 2026-11-20-tc-records-migrate-skips-macos-metadata
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-skips-macos-metadata]
case_digest: [da5102d13f5a8ae9701fd4461c1d13f985754f77525b3bdb8205abccbf621e5d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - migrate_skips_macos_metadata_beside_0_5_records
exit_code: 0
tests_run: 1
duration_seconds: 4
revision: 4c7035d55758c635b2f8a98e3f425e3ca9e64485
content: ff0b856dc4058727d416cf8d496078c404e4db6a97ee484547c71978d5b7a241
environment: local-macos-arm64
fingerprint: sha256:ca40d21a5b678947fcf491679493cf614be8389ae23eaafa74916cd8ff28209c
criteria: [DRIVE-14]
requirements: [REQ-records-015]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:00:16Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 4c7035d55758, observed 2026-11-20T04:00:16Z. The raw log stays in the machine-local store.
