---
kind: evidence
id: 2026-11-19-tc-history-next-names-finishing-the-archive
aliases: []
claim: "Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-next-names-finishing-the-archive]
case_digest: [070774317b2ea37170632459474d531da442a09db24605e729171fff018eeb1a]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - next
  - "--"
  - "--exact"
  - next_names_finishing_the_archive_until_the_folder_is_gone
exit_code: 0
tests_run: 1
duration_seconds: 6
revision: f09cf6ba0f1e40b8dab02f158f82c87adf20e886
content: 1224e0731b7c58721d3acad28242e08dd3912f98ed403faf2fc0391423a77a76
environment: local-macos-arm64
fingerprint: sha256:da3aa0fb5fadb792e773e055e66794f18a91c2968987044af784727afa9357ef
criteria: [DRIVE-17]
requirements: [REQ-history-006]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-19T05:38:18Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test next -- --exact next_names_finishing_the_archive_until_the_folder_is_gone for TC-history-next-names-finishing-the-archive: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f09cf6ba0f1e, observed 2026-11-19T05:38:18Z. The raw log stays in the machine-local store.
