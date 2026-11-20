---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-counts-only-own-work-after-a-squash-merge]
case_digest: [6c4edb31439a53fd6a2068ef21b1b47fa6479329c6ed274e245c3f9170e6d8c8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - after_a_squash_merge_of_main_only_the_change_s_own_files_are_named
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
environment: local-macos-arm64
fingerprint: sha256:d54be2eba13980c57856726b4cea3b43a44236166d6028e00f1f30b27cdd7bd8
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:32:41Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 874d3a4e8cb9, observed 2026-11-20T04:32:41Z. The raw log stays in the machine-local store.
