---
kind: evidence
id: 2026-11-18-tc-records-migrate-skips-macos-metadata
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
revision: cef86e2622a2a908856b0c2cf11b29d3c1d3e472
content: 3f8a8cb3a0677ff393daf067ed1e1391667e306a3792a743d9ea67651492f153
environment: local-macos-arm64
fingerprint: sha256:eca92f21b7b7613fb173ae54fc6ddc42fb2159b5bd7f9bf06cd3777252cdf07e
criteria: [DRIVE-14]
requirements: [REQ-records-015]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T17:54:32Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test migrate -- --exact migrate_skips_macos_metadata_beside_0_5_records for TC-records-migrate-skips-macos-metadata: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision cef86e2622a2, observed 2026-11-18T17:54:32Z. The raw log stays in the machine-local store.
