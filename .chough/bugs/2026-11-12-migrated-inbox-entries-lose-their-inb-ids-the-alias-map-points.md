---
kind: bug
id: 2026-11-12-migrated-inbox-entries-lose-their-inb-ids-the-alias-map-points
aliases:
  - BUG-migrated-inbox-entries-lose-their-inb-ids-the-alias-map-points-at-the-r-9b4c38af5254610925b5bd88
title: "Migrated inbox entries lose their INB- IDs: the alias map points at the removed 0.5.x file and the new entry has no aliases"
status: verified
classification: product-defect
scope: in-scope
impact: A wrong receipt map and a lost alias; no 0.6.0 command takes an inbox ID yet
criteria: []
requirements: [REQ-records-009, REQ-records-003]
cases: []
evidence:
  - OBS-tc-records-migrated-inbox-entries-keep-their-ids-4c5788ccafd695344dc82467
  - OBS-tc-records-migrated-inbox-entries-keep-their-ids-34fce22b12fe1a1315cdf071
receipts:
  - id: 2026-11-12-tc-records-migrated-inbox-entries-keep-their-ids
    aliases: OBS-tc-records-migrated-inbox-entries-keep-their-ids-4c5788ccafd695344dc82467
    claim: "Ran cargo test --locked --test migrate -- --exact migrated_inbox_entries_keep_their_ids_and_the_map_names_the_new_files for TC-records-migrated-inbox-entries-keep-their-ids: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test migrate -- --exact migrated_inbox_entries_keep_their_ids_and_the_map_names_the_new_files
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:8642da2342e9e18df86f5e6c4effcd7226ed1047d615ecd74f8024f927d67b80
    case: TC-records-migrated-inbox-entries-keep-their-ids
    case_digest: 15f80fb4583f86e98ae7b3f2bda05c79aa4f8bdffb89948073acdeb8e47a5e52
    observed: 2026-11-12T01:59:07Z
    duration_seconds: 2
  - id: 2026-11-12-tc-records-migrated-inbox-entries-keep-their-ids-56b4cf
    aliases: OBS-tc-records-migrated-inbox-entries-keep-their-ids-34fce22b12fe1a1315cdf071
    claim: "Ran cargo test --locked --test migrate -- --exact migrated_inbox_entries_keep_their_ids_and_the_map_names_the_new_files for TC-records-migrated-inbox-entries-keep-their-ids: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test migrate -- --exact migrated_inbox_entries_keep_their_ids_and_the_map_names_the_new_files
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:4dba2a4c259480d8d85e0094a6c7d745d3cc89d9dfc68bff595aba037f4ba8e0
    case: TC-records-migrated-inbox-entries-keep-their-ids
    case_digest: 15f80fb4583f86e98ae7b3f2bda05c79aa4f8bdffb89948073acdeb8e47a5e52
    observed: 2026-11-12T02:16:59Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: fd503b01e3cda963cb368fe514a39b08d2b761aa
found_by: t384, test stage, area upgrade
related: []
created: 2026-11-12T01:49:38Z
updated: 2026-11-12T02:18:06Z
---
# Migrated inbox entries lose their INB- IDs: the alias map points at the removed 0.5.x file and the new entry has no aliases

**What happened:** The plan's aliases and the receipt's ID map send each INB- ID to .chough/sessions/<key>.inbox/<id>.json, which the apply removes; the new agent/inbox entry's front matter has no aliases
Actual: The map names the removed source and the alias is dropped

**Steps:** Migrate a 0.5.0 project with inbox entries on an open session

**Expected:** Every 0.5.x ID is kept as an alias (REQ-records-003) and the receipt maps it to the new file (REQ-records-009)

**Why it matters:** A wrong receipt map and a lost alias; no 0.6.0 command takes an inbox ID yet

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:49:38Z · Reported (t384, test stage, area upgrade)
- 2026-11-12T02:01:22Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-records-migrated-inbox-entries-keep-their-ids failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-migrated-inbox-entries-keep-their-ids-4c5788ccafd695344dc82467
- 2026-11-12T02:18:06Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-migrated-inbox-entries-keep-their-ids passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-migrated-inbox-entries-keep-their-ids-34fce22b12fe1a1315cdf071
