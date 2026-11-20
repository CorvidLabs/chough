---
kind: evidence
id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-941dd9
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-the-fix-names-the-journal-post]
case_digest: [2c32119ea0e9c733d9858ec464529dce4f8dccf61b3b23374207a309994e4dbd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - the_unfinished_archive_fix_names_the_journal_post
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: 01593390f15cb0ca05623b83d99173b1d09e9dba
content: 135121ca601cf4d173f91cd8362805a9b0f4c7a2c2fd10e09b1be2c086170999
environment: local-macos-arm64
fingerprint: sha256:afe31bb11024c7218d5f3a8acf2a53018d7e78cdffdbf7736ef7ad6be2732c1a
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T21:09:50Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 01593390f15c, observed 2026-11-20T21:09:50Z. The raw log stays in the machine-local store.
