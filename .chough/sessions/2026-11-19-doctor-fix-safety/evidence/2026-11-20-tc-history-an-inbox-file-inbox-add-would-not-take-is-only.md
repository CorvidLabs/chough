---
kind: evidence
id: 2026-11-20-tc-history-an-inbox-file-inbox-add-would-not-take-is-only
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:4accd8c08219c1670d65fe761f182dbced495cdd30f87912d8dbe2aaf458d224
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:47Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_inbox_file_inbox_add_would_not_take_is_only_dropped for TC-history-an-inbox-file-inbox-add-would-not-take-is-only-dropped: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:47Z. The raw log stays in the machine-local store.
