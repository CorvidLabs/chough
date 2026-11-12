---
kind: evidence
id: 2026-11-12-tc-history-direct-write-entry-says-what-changed-ff8f62
aliases: []
claim: "Ran cargo test --locked --test direct_writes -- --exact a_direct_write_entry_says_what_changed_outside_chough for TC-history-direct-write-entry-says-what-changed: passed (exit 0, tests run 1)."
result: passed
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
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e8bde99c60a7555434694e8ffadf1bb28fa25fda
content: 3ee3e75bcaf0a92f7bc5ca41f59bd421498d8a26342f52e2d390ef2dd4377b54
environment: local-macos-arm64
fingerprint: sha256:112befad65492c22f351eca61696d00dca7b1ef90573b9376f72e7d603962331
criteria: [DRIVE-21]
requirements: [REQ-history-013]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:39:37Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test direct_writes -- --exact a_direct_write_entry_says_what_changed_outside_chough for TC-history-direct-write-entry-says-what-changed: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e8bde99c60a7, observed 2026-11-12T17:39:37Z. The raw log stays in the machine-local store.
