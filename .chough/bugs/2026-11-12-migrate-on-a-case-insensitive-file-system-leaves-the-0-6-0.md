---
kind: bug
id: 2026-11-12-migrate-on-a-case-insensitive-file-system-leaves-the-0-6-0
aliases:
  - BUG-migrate-on-a-case-insensitive-file-system-leaves-the-0-6-0-index-commit-c61efcb1dcd7e7bf7564f11f
title: Migrate on a case-insensitive file system leaves the 0.6.0 index committed under the old name knowledge/INDEX.md
status: verified
classification: product-defect
scope: blocking
impact: Every macOS upgrade from 0.5.x commits a tree whose index 0.6.0 cannot find on Linux, and the receipt names a file the commit lacks
criteria: []
requirements: [REQ-records-009]
cases: []
evidence:
  - OBS-tc-records-migrate-case-only-rename-reaches-git-c70fdbb98739586330d855ca
  - OBS-tc-records-migrate-case-only-rename-reaches-git-be9c06750fb6ea792086720d
receipts:
  - id: 2026-11-12-tc-records-migrate-case-only-rename-reaches-git
    aliases: OBS-tc-records-migrate-case-only-rename-reaches-git-c70fdbb98739586330d855ca
    claim: "Ran cargo test --locked --test migrate_knowledge -- --exact a_case_only_rename_reaches_git_under_the_new_name for TC-records-migrate-case-only-rename-reaches-git: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate_knowledge -- --exact a_case_only_rename_reaches_git_under_the_new_name
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:3e6f9e4a608078ae18301bde76f7ab327849bb99094e517dfcbd8736bc0336bc
    case: TC-records-migrate-case-only-rename-reaches-git
    case_digest: 076b02dc2283f39bd7bab8d096eb4e82b852f6a2dad3506b0407dafc57e112f1
    observed: 2026-11-12T01:59:05Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-migrate-case-only-rename-reaches-git-e2a118
    aliases: OBS-tc-records-migrate-case-only-rename-reaches-git-be9c06750fb6ea792086720d
    claim: "Ran cargo test --locked --test migrate_knowledge -- --exact a_case_only_rename_reaches_git_under_the_new_name for TC-records-migrate-case-only-rename-reaches-git: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate_knowledge -- --exact a_case_only_rename_reaches_git_under_the_new_name
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:3331680dbb6b7e2816e0f79cf71d6be93a3e62e789725a3697618f8734fd2473
    case: TC-records-migrate-case-only-rename-reaches-git
    case_digest: 076b02dc2283f39bd7bab8d096eb4e82b852f6a2dad3506b0407dafc57e112f1
    observed: 2026-11-12T02:16:56Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t384, test stage, area upgrade
related: []
created: 2026-11-12T01:38:28Z
updated: 2026-11-12T02:18:06Z
---
# Migrate on a case-insensitive file system leaves the 0.6.0 index committed under the old name knowledge/INDEX.md

**What happened:** After chough migrate --apply on macOS, git status shows the index as modified under knowledge/INDEX.md; after git add -A and commit, HEAD holds the new bytes as knowledge/INDEX.md and no knowledge/index.md
Actual: Git keeps its index entry under the old case; a case-sensitive clone and Linux CI see only knowledge/INDEX.md

**Steps:** Migrate a committed 0.5.0 project with knowledge/INDEX.md on a case-insensitive file system (git core.ignorecase=true), then git add -A and commit

**Expected:** The commit removes knowledge/INDEX.md and adds knowledge/index.md, so the receipt's after digests match the files at the new commit (REQ-records-009)

**Why it matters:** Every macOS upgrade from 0.5.x commits a tree whose index 0.6.0 cannot find on Linux, and the receipt names a file the commit lacks

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:28Z · Reported (t384, test stage, area upgrade)
- 2026-11-12T02:01:36Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-migrate-case-only-rename-reaches-git failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-migrate-case-only-rename-reaches-git-c70fdbb98739586330d855ca
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-migrate-case-only-rename-reaches-git passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-migrate-case-only-rename-reaches-git-be9c06750fb6ea792086720d
