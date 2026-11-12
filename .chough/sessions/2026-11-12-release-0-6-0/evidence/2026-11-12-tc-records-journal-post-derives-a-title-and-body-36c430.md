---
kind: evidence
id: 2026-11-12-tc-records-journal-post-derives-a-title-and-body-36c430
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
duration_seconds: 1
revision: ac0cafef56b124ae2047f3f9c9798fa216c40551
content: b7604bbec7da005916f03fc30fbfe1cb2327c04a826f4446bfaa33fc39551f81
environment: local-macos-arm64
fingerprint: sha256:f2402f86063a7553e64157a964aeda0c4cc31980a4651793a42d910fa6d28d5a
criteria: [KNOW-33]
requirements: [REQ-records-011]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:41:40Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test journal -- --exact without_a_title_or_summary_the_post_derives_both_from_the_story for TC-records-journal-post-derives-a-title-and-body: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision ac0cafef56b1, observed 2026-11-12T17:41:40Z. The raw log stays in the machine-local store.
