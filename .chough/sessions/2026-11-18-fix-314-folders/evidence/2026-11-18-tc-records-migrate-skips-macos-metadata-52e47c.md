---
kind: evidence
id: 2026-11-18-tc-records-migrate-skips-macos-metadata-52e47c
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
duration_seconds: 1
revision: b2a227e3aa07fed5bf465dca8c70a8c35c4476ca
content: df08cfa494322a8c467b5c1102d3efa444bdae2cd5fb1b4f49f9bdaa6d72c9d2
environment: local-macos-arm64
fingerprint: sha256:f06308b4c092758f4497660f8e8a1d6fabf8a9ee1cf89613a992d9deb9748f75
criteria: [DRIVE-14]
requirements: [REQ-records-015]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T18:21:06Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b2a227e3aa07, observed 2026-11-18T18:21:06Z. The raw log stays in the machine-local store.
