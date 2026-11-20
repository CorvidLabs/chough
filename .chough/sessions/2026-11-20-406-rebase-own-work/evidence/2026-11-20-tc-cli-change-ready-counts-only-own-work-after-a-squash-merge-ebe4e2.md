---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-ebe4e2
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
duration_seconds: 1
revision: ff13be893391d1866f1f881d014ec5d181c7e60f
content: 79d69754fcc4e759ac7714c82d28e0cf90a2841d03adf3a6e37f92b923396d4d
environment: local-macos-arm64
fingerprint: sha256:8a34995c581488a334fb0ea23f78152132fd867b91b92d4138c4abe58facb27c
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T14:14:27Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ff13be893391, observed 2026-11-20T14:14:27Z. The raw log stays in the machine-local store.
