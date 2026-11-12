---
kind: bug
id: 2026-11-12-the-standing-migration-preference-cannot-be-set-by-any-chough
aliases:
  - BUG-the-standing-migration-preference-cannot-be-set-by-any-chough-command-b-a114304b2d48780c3b644360
title: The standing migration preference cannot be set by any Chough command before the first migration
status: verified
classification: requirement-gap
scope: in-scope
impact: The standing route is documented but unreachable without a hand edit
criteria: []
requirements: [REQ-records-009, REQ-records-010]
cases: []
evidence:
  - OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-3941ab5e1920731f8bc23a8a
  - OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-e1012340b1f60207dcc997c6
receipts:
  - id: 2026-11-12-tc-records-the-standing-migration-preference-is-set-before
    aliases: OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-3941ab5e1920731f8bc23a8a
    claim: "Ran cargo test --locked --test migrate -- --exact the_standing_migration_preference_is_set_before_the_first_migration for TC-records-the-standing-migration-preference-is-set-before-migrating: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact the_standing_migration_preference_is_set_before_the_first_migration
    revision: 591f61b54966363d15931430e8d36bb2afc06d18
    environment: local-macos-arm64
    content: a9936647a19e2eb689e2e47509b638bbc842945e0cd53f15dbad4d7d5d081bb0
    fingerprint: sha256:00dfadfca5e76c7b4cadcb2bb3428736884806054f5b6e2dee9ab54d2bd2d667
    case: TC-records-the-standing-migration-preference-is-set-before-migrating
    case_digest: 4cdf403eb9010cbc16f344ee9c6f57e7d7685f97d67141d45ce0b500ea3ef104
    observed: 2026-11-12T06:56:52Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-the-standing-migration-preference-is-set-before-4a6da2
    aliases: OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-e1012340b1f60207dcc997c6
    claim: "Ran cargo test --locked --test migrate -- --exact the_standing_migration_preference_is_set_before_the_first_migration for TC-records-the-standing-migration-preference-is-set-before-migrating: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact the_standing_migration_preference_is_set_before_the_first_migration
    revision: 8091890cb874cf2bf33f8d5e3a6467d3bbee0dbb
    environment: local-macos-arm64
    content: 0d709ba899b8dfcf2dc898f29ca4dbb97df391dcac2ed6abe29f19ea553c235d
    fingerprint: sha256:fe2f3c84dc599a95fd5e85dbed1417cac6707f3fa77ae2c2cc9b061964820af2
    case: TC-records-the-standing-migration-preference-is-set-before-migrating
    case_digest: 4cdf403eb9010cbc16f344ee9c6f57e7d7685f97d67141d45ce0b500ea3ef104
    observed: 2026-11-12T06:59:08Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 81d2bf8c3bc2a1705e67113cb95c14ce0a3b4c59
found_by: t384, test stage, area upgrade
related: []
created: 2026-11-12T01:45:37Z
updated: 2026-11-12T06:59:41Z
---
# The standing migration preference cannot be set by any Chough command before the first migration

**What happened:** On a committed 0.5.0 project, chough pref set migration standing is refused as refused:unmigrated by 0.6.0 and unknown to 0.5.0; only a hand edit of .chough/preferences.json works
Actual: REQ-records-010 refuses every command but migrate, doctor, skill, --help and --version on an unmigrated tree

**Steps:** Follow docs/cli.md's standing-preference route to migrate a 0.5.x tree

**Expected:** The documented route works before the first migration (REQ-records-009 offers it as an alternative to --consent)

**Why it matters:** The standing route is documented but unreachable without a hand edit

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:45:37Z · Reported (t384, test stage, area upgrade)
- 2026-11-12T06:57:33Z · status: open -> investigating. Red at 591f61b5 (TC-records-the-standing-migration-preference-is-set-before-migrating, before its fix, SpecSync 6.3.0 on PATH): failed for this bug's reason; fix in d6d7eeba.; evidence: OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-3941ab5e1920731f8bc23a8a
- 2026-11-12T06:59:41Z · status: investigating -> verified. Green at 8091890c: TC-records-the-standing-migration-preference-is-set-before-migrating passed; full suite 638 passed, clippy and fmt clean on SpecSync 6.3.0 at 596ced67; t38b's black-box retest passed.; evidence: OBS-tc-records-the-standing-migration-preference-is-set-before-migrating-e1012340b1f60207dcc997c6
