---
kind: evidence
id: 2026-11-20-tc-cli-g2-the-fix-names-the-journal-post
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
revision: 0cad78fd4d15d0f7246b8e879fb89b52135dbbbb
content: 5a6e7ccccfe323e16757d4386ae3850f4412d3c275cf05dc323e4ad1cb0e16a0
environment: local-macos-arm64
fingerprint: sha256:f297402455e16a111c4602b64a34ca13c260a018292921e133f21be78fb99674
criteria: [DRIVE-17]
requirements: [REQ-cli-055]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:43:28Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test doctor -- --exact the_unfinished_archive_fix_names_the_journal_post for TC-cli-g2-the-fix-names-the-journal-post: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 0cad78fd4d15, observed 2026-11-20T17:43:28Z. The raw log stays in the machine-local store.
