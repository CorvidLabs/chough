---
kind: evidence
id: 2026-11-20-tc-cli-change-ready-counts-only-own-work-after-a-rebase-306018
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
duration_seconds: 0
revision: 9c79d7694223aa3acc2027e8f95d03edb888ab16
content: a5023893b74f5a383e037635a88a59ffabfdf2c3a6e06852324b3460eec94e03
environment: local-macos-arm64
fingerprint: sha256:6e2c5bbee2997e262c2a1f0374023cb7a466fc2a691336e47be31fba30a4fe13
criteria: [DRIVE-29]
requirements: [REQ-cli-067]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:41:36Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
**Ran cargo test --locked --test ready -- --exact after_a_rebase_onto_main_only_the_change_s_own_files_are_named for TC-cli-change-ready-counts-only-own-work-after-a-rebase: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9c79d7694223, observed 2026-11-20T06:41:36Z. The raw log stays in the machine-local store.
