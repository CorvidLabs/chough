---
kind: evidence
id: 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-archived-stories-join-evidence-without-an-em-dash]
case_digest: [4e818ba81bc305fd85ad341fdcd94a12acab6d81f15d107fb483233c3afb9c4f]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_lists_each_entry_once_with_its_receipt
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 9bdae1013b8a5dd5b009543d27ce846037200189
content: 2f78bb446b2610e5a31daa31d2b523f0bdb6210046a81665846ac17aa631edfe
environment: local-macos-arm64
fingerprint: sha256:1dce59554ece2cd6b0af9c4fb8de506a09d433b859b3ca46a8adda20e95cf054
criteria: [DRIVE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T03:04:34Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 9bdae1013b8a, observed 2026-11-15T03:04:34Z. The raw log stays in the machine-local store.
