---
kind: evidence
id: 2026-11-13-tc-records-migrate-preview-names-what-doctor-reports
aliases: []
claim: "Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-preview-names-what-doctor-reports]
case_digest: [6c9a069cdb9cbfb5a53789f87c9a5b593cc427539e89a3364028853ea22c7af5]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate
  - "--"
  - "--exact"
  - the_preview_names_the_sessions_doctor_reports_after_the_apply
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: bc6d4479ffdc60d4b84891542c8949325f8886a7
content: bc32e0a8290da75323b46576c288e2da274d9e5f14fd04175a01dffb2ab183ce
environment: local-macos-arm64
fingerprint: sha256:d4e9fff1c249051c0f861738a51f5199058899e10457ea025e0fee0af906535c
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T07:45:52Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate -- --exact the_preview_names_the_sessions_doctor_reports_after_the_apply for TC-records-migrate-preview-names-what-doctor-reports: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision bc6d4479ffdc, observed 2026-11-13T07:45:52Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- the_preview_names_the_sessions_doctor_reports_after_the_apply stdout ----
- thread 'the_preview_names_the_sessions_doctor_reports_after_the_apply' (978175865) panicked at tests/migrate.rs:1794:13:
- the preview's follow-ups do not name finished_session 2026-11-13-finished:
- Declare Chough's merge drivers so generated files and bugs merge without a hand-resolved conflict (REQ-cli-047): add `.chough/bugs/*.md merge=chough-bug`, `knowledge/index.md merge=chough-generated`, `knowledge/journal/feed.xml merge=chough-generated` to `.gitattributes` and commit it, then run `chough doctor --fix` in each clone, which sets `merge.chough-bug.driver` and `merge.chough-generated.driver`.
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 26 filtered out; finished in 1.98s
