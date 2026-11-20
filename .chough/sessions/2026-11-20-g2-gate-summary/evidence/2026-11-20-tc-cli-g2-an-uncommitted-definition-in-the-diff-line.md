---
kind: evidence
id: 2026-11-20-tc-cli-g2-an-uncommitted-definition-in-the-diff-line
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval for TC-cli-g2-an-uncommitted-definition-in-the-diff-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-an-uncommitted-definition-in-the-diff-line]
case_digest: [67e7c44996f40f1e5d4511dc7ee370aa407802ca65c362e0732c7d6e905721db]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:df45e136025426f6755051fb5244ddee93b5231871f5cf1a00190ffc57e16036
criteria: [DRIVE-24]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:54Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test gates -- --exact an_uncommitted_definition_is_shown_by_the_diff_line_at_scope_approval for TC-cli-g2-an-uncommitted-definition-in-the-diff-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:54Z. The raw log stays in the machine-local store.
