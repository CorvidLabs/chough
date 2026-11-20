---
kind: evidence
id: 2026-11-20-tc-history-the-way-out-names-the-open-sessions
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-the-way-out-names-the-open-sessions]
case_digest: [ea61f8b1731f5ac470892706e868693c5ee7e9fbc9d462a5748f39380f4be02f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - the_way_out_names_the_open_sessions_when_none_is_active
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:fee6ad68e2b7df69502c49bbaa4935598934ebe926baae8deb80027e8ba314bf
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:31Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact the_way_out_names_the_open_sessions_when_none_is_active for TC-history-the-way-out-names-the-open-sessions: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:31Z. The raw log stays in the machine-local store.
