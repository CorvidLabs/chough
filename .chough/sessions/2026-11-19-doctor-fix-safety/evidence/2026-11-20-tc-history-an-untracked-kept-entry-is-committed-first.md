---
kind: evidence
id: 2026-11-20-tc-history-an-untracked-kept-entry-is-committed-first
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
revision: 53309b87af13170939e413a6513c16dc5063683c
content: 16c3fa6b5b94e0511c1ec6a3a5c26d3eed5a5161b6184275a82ed9750e962522
environment: local-macos-arm64
fingerprint: sha256:f5c568dc3363609fc4348bcc4205ee71a64e4806d0d595abb01a41777b8d9d4c
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T00:49:35Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact an_untracked_kept_entry_is_committed_first_so_its_way_out_runs for TC-history-an-untracked-kept-entry-is-committed-first: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 53309b87af13, observed 2026-11-20T00:49:35Z. The raw log stays in the machine-local store.
