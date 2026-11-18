---
kind: evidence
id: 2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths-8e78d9
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
revision: 26002fe92415e05be32c5575aa91ab8f5bd73aff
content: e66812ad32ab4ac8b2c0899a0728c4425d39d4f317f1831292d45895be8ef6a0
environment: local-macos-arm64
fingerprint: sha256:3452da1bc082108977f7c0516bc40f77a2d422862c3e39de723758b66ccdc468
criteria: [DRIVE-16]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T23:21:22Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 26002fe92415, observed 2026-11-18T23:21:22Z. The raw log stays in the machine-local store.
