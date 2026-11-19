---
kind: evidence
id: 2026-11-19-tc-history-an-untracked-kept-entry-is-committed-first
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-an-untracked-kept-entry-is-committed-first]
case_digest: [74983f76bcf414a85ba51e127ebc98e82f276b0bcda5fa400244b80d437759dd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - an_untracked_kept_entry_is_committed_first_so_its_way_out_runs
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:3878bc2dfbfcb43d5852245de3c3c072ad492bf3e155074176d03c2c470fc4ab
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:44:44Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:44:44Z. The raw log stays in the machine-local store.
