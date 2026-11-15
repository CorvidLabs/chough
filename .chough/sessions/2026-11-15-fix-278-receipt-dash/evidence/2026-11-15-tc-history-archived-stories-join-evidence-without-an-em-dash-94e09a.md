---
kind: evidence
id: 2026-11-15-tc-history-archived-stories-join-evidence-without-an-em-dash-94e09a
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
duration_seconds: 1
revision: 2a4c1d6bc7edd2c38796e417603ac87fe4335775
content: 001e32db55bf23251f55a66232187ab8797bcc93eb57a584e480d67bbc9364e0
environment: local-macos-arm64
fingerprint: sha256:4ae983ffc913caf2cb3f9c979db9ee05b5ac19414b30ed98ad146493cb257c5e
criteria: [DRIVE-7]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-15T03:39:12Z
actor:
  - agent: claude-code
    terminal: t28
---
**Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-archived-stories-join-evidence-without-an-em-dash: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 2a4c1d6bc7ed, observed 2026-11-15T03:39:12Z. The raw log stays in the machine-local store.
