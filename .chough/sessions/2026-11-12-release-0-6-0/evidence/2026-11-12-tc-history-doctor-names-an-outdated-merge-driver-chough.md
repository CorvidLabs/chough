---
kind: evidence
id: 2026-11-12-tc-history-doctor-names-an-outdated-merge-driver-chough
aliases: []
claim: "Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-history-doctor-names-an-outdated-merge-driver-chough]
case_digest: [24ca88bc8d4fe7aac81f8e9663dee8ab55fa86abc92010a89fd999a0aab9e948]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - doctor
  - "--"
  - "--exact"
  - doctor_reports_merge_drivers_that_would_run_an_older_chough
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 804bdb506371f0f50b97e76f0c09cc33daf439af
content: 431b305478c9131e54c07bc6bd895e4d5380b9af25bb28dad9e6056aed6a4f09
environment: local-macos-arm64
fingerprint: sha256:532f0975644e5a7181f1e516a33ed85d7fafc45138be5d49d1f531c823729efe
criteria: [DRIVE-17]
requirements: [REQ-history-012]
contracts: [specs/history/history.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T21:05:51Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test doctor -- --exact doctor_reports_merge_drivers_that_would_run_an_older_chough for TC-history-doctor-names-an-outdated-merge-driver-chough: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 804bdb506371, observed 2026-11-12T21:05:51Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- doctor_reports_merge_drivers_that_would_run_an_older_chough stdout ----
- thread 'doctor_reports_merge_drivers_that_would_run_an_older_chough' (924235093) panicked at tests/doctor.rs:982:5:
- assertion `left == right` failed: {
-   "findings": [],
-   "mode": "report",
-   "schema_version": 1
- }
-   left: 0
-  right: 1
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 1.12s
