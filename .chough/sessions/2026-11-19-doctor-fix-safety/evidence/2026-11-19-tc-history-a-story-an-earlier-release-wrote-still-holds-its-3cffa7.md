---
kind: evidence
id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-3cffa7
aliases: []
claim: "Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries]
case_digest: [519c9da40455e7b15f76ac7bafd083659e6d02313e2d8cb0e4544c18738cc147]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - sessions
  - "--"
  - "--exact"
  - a_story_an_earlier_release_wrote_still_holds_its_entries
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 3aed838ee2d777daba6dc4257a0b641b7cd52c91
content: 9a01fc68881f7303000b81075fabaf9ee48bc10b10288863aac7e6acaaf65b24
environment: local-macos-arm64
fingerprint: sha256:b81dae5e22bac26c62773278a9ed1cb7af37d27546158d941f9413aac2638888
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T19:43:58Z
actor:
  - agent: claude-code
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 3aed838ee2d7, observed 2026-11-19T19:43:58Z. The raw log stays in the machine-local store.
