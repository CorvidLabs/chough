---
kind: evidence
id: 2026-11-12-tc-records-a-0-5-pointer-still-names-its-session
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-0-5-pointer-still-names-its-session]
case_digest: [a0cbce8aeeb365c49bbebe626796911f3c14835366c3baccc51931b8411aaf9e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - a_pointer_naming_a_migrated_key_still_names_its_session
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 53f118505487a270e1b1f167e316960fe8d83d4f
content: 9c181bd1969b0d08d34f418cae377ba2e6dae39e8daf6c0a49747d0815955ce4
environment: local-macos-arm64
fingerprint: sha256:65c46703e17a694a7366875b84886bafc30b753d98a777c085cbd76766c180ed
criteria: [DRIVE-27, LIFE-20, LIFE-8, KNOW-30, LIFE-26]
requirements: [REQ-records-009, REQ-records-003]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T14:54:10Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact a_pointer_naming_a_migrated_key_still_names_its_session for TC-records-a-0-5-pointer-still-names-its-session: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 53f118505487, observed 2026-11-12T14:54:10Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_pointer_naming_a_migrated_key_still_names_its_session stdout ----
- thread 'a_pointer_naming_a_migrated_key_still_names_its_session' (917146384) panicked at tests/common/mod.rs:154:9:
- ["status", "--json"] failed: Error: the active session open-one does not exist; run `chough session use <id>` or `chough session new <key>`
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out; finished in 1.69s
