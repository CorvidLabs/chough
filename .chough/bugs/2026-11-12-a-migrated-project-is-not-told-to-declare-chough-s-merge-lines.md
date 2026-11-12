---
kind: bug
id: 2026-11-12-a-migrated-project-is-not-told-to-declare-chough-s-merge-lines
aliases:
  - BUG-a-migrated-project-is-not-told-to-declare-chough-s-merge-lines-and-driv-1973798d88df25174118eaf3
title: A migrated project is not told to declare Chough's merge lines and drivers, so doctor reports merge_drivers after every migration
status: verified
classification: product-defect
scope: in-scope
impact: A migrated project merges its generated files with Git's text merge until someone runs doctor
criteria: []
requirements: [REQ-records-009, REQ-cli-047]
cases: []
evidence:
  - OBS-tc-records-the-migration-names-the-merge-setup-0d1117bad2d983c964b9eef4
  - OBS-tc-records-the-migration-names-the-merge-setup-6775163a23ce48da839d02ce
receipts:
  - id: 2026-11-12-tc-records-the-migration-names-the-merge-setup
    aliases: OBS-tc-records-the-migration-names-the-merge-setup-0d1117bad2d983c964b9eef4
    claim: "Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_the_merge_lines_and_drivers for TC-records-the-migration-names-the-merge-setup: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_the_merge_lines_and_drivers
    revision: f59f10fe6a8af7b596e9d39b5852319e687f1a6b
    environment: local-macos-arm64
    content: ab1326202bcc3ce4d6298061ee4546dd057608a66bc0e713f6452ea4fa68e1bb
    fingerprint: sha256:28712caca9cf03a4efaf7bb5bcf6c7e86c952eb1e55477fdf84e9a104b513052
    case: TC-records-the-migration-names-the-merge-setup
    case_digest: e6e303d6c78853044af68cc8ff597e2177783b4e4b4f7414eb9bbaf7b71d86c3
    observed: 2026-11-12T07:04:03Z
    duration_seconds: 2
  - id: 2026-11-12-tc-records-the-migration-names-the-merge-setup-f89094
    aliases: OBS-tc-records-the-migration-names-the-merge-setup-6775163a23ce48da839d02ce
    claim: "Ran cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_the_merge_lines_and_drivers for TC-records-the-migration-names-the-merge-setup: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact the_migration_follow_ups_name_the_merge_lines_and_drivers
    revision: a2591337d3ef483220fa572492173a6e234e088b
    environment: local-macos-arm64
    content: 32e0fc48c5a494e26afda58bf670d662de61d431cde15afa62b8f41b37df7d7e
    fingerprint: sha256:ba6d8c8cc2f02836d829477391a7c2e1260bdda650c91ce9a316b00df2177390
    case: TC-records-the-migration-names-the-merge-setup
    case_digest: e6e303d6c78853044af68cc8ff597e2177783b4e4b4f7414eb9bbaf7b71d86c3
    observed: 2026-11-12T07:04:29Z
    duration_seconds: 3
environment: local-macos-arm64, tester build at 596ced67, debug, SpecSync 6.3.0
revision: 6360516bdac4f5480902097986c2218b411356f5
found_by: t384, 6.3.0 re-run, area upgrade (N1)
related: []
created: 2026-11-12T07:02:05Z
updated: 2026-11-12T07:04:39Z
---
# A migrated project is not told to declare Chough's merge lines and drivers, so doctor reports merge_drivers after every migration

**What happened:** After chough migrate --apply, doctor reports merge_drivers for the three .gitattributes lines and both drivers, while the plan's and receipt's follow-ups name only the verification exclusions
Actual: Only doctor says it

**Steps:** Migrate any 0.5.x project, then chough doctor

**Expected:** The migration's follow-ups say how to declare the merge lines and set the drivers (REQ-cli-047), as they already do for the verification exclusions

**Why it matters:** A migrated project merges its generated files with Git's text merge until someone runs doctor

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T07:02:05Z · Reported (t384, 6.3.0 re-run, area upgrade (N1))
- 2026-11-12T07:04:39Z · status: open -> investigating. Red at f59f10fe (before its fix, SpecSync 6.3.0 on PATH): the plan's follow-ups named no merge line; fix in a2591337.; evidence: OBS-tc-records-the-migration-names-the-merge-setup-0d1117bad2d983c964b9eef4
- 2026-11-12T07:04:39Z · status: investigating -> verified. Green at a2591337: TC-records-the-migration-names-the-merge-setup passed; migrate, migrate_knowledge, migrate_bugs, migrate_evidence and docs tests pass.; evidence: OBS-tc-records-the-migration-names-the-merge-setup-6775163a23ce48da839d02ce
