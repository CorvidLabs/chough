---
kind: evidence
id: 2026-11-12-tc-records-journal-post-uses-the-title-and-summary-8c54e8
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-journal-post-uses-the-title-and-summary]
case_digest: [8d2a80ed3098d093d6da397a39bb07247a432b24a744f830df428f934cdc804d]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - the_post_uses_the_title_and_summary_the_agent_set
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 69b60a68d925da1c3a5654890bd62627599666bd
content: 18d24a04ea352eed30462b2a6d05214a9659b6d958ad33db5ee89f1b546be238
environment: local-macos-arm64
fingerprint: sha256:a1027ab57ac12951858ee63e5f659db36887cfbd8226ecae259f89cdfab41064
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T16:19:45Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact the_post_uses_the_title_and_summary_the_agent_set for TC-records-journal-post-uses-the-title-and-summary: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 69b60a68d925, observed 2026-11-12T16:19:45Z. The raw log stays in the machine-local store.
