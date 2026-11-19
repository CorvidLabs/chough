---
kind: evidence
id: 2026-11-19-tc-history-a-story-an-earlier-release-wrote-still-holds-its-422bab
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
duration_seconds: 4
revision: 02935a6897b2e5fc68dcb45d16c711964d27eb18
content: 3ba9002823e59d59efd97bbc3b84db0935441318cad5138e4be5a6158a0549c8
environment: local-macos-arm64
fingerprint: sha256:54020b71c52d14080cc2b84b1c511278746f3491396822cd0fce2f240e980d32
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T15:13:52Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test sessions -- --exact a_story_an_earlier_release_wrote_still_holds_its_entries for TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 02935a6897b2, observed 2026-11-19T15:13:52Z. The raw log stays in the machine-local store.
