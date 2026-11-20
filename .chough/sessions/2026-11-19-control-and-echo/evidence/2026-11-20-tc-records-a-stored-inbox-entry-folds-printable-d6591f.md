---
kind: evidence
id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-d6591f
aliases: []
claim: "Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-stored-inbox-entry-folds-printable]
case_digest: [3481f41774e6a2e6e61190d52263a109b722dc7f01f03e6e88a0a0dbcc5b17ab]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - output_safety
  - "--"
  - "--exact"
  - a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 6816b8fe5e23cd58aa0d8f95511b325340cf16c6
content: b01158cb36daa642b345e51ec87b1143a917af4073ab0d63456b8e5c81d5f2d0
environment: local-macos-arm64
fingerprint: sha256:b81218fba6404c14e7e4e4a94ba047d6d3653479e86d11aa041148c36eea69aa
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:39:12Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 6816b8fe5e23, observed 2026-11-20T06:39:12Z. The raw log stays in the machine-local store.
