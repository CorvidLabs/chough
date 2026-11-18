---
kind: evidence
id: 2026-11-18-tc-records-a-changed-objective-is-in-the-post-and-one-story
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact a_changed_objective_is_in_the_post_and_one_story_line for TC-records-a-changed-objective-is-in-the-post-and-one-story-line: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-a-changed-objective-is-in-the-post-and-one-story-line]
case_digest: [67a469cc14b699b68c60a8fbef8ba05644b438ce80c860db0caf1c91f7906fae]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - a_changed_objective_is_in_the_post_and_one_story_line
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 8f3270abaedf56575c3afcc9999c43e6cf6cbd9b
content: 3d2ae70c3a11d751757441207841fab533825fc45c597c626b9b002c3931bb05
environment: local-macos-arm64
fingerprint: sha256:e4deec91fa9bf21235952e60bd663b73ddcd422756f1a1b05e9a55e9bce1e38f
criteria: [DRIVE-24]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T20:43:33Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test journal -- --exact a_changed_objective_is_in_the_post_and_one_story_line for TC-records-a-changed-objective-is-in-the-post-and-one-story-line: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8f3270abaedf, observed 2026-11-18T20:43:33Z. The raw log stays in the machine-local store.
