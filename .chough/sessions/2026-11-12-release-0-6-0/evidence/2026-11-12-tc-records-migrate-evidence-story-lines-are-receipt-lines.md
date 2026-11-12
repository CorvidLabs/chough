---
kind: evidence
id: 2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines
aliases: []
claim: "Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-evidence-story-lines-are-receipt-lines]
case_digest: [b8b2042bb991bd9e9975b8d72dfb12c50c4cdb7c9c2ebbb8f0d4d45809a617ff]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - migrate_evidence
  - "--"
  - "--exact"
  - a_migrated_storys_verified_lines_are_receipt_lines
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
content: 073d7330cf258fdd25b33d439e35f9344842f8fdfa2a1cd81c388ab4af69e3ca
environment: local-macos-arm64
fingerprint: sha256:fe60276410f2a35555b23864c077d60474a7e0175037b9ffa84dcabc3f08d3da
criteria: [DRIVE-27, LIFE-20]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T23:04:42Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate_evidence -- --exact a_migrated_storys_verified_lines_are_receipt_lines for TC-records-migrate-evidence-story-lines-are-receipt-lines: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 4e245263fe4e, observed 2026-11-12T23:04:42Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_migrated_storys_verified_lines_are_receipt_lines stdout ----
- thread 'a_migrated_storys_verified_lines_are_receipt_lines' (927441837) panicked at tests/migrate_evidence.rs:1316:5:
- the bare OBS-product-test-00000000000000000000a001 is still listed: [
-     "- OBS-product-test-00000000000000000000a001",
- ]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 1.02s
