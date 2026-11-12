---
kind: bug
id: 2026-11-11-no-migration-carries-a-0-x-project-s-records-into-0-6-0-213
aliases: [BUG-no-migration-carries-a-0-x-project-s-records-into-0-6-0-213-1242c0958cd9749eb05ebd64]
title: No migration carries a 0.x project's records into 0.6.0 (#213)
status: verified
classification: product-defect
scope: in-scope
impact: "Existing projects cannot move to 0.6.0 (GitHub #213)"
criteria: [DRIVE-27, LIFE-21]
requirements: [REQ-cli-007]
cases: [TC-records-migrate-from-0-5]
evidence:
  - OBS-tc-records-migrate-from-0-5-9a6172dd97f16a59589dfcf5
  - OBS-tc-records-migrate-from-0-5-352c53f003aa9671c80affda
receipts:
  - id: 2026-11-11-tc-records-migrate-from-0-5
    aliases: OBS-tc-records-migrate-from-0-5-9a6172dd97f16a59589dfcf5
    claim: "Ran cargo test --locked --test migrate for TC-records-migrate-from-0-5: failed (exit 101)."
    result: failed
    command: cargo test --locked --test migrate
    revision: 370d221d68840f2e263e90b927a200438a7a0173
    environment: local-macos-arm64
    content: 843a083f60a67703f38514e1fcffa147b0e3c500ec307ca879614ca5ee672a4c
    fingerprint: sha256:d468f21e672a8f602aeaf732281592869fc3a5c7437423b5482644e9f0a35db8
    case: TC-records-migrate-from-0-5
    case_digest: c5b17f4681d2e002e53a82f59451189eb9cf32f55b3c5e368f4725676de2cbcb
    observed: 2026-11-11T20:36:08Z
    duration_seconds: 1
  - id: 2026-11-11-tc-records-migrate-from-0-5-ceb0b3
    aliases: OBS-tc-records-migrate-from-0-5-352c53f003aa9671c80affda
    claim: "Ran cargo test --locked --test migrate for TC-records-migrate-from-0-5: passed (exit 0)."
    result: passed
    command: cargo test --locked --test migrate
    revision: 7b4cd3eb827f70c821255c0efb0326f17d7ca23a
    environment: local-macos-arm64
    content: e287cef970a2e70b5d716af6bc20f127df4d7275ee92871b760314f7393345e9
    fingerprint: sha256:a2ad4821130b32599c89dccd08d74f035cf8b1598c6d48d11f6f7b84280a1794
    case: TC-records-migrate-from-0-5
    case_digest: c5b17f4681d2e002e53a82f59451189eb9cf32f55b3c5e368f4725676de2cbcb
    observed: 2026-11-11T20:39:20Z
    duration_seconds: 12
environment: local-macos-arm64
revision: 67d001e55e59fa271166d92c95ea44fac6d4f304
found_by: "GitHub issue #213; red on 0.5.0 at leif/0.6.0-batch-red 370d221d"
related: []
created: 2026-11-11T20:36:57Z
updated: 2026-11-11T20:39:43Z
---
# No migration carries a 0.x project's records into 0.6.0 (#213)

**What happened:** chough migrate only migrates a schema 1 session; a 0.5.x project cannot be carried into the 0.6.0 records
Actual: no 0.6.0 migration plan

**Steps:** chough migrate in a 0.5.x project
1. chough session new alpha --objective <outcome>
2. chough migrate

**Expected:** a preview plan with its digest, then an apply with consent that writes the 0.6.0 tree and a receipt, proven on a fixture from every 0.x release

**Why it matters:** Existing projects cannot move to 0.6.0 (GitHub #213)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T20:36:57Z · Reported (GitHub issue #213; red on 0.5.0 at leif/0.6.0-batch-red 370d221d); evidence: OBS-tc-records-migrate-from-0-5-9a6172dd97f16a59589dfcf5
- 2026-11-11T20:39:43Z · status: open -> verified. Fixed by plan step 10 (PR #238): chough migrate previews, applies with consent and a receipt, proven on a fixture from every 0.x release; the evidence mapper (t37d) joins when step 4 lands. The case passed after its 0.5.0 failure.; evidence: OBS-tc-records-migrate-from-0-5-352c53f003aa9671c80affda
