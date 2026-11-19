---
kind: evidence
id: 2026-11-19-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped]
case_digest: [1665078ecc0a685a13e6b2748ca76bed04dc4928d894c6e2f1409c67d1692c0c]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_inbox_file_inbox_add_would_not_take_is_only_dropped
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:5ec1a8e4c1f9a2c061e83d1eef9f8f12cf36df42da3a8082e20ec08d4a991c8c
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:55Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:55Z. The raw log stays in the machine-local store.
