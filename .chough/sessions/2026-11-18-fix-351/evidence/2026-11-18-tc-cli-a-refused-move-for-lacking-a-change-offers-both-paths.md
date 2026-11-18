---
kind: evidence
id: 2026-11-18-tc-cli-a-refused-move-for-lacking-a-change-offers-both-paths
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
revision: df6c8df004c0dc1565a2f22c4432dd56240831f1
content: f4968e711639f3528e548a943b725db442ae8401864c38684f8e08ea6e1b80c7
environment: local-macos-arm64
fingerprint: sha256:fc1f4074aeab6a99a11db780ebbc657d9227142ac1d78f9314afcb3168b5a211
criteria: [DRIVE-16]
requirements: [REQ-cli-021]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T22:57:58Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test stages -- --exact a_move_refused_for_lacking_a_change_offers_both_paths for TC-cli-a-refused-move-for-lacking-a-change-offers-both-paths: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision df6c8df004c0, observed 2026-11-18T22:57:58Z. The raw log stays in the machine-local store.
