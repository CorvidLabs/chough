---
kind: evidence
id: 2026-11-20-tc-history-a-kept-entry-is-kept-by-its-file
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-kept-entry-is-kept-by-its-file]
case_digest: [768ee8a528719bfdbba66e84cc11491e82351b770ade2896855af81ee75fa3fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_kept_entry_is_kept_by_its_file_whatever_its_id
exit_code: 0
tests_run: 1
duration_seconds: 3
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:569f8657b90c8672422a924c63af5014953b2a667bebbdf164197696a0c5321a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:23Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_kept_entry_is_kept_by_its_file_whatever_its_id for TC-history-a-kept-entry-is-kept-by-its-file: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:23Z. The raw log stays in the machine-local store.
