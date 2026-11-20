---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase
aliases: []
claim: "Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-change-ready-counts-only-own-work-after-a-rebase]
case_digest: [e001c17066461d24c97a6896e5db031cfcf315baae14097591a43c893633e734]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - ready
  - "--"
  - "--exact"
  - after_a_rebase_onto_main_only_the_change_s_own_files_are_named
exit_code: 0
tests_run: 1
duration_seconds: 2
revision: 874d3a4e8cb923c923381debbcc7e45603fc0f7b
content: 360b47091367fe398daf5ee42a7b0f7e64c534cf2947b0f34445012bdb69d358
environment: local-macos-arm64
fingerprint: sha256:4fe4b367a97ca4928652a424120ff2dfe04cb4459a523d5a999e9c91f5fc2603
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T04:32:36Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 874d3a4e8cb9, observed 2026-11-20T04:32:36Z. The raw log stays in the machine-local store.
