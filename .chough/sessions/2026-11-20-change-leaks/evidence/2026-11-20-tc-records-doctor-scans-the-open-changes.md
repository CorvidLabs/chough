---
kind: evidence
id: 2026-11-20-tc-records-doctor-scans-the-open-changes
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
revision: 38a01b22378e70a778b05f5da6891e6d6b41e13f
content: 6938808c10085e6558eb80e4f8523360fcb44ba436559dcd6187620cc9cd6a65
environment: local-macos-arm64
fingerprint: sha256:90b78e02e14b0a131fc5e82bc547d77d3ad955b79ac1a7279156631eb467ad04
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:06:44Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 38a01b22378e, observed 2026-11-20T17:06:44Z. The raw log stays in the machine-local store.
