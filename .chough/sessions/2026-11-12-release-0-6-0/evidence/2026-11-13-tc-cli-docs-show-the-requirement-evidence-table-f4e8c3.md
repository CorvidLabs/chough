---
kind: evidence
id: 2026-11-13-tc-cli-docs-show-the-requirement-evidence-table-f4e8c3
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
duration_seconds: 1
revision: 1e53ea3564016f1bd193849ec0bd86e270fb455b
content: da4768cda3395502ad2da5d0c0d7dd5cf9bee4a4d0cdc715e4cd0b7b25c8a85e
environment: local-macos-arm64
fingerprint: sha256:4c33040730b560615425dcbb4972703e8b67b27aae70cec2848a62aae99e1893
criteria: [FLOW-18]
requirements: [REQ-cli-019]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T00:43:03Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test docs -- --exact the_requirement_evidence_table_in_a_changes_testing_md_is_documented for TC-cli-docs-show-the-requirement-evidence-table: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1e53ea356401, observed 2026-11-13T00:43:03Z. The raw log stays in the machine-local store.
