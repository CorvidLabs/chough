---
kind: evidence
id: 2026-11-18-tc-cli-journal-add-holds-the-writer-lock-with-no-chough-2cdcfe
aliases: []
claim: "Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory]
case_digest: [28f1b67751997c8ab2317cd79e35944b6200f2e7c4ea66e276b90685dc273abd]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - journal
  - "--"
  - "--exact"
  - journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory
exit_code: 0
tests_run: 1
duration_seconds: 1
revision: e157bdd269e733a5b42eb453598c629bc4565065
content: fee319a43a0114520f76b43153ba65fe2ff77388873fc8d06a9cc045d7745f8c
environment: local-macos-arm64
fingerprint: sha256:02fc65ea289e12ec0c736e2732a62fd049d814595f0dfd1685c7bdbe9500daca
criteria: [DRIVE-11]
requirements: [REQ-cli-040]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-18T05:58:17Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran cargo test --locked --test journal -- --exact journal_add_takes_the_writer_lock_in_a_project_with_no_chough_directory for TC-cli-journal-add-holds-the-writer-lock-with-no-chough-directory: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision e157bdd269e7, observed 2026-11-18T05:58:17Z. The raw log stays in the machine-local store.
