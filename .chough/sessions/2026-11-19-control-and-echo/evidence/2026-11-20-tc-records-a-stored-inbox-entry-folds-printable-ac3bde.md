---
kind: evidence
id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable-ac3bde
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
revision: 1406385069c2d1509e6a06c6a4f7ddfc5b56e06b
content: 20189782d5a644a4b1a6ca488e5c698d73a13e0eaec50e3b56e637c50d6f7c52
environment: local-macos-arm64
fingerprint: sha256:e96af6d3d134082d068b4136291278a86d6ac3c638c65c8651c1923f30e9eaa4
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T06:12:40Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 1406385069c2, observed 2026-11-20T06:12:40Z. The raw log stays in the machine-local store.
