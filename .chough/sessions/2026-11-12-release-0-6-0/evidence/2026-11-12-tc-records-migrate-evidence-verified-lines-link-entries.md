---
kind: evidence
id: 2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-evidence-verified-lines-link-entries]
case_digest: [315413e3ec932f7b304b5bd66a15e95c81885c0378e9970f506ddb0617779436]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - a_migrated_sessions_verified_lines_link_the_carried_entries
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:552b80fa061766ba71a6a42a37800502f1d1f8f1ff5a6b22db892ca00085b182
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_sessions_verified_lines_link_the_carried_entries for TC-records-migrate-evidence-verified-lines-link-entries: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:36Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_migrated_sessions_verified_lines_link_the_carried_entries stdout ----
- thread 'a_migrated_sessions_verified_lines_link_the_carried_entries' (927441096) panicked at tests/migrate_evidence.rs:1253:13:
- assertion `left == right` failed: after the apply: OBS-tc-fixture-0001-b83687e52f4a30ec930beeda [
-     "- OBS-tc-fixture-0001-b83687e52f4a30ec930beeda",
-     "- OBS-product-test-6853752a8d9a557c1f20b8d8",
- ]
-   left: 0
-  right: 1
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 1.00s
