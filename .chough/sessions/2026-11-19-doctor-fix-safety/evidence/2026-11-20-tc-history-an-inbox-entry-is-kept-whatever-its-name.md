---
kind: evidence
id: 2026-11-20-tc-history-an-inbox-entry-is-kept-whatever-its-name
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-inbox-entry-is-kept-whatever-its-name]
case_digest: [239e62b0874ea258aaed6e5e48acb7208fe7fe805f843405ce534ae27c6cf4b9]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_inbox_entry_is_kept_whatever_its_name
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:400aa9a143d311a0bff1a60aa9258ff5f31e666f3f354178c5fa2535f08f2a1a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:39Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_inbox_entry_is_kept_whatever_its_name for TC-history-an-inbox-entry-is-kept-whatever-its-name: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:39Z. The raw log stays in the machine-local store.
