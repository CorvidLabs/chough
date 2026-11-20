---
kind: evidence
id: 2026-11-20-tc-records-a-stored-inbox-entry-folds-printable
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
revision: 00a1e08cb8f6b4ffdf4edb86da174026aea4af28
content: 6d7e648114b3f58208380f34d2d3ced60fd90f643b16c09e7b493523e2185399
environment: local-macos-arm64
fingerprint: sha256:fe845b5f3a630910fa4cabe7d49ece327a73f220b08212913e3bd21923ca5c5b
criteria: [LIFE-26, LIFE-3]
requirements: [REQ-records-016]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T05:42:34Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test output_safety -- --exact a_stored_inbox_entry_with_a_control_character_folds_and_the_story_holds_none_raw for TC-records-a-stored-inbox-entry-folds-printable: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 00a1e08cb8f6, observed 2026-11-20T05:42:34Z. The raw log stays in the machine-local store.
