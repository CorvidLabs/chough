---
kind: evidence
id: 2026-11-19-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths]
case_digest: [52a1a915c2a3438afea2893fb6cee8b6d7e5466e6fac6851a7c63ab0a4212c53]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_move_refused_for_lacking_a_change_offers_both_paths
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 45c0ed36c6982b20bee4f4fa21f3ea3177b9ea4a
content: 9dd0a31491eb10e45012c2bc9588d1164707a78ab8a58c9cfa860fb50870fa37
environment: local-macos-arm64
fingerprint: sha256:c13da6eea7ff3877564fa772d9c7c48c316d5dab50bf4364c38ea54c137e1668
criteria: [DRIVE-16]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T01:29:05Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 45c0ed36c698, observed 2026-11-19T01:29:05Z. The raw log stays in the machine-local store.
