---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-squash-merge-c155e2
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
duration_seconds: 0
revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
environment: local-macos-arm64
fingerprint: sha256:9132ab4629055d040be9a2808759f230caf1dd6a8640ece0d9e9af35ae3d93bf
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:41:38Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact after_a_squash_merge_of_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-squash-merge: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c79d7694223, observed 2026-11-20T06:41:38Z. The raw log stays in the machine-local store.
