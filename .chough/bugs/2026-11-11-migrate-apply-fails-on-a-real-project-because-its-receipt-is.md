---
kind: bug
id: 2026-11-11-migrate-apply-fails-on-a-real-project-because-its-receipt-is
aliases:
  - BUG-migrate-apply-fails-on-a-real-project-because-its-receipt-is-written-un-cf8bb6d00a0e67dcd85a9e98
title: migrate --apply fails on a real project because its receipt is written under the ordinary 64 KiB record bound
status: verified
classification: product-defect
scope: in-scope
impact: No real project can migrate; a refused apply leaves the 0.6.0 files beside every 0.5.x file until it is resumed
criteria: [DRIVE-27]
requirements: [REQ-records-009, REQ-records-001]
cases: [TC-records-migrate-receipt-bound]
evidence:
  - OBS-tc-records-migrate-receipt-bound-3b9e75047dc1913f7041591e
  - OBS-tc-records-migrate-receipt-bound-5bcd8311e4e7a9153331bc63
receipts:
  - id: 2026-11-11-tc-records-migrate-receipt-bound
    aliases: OBS-tc-records-migrate-receipt-bound-3b9e75047dc1913f7041591e
    claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_past_the_ordinary_record_bound_applies for TC-records-migrate-receipt-bound: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact a_receipt_past_the_ordinary_record_bound_applies
    revision: b405da6ec7d62809682cb1a8ae9b0a29a58aecfa
    environment: local-macos-arm64
    content: ff198fc6be467e9303abf3f3f79a17c4da2b57be5b64d179e8809a068ea6350a
    fingerprint: sha256:6955262c3737ddfe3c9374d13f78a56eeb499752d611bcb9955b5b7e1da451cd
    case: TC-records-migrate-receipt-bound
    case_digest: fbca3a057e623eff3f46b9c600087b1c0c57af78f92cf440dd6e162a3adcc3a9
    observed: 2026-11-11T23:22:24Z
    duration_seconds: 13
  - id: 2026-11-11-tc-records-migrate-receipt-bound-3a5d2b
    aliases: OBS-tc-records-migrate-receipt-bound-5bcd8311e4e7a9153331bc63
    claim: "Ran cargo test --locked --test migrate -- --exact a_receipt_past_the_ordinary_record_bound_applies for TC-records-migrate-receipt-bound: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact a_receipt_past_the_ordinary_record_bound_applies
    revision: 28093a9ab056c3fa78cc3aedcc8fd2d7d8879fa2
    environment: local-macos-arm64
    content: 9218d78334a5ddd94e2d5c42179486152591ab8c270169b953c2fbefd326b2df
    fingerprint: sha256:97bc28bcf045ea885fd71baa1482fb51826a0a7870193f8f220712720b009fd9
    case: TC-records-migrate-receipt-bound
    case_digest: fbca3a057e623eff3f46b9c600087b1c0c57af78f92cf440dd6e162a3adcc3a9
    observed: 2026-11-11T23:23:27Z
    duration_seconds: 11
environment: local-macos-arm64
revision: 9b9152bf9d7595cd104b06e1ab11fdefdefd2819
found_by: t37j's second migration rehearsal (plan Migration step 3) at d57d3aea in a throwaway clone
related: []
created: 2026-11-11T23:21:48Z
updated: 2026-11-11T23:23:27Z
---
# migrate --apply fails on a real project because its receipt is written under the ordinary 64 KiB record bound

**What happened:** Applying the migration of this repository fails with 'managed file exceeds 65536 bytes' for the receipt, after the 0.6.0 files were already written, leaving both layouts in the tree
Actual: The receipt is written with the 65,536-byte ordinary record bound and refuses only after the planned files were written

**Steps:** Preview and apply chough migrate on a project with a few hundred 0.5.x files, such as this repository in a throwaway clone
1. Commit a 0.5.x project with about 400 removable files
2. Run chough migrate, then chough migrate --apply <digest> --consent <source>

**Expected:** The receipt applies within its own 4,194,304-byte bound (REQ-records-001), and a receipt over that bound refuses before anything is written

**Why it matters:** No real project can migrate; a refused apply leaves the 0.6.0 files beside every 0.5.x file until it is resumed

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T23:21:48Z · Reported (t37j's second migration rehearsal (plan Migration step 3) at d57d3aea in a throwaway clone)
- 2026-11-11T23:22:36Z · Red on the current apply: TC-records-migrate-receipt-bound fails with 'managed file exceeds 65536 bytes' for the receipt; revision b405da6e on leif/0.6.0-receipt-bound-red.; evidence: OBS-tc-records-migrate-receipt-bound-3b9e75047dc1913f7041591e
- 2026-11-11T23:23:27Z · status: open -> verified. Green on the fix: TC-records-migrate-receipt-bound passes; a 400-file project's receipt applies within the receipt bound.; evidence: OBS-tc-records-migrate-receipt-bound-5bcd8311e4e7a9153331bc63
