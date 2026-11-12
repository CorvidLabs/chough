---
kind: evidence
id: 2026-11-12-tc-records-migrate-from-0-5
aliases: []
claim: "Ran cargo test --locked --test migrate for TC-records-migrate-from-0-5: passed (exit 0)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-migrate-from-0-5]
case_digest: [c5b17f4681d2e002e53a82f59451189eb9cf32f55b3c5e368f4725676de2cbcb]
command: [cargo, test, --locked, --test, migrate]
exit_code: 0
duration_seconds: 17
revision: 8daaad2327e308118c1a4e4df41b9d210ffbd863
content: ba539ee4282cc272a62844eb927a90969ea14ab58ed53e697face9604520200e
environment: local-macos-arm64
fingerprint: sha256:45a5324507f7d73d0003f89a6f7dbef7bced2a4731395ddec4a181ba5eb74a3b
criteria: [DRIVE-27, LIFE-20, LIFE-21, KNOW-10, KNOW-11]
requirements: [REQ-records-009]
contracts: [specs/records/records.spec.md]
limitations:
  - outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected
  - inherited environment variables were not recorded
observed: 2026-11-12T15:39:49Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test migrate for TC-records-migrate-from-0-5: passed (exit 0).**

Recorded as passed in local-macos-arm64 at revision 8daaad2327e3, observed 2026-11-12T15:39:49Z. The raw log stays in the machine-local store.
