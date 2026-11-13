---
kind: evidence
id: 2026-11-13-tc-records-migrated-front-matter-names-the-carried-entries
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrated-front-matter-names-the-carried-entries]
case_digest: [31d3e1b6c6c9b6299ba3da51213934d7bb063b3a4911db955f6741f6fe01930b]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - migrated_front_matter_names_the_carried_entries
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: f4cfb57bd145d15ca613b2747d73c5372137538d
content: 7b5512beec4b9a2f80ee505be47bcd1cb09ac81b65b09d909d878177e022d9ed
environment: local-macos-arm64
fingerprint: sha256:2895191ebadad78d063af842b9792e41063514c6f60b20330a94dd9a8472f71d
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T03:11:31Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact migrated_front_matter_names_the_carried_entries for TC-records-migrated-front-matter-names-the-carried-entries: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision f4cfb57bd145, observed 2026-11-13T03:11:31Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- migrated_front_matter_names_the_carried_entries stdout ----
- thread 'migrated_front_matter_names_the_carried_entries' (932806980) panicked at tests/migrate_evidence.rs:1371:5:
- assertion `left == right` failed: ---
- kind: session
- id: 2026-11-11-fixture-first
- aliases: [fixture-first]
- objective: Fixture objective for v0.5.0
- intent_scope: FIXT
- modules: [fixture]
- stage: plan
- stage_status: active
- iteration: 1
- callers: []
- native_change: ""
- pr: ""
- criteria: []
- requirements: []
- cases: []
- design: []
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.99s
