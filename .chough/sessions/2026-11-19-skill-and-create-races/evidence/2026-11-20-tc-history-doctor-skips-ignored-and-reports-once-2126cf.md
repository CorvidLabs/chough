---
kind: evidence
id: 2026-11-20-tc-history-doctor-skips-ignored-and-reports-once-2126cf
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-skips-ignored-and-reports-once]
case_digest: [bca77d93afd1d748cd865bf4ee61a60f234c7d0465b9b3bbd91da7f9a523f336]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_skips_ignored_files_and_reports_a_committed_record_once
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: f38abb556cee50e587a4c2446a91c7f2fa140997
content: 22f079e0c52edd4b8e261a363b63b124fc70d687f3ee31cf0865838d130d7502
environment: local-macos-arm64
fingerprint: sha256:e08f27364046e845374ac3ff8d74377f419b86e87105df256e09ff1d95530d55
criteria: [DRIVE-17]
requirements: [REQ-history-016]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T15:00:14Z
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
**Ran cargo test --locked --test doctor -- --exact doctor_skips_ignored_files_and_reports_a_committed_record_once for TC-history-doctor-skips-ignored-and-reports-once: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision f38abb556cee, observed 2026-11-20T15:00:14Z. The raw log stays in the machine-local store.
