---
kind: bug
id: 2026-11-12-a-migrated-observation-with-an-uppercase-content-digest-makes
aliases:
  - BUG-a-migrated-observation-with-an-uppercase-content-digest-makes-status-an-c39258307ec6ded0e93a3d02
title: A migrated observation with an uppercase content digest makes status and next refuse
status: verified
classification: product-defect
scope: in-scope
impact: A migrated tree is unusable until someone edits the entry by hand
criteria: [DRIVE-27]
requirements: [REQ-records-009, REQ-cli-017]
cases: [TC-records-migrate-evidence-uppercase-digests]
evidence:
  - OBS-tc-records-migrate-evidence-uppercase-digests-9ce28d5e695eb59d47b6b354
  - OBS-tc-records-migrate-evidence-uppercase-digests-662277bf7ccc63e2a6cb7688
receipts:
  - id: 2026-11-12-tc-records-migrate-evidence-uppercase-digests
    aliases: OBS-tc-records-migrate-evidence-uppercase-digests-9ce28d5e695eb59d47b6b354
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact uppercase_digests_migrate_lowercased_and_read for TC-records-migrate-evidence-uppercase-digests: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate_evidence -- --exact uppercase_digests_migrate_lowercased_and_read
    revision: 8f019ca77db58f9cb11639983a85fff66922d6a6
    environment: local-macos-arm64
    content: b3b56ee8b6078d0d2187296f0e942c0219beb52e27563f22139ccd499f1c1962
    fingerprint: sha256:ee777e0749681a4c59155abf9d598f7fd45f7b2c09d33b995e19ccc687528c89
    case: TC-records-migrate-evidence-uppercase-digests
    case_digest: b2ee74c0edc053f5ca7ed0579182163f32fc00f519c6e3229170fc7edee837df
    observed: 2026-11-12T00:10:26Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-migrate-evidence-uppercase-digests-95a2c2
    aliases: OBS-tc-records-migrate-evidence-uppercase-digests-662277bf7ccc63e2a6cb7688
    claim: "Ran cargo test --locked --test migrate_evidence -- --exact uppercase_digests_migrate_lowercased_and_read for TC-records-migrate-evidence-uppercase-digests: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_evidence -- --exact uppercase_digests_migrate_lowercased_and_read
    revision: 5e60320848ed1b4c686fd81a492ef3030311cddf
    environment: local-macos-arm64
    content: dfb16a6ddf0f6adcf3ade9bdf62edd5015a77c550fe5d937bbc07f72300a7f12
    fingerprint: sha256:fb161509819bf6649d3a73fbd6f76c6b9497823f804edcf49079979cfd42f798
    case: TC-records-migrate-evidence-uppercase-digests
    case_digest: b2ee74c0edc053f5ca7ed0579182163f32fc00f519c6e3229170fc7edee837df
    observed: 2026-11-12T00:10:59Z
    duration_seconds: 1
environment: local-macos-arm64
revision: bf54e24ff3d5ed73a4457bd351b42d56e6ccd518
found_by: t38b's review (R4) of t37j's content check 9a914511; red at 8f019ca7 (leif/0.6.0-uppercase-digests-red)
related: []
created: 2026-11-12T00:10:39Z
updated: 2026-11-12T00:10:59Z
---
# A migrated observation with an uppercase content digest makes status and next refuse

**What happened:** After migrating a 0.5.x observation whose snapshot content digest is uppercase, chough status and chough next exit 1: content must be a SHA-256 digest (64 lowercase hexadecimal digits)
Actual: The digest is carried as written, and 0.6.0's reader refuses the entry

**Steps:** Migrate a 0.5.x project with an observation whose snapshot.content is uppercase hex and that an open session references, then run chough status
1. Migrate a fixture whose observation has an uppercase content digest
2. Run chough status --environment fixture

**Expected:** The migration carries every 0.5.x digest lowercased (the same SHA-256), so the migrated tree reads (REQ-records-009, REQ-cli-017)

**Why it matters:** A migrated tree is unusable until someone edits the entry by hand

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T00:10:39Z · Reported (t38b's review (R4) of t37j's content check 9a914511; red at 8f019ca7 (leif/0.6.0-uppercase-digests-red)); evidence: OBS-tc-records-migrate-evidence-uppercase-digests-9ce28d5e695eb59d47b6b354
- 2026-11-12T00:10:59Z · status: open -> verified. Green after t38b's 6bf8e8c7: every carried digest is lowercased and status and next read the migrated entry.; evidence: OBS-tc-records-migrate-evidence-uppercase-digests-662277bf7ccc63e2a6cb7688
