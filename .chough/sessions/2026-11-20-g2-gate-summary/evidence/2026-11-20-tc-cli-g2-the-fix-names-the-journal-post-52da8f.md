---
kind: evidence
id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post-52da8f
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
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:115dbddadec8dc9d88c4a26b6d42209199cca9f7be8ae539537b23d84d71f192
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:54Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:54Z. The raw log stays in the machine-local store.
