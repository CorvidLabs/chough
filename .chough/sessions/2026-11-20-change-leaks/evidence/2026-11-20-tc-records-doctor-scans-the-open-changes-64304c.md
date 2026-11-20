---
kind: evidence
id: 2026-11-20-tc-records-doctor-scans-the-open-changes-64304c
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
revision: 8ead84204edc55fcf5c30db35fb0beb6b7290cb2
content: 6880a79a758fb134e06561e2e779a9d6a9025ddc5484976b625f5c52cc4b2dea
environment: local-macos-arm64
fingerprint: sha256:cda11d8f0c83f7f47b8d8a6210ff8a57a63add40fd1e60eb5bfc88d91fa4d46c
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T17:33:58Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test leaks -- --exact doctor_scans_the_open_changes_but_not_their_deltas_or_the_archive for TC-records-doctor-scans-the-open-changes: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 8ead84204edc, observed 2026-11-20T17:33:58Z. The raw log stays in the machine-local store.
