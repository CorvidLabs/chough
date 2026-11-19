---
kind: evidence
id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-f81635
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
duration_seconds: 6
revision: 79a8e0d3d85b37c9bbaf466e54f175f2d02aebeb
content: 66171bdb992101909d26dfa6616a248b6cd59344a7962053de70742607fb3dae
environment: local-macos-arm64
fingerprint: sha256:b5cfa8761970d2ca51acbcd15ca63cc68d5c6173279a02c4fc56102d8c4d2c9a
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T16:09:54Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 79a8e0d3d85b, observed 2026-11-19T16:09:54Z. The raw log stays in the machine-local store.
