---
kind: evidence
id: 2026-11-12-tc-history-direct-write-entry-says-what-changed
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_entry_says_what_changed_outside_chough for TC-history-direct-write-entry-says-what-changed: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-direct-write-entry-says-what-changed]
case_digest: [91cd05925e21d08f5c815947c7c11011dba490981d5599b6ae688b42387dd512]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - direct_writes
  - "--"
  - "--exact"
  - a_direct_write_entry_says_what_changed_outside_chough
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: c7cefe408f5e9b926ffab814452bb27ecf044d71
content: 689c6fe45ce152ebad43eda1f8c4b2b202ff0cf2434882b5ae05929bef8bbbb8
environment: local-macos-arm64
fingerprint: sha256:b77ea2b357babb59b9ca833b44be5070c6b203e71dd509bc274687e9c4a9ff57
criteria: [DRIVE-21]
requirements: [REQ-history-013]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:37:29Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_entry_says_what_changed_outside_chough for TC-history-direct-write-entry-says-what-changed: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision c7cefe408f5e, observed 2026-11-12T17:37:29Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_direct_write_entry_says_what_changed_outside_chough stdout ----
- thread 'a_direct_write_entry_says_what_changed_outside_chough' (920974372) panicked at tests/direct_writes.rs:179:5:
- assertion `left == right` failed: [
-     Object {
-         "at_unix_seconds": Number(1794505047),
-         "direct_write": Object {
-             "after_state": String("verifying"),
-             "before_state": String("approved"),
-             "change": String("d1"),
-             "files": Array [
-                 Object {
-                     "change": String("changed"),
-                     "path": String("state.json"),
-                 },
-                 Object {
-                     "change": String("added"),
-                     "path": String("verification.json"),
-                 },
-             ],
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 1.08s
