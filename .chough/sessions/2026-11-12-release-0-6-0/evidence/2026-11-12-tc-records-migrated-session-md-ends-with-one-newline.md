---
kind: evidence
id: 2026-11-12-tc-records-migrated-session-md-ends-with-one-newline
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact every_migrated_session_md_ends_with_exactly_one_newline for TC-records-migrated-session-md-ends-with-one-newline: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrated-session-md-ends-with-one-newline]
case_digest: [df8097e5bd7ff881583684e930cb50ae3770ff1f09119db34a7195a6e3eaef5a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - every_migrated_session_md_ends_with_exactly_one_newline
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: caa29f5e704c512b9af03b0ea33bdaaff1fc962c
content: 9db289c4ce1f8d5f1ad79c80bb5b911b1f95526ca85ae2f905c26e24c0c96e59
environment: local-macos-arm64
fingerprint: sha256:32e6a1b76687a6931e1a573b653f5f34a4474dd85e7cc72953fa7286ab35cb72
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:45:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact every_migrated_session_md_ends_with_exactly_one_newline for TC-records-migrated-session-md-ends-with-one-newline: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision caa29f5e704c, observed 2026-11-12T17:45:47Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- every_migrated_session_md_ends_with_exactly_one_newline stdout ----
- thread 'every_migrated_session_md_ends_with_exactly_one_newline' (921117015) panicked at tests/migrate.rs:1471:9:
- .chough/sessions/2026-11-11-open-one/session.md does not end with exactly one newline:
- ---
- kind: session
- id: 2026-11-11-open-one
- aliases: [open-one]
- objective: Open work one
- intent_scope: ""
- modules: []
- stage: design
- stage_status: active
- iteration: 1
- callers: []
- native_change: ""
- pr: ""
- criteria: []
- requirements: []
- cases: []
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 21 filtered out; finished in 1.66s
