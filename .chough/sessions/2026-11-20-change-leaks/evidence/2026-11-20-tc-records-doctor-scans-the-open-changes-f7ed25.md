---
kind: evidence
id: 2026-11-20-tc-records-doctor-scans-the-open-changes-f7ed25
aliases: []
claim: "Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-doctor-scans-the-open-changes]
case_digest: [7942075b623823367ae7473a1ec6a904d54579623c7c34719dcf233654416fdb]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - leaks
  - "--"
  - "--exact"
  - doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f34dcc8391b47cd2b32c04a89933af75497bc177
content: 9c46ada16719ee7f5bc1b93a1452cb6032aad8f285116a7b70082427b6284a88
environment: local-macos-arm64
fingerprint: sha256:f90ac9567b2e23c3091f314eb787455893310e9046caede737e14f3bf14eea7e
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T22:15:36Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f34dcc8391b4, observed 2026-11-20T22:15:36Z. The raw log stays in the machine-local store.
