---
kind: evidence
id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-446851
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
revision: 17f214884b36a9edd288d49c998da9712d7c4117
content: e771273e91fa0f9e7f25fa906ce9d06f49dc1e22605e38d848a7c09890b7aebd
environment: local-macos-arm64
fingerprint: sha256:bd24a404626c59b0440c7a9ee852ee89f7c864c545b282cc12c065b5680a8336
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:26:53Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 17f214884b36, observed 2026-11-20T06:26:53Z. The raw log stays in the machine-local store.
