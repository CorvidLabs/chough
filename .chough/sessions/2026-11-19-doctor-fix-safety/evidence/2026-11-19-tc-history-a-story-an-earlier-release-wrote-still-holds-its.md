---
kind: evidence
id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its
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
revision: 44aec05c95e2c562e6f32be4225529a011e3ce84
content: 72698be58c6fcf4e09cd2c4ecd4df7955558b9b064e99d999d8f74e847af049a
environment: local-macos-arm64
fingerprint: sha256:42b6c9556ee992f68454edf9555ff8ba0250d390cbc2cc5af7ccfde33c418a1d
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T14:31:45Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 44aec05c95e2, observed 2026-11-19T14:31:45Z. The raw log stays in the machine-local store.
