---
kind: evidence
id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-7a867b
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
revision: b0006dd3acf176c6e05b870c0213084d93116a71
content: f8d2d03c6db5bb7203d64fcc49105b7ed4cb88919b89d4f43253279be8c49fa7
environment: local-macos-arm64
fingerprint: sha256:bfdae2a0504e2942b8b032e131b5abc596b8b7645f0fafe842cab87406d0b3bc
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T19:57:09Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision b0006dd3acf1, observed 2026-11-20T19:57:09Z. The raw log stays in the machine-local store.
