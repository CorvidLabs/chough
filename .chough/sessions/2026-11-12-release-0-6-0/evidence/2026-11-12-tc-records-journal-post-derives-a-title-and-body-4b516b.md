---
kind: evidence
id: 2026-11-12-tc-records-journal-post-derives-a-title-and-body-4b516b
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-post-derives-a-title-and-body]
case_digest: [06cccea745f8ce3a4b3adec1fc302844aeeeb3b1392f690b73e520f13889dc4d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - without_a_title_or_summary_the_post_derives_both_from_the_story
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:a2002bc876d449ee341b3a55c4fe1497d51adf8d28888dadb734299ce4a2eece
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:19:47Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:47Z. The raw log stays in the machine-local store.
