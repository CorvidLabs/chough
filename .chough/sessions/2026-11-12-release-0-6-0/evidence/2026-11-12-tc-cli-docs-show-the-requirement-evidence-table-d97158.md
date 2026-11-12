---
kind: evidence
id: 2026-11-12-tc-cli-docs-show-the-requirement-evidence-table-d97158
aliases: []
claim: "Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-docs-show-the-requirement-evidence-table]
case_digest: [627d1961f56c1970aaebdfc5fa3382e7b8cf6ec7fb816dbcf8776b44859605fc]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - docs
  - "--"
  - "--exact"
  - the_requirement_evidence_table_in_a_changes_testing_md_is_documented
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 34b7cbb75288a57e7249c1bc50eb3354441840b6
content: 1b9f7fae45c3596bc36a084cc2c4b391a1bb119b03fd75d07f671e11dedfaf1d
environment: local-macos-arm64
fingerprint: sha256:821265359fadcc0db7e9d38cda2a3b801dd20f13e4cd20685f68be53fa67635e
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T20:27:11Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 34b7cbb75288, observed 2026-11-12T20:27:11Z. The raw log stays in the machine-local store.
