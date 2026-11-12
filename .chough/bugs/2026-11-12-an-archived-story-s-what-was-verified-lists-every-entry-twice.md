---
kind: bug
id: 2026-11-12-an-archived-story-s-what-was-verified-lists-every-entry-twice
aliases:
  - BUG-an-archived-story-s-what-was-verified-lists-every-entry-twice-once-with-856e99557be80488426cfcd9
title: An archived story's What was verified lists every entry twice, once with its receipt and once as a bare ID
status: verified
classification: product-defect
scope: in-scope
impact: Every story doubles its verified list
criteria: []
requirements: [REQ-history-006, REQ-records-004]
cases: []
evidence:
  - OBS-tc-history-a-story-lists-each-entry-once-5de8b596051a6dd542630865
  - OBS-tc-history-a-story-lists-each-entry-once-2071c95dbccd036ea394e9ed
receipts:
  - id: 2026-11-12-tc-history-a-story-lists-each-entry-once
    aliases: OBS-tc-history-a-story-lists-each-entry-once-5de8b596051a6dd542630865
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-a-story-lists-each-entry-once: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:459b70202634297afec8e73181c63da58237c580749f1d909ddf72b7482c878d
    case: TC-history-a-story-lists-each-entry-once
    case_digest: f6338bdefd34eed566edafeef8dc084ac5daa95a4c180547cba0577ee56ee4d7
    observed: 2026-11-12T01:59:28Z
    duration_seconds: 1
  - id: 2026-11-12-tc-history-a-story-lists-each-entry-once-589cf8
    aliases: OBS-tc-history-a-story-lists-each-entry-once-2071c95dbccd036ea394e9ed
    claim: "Ran cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt for TC-history-a-story-lists-each-entry-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact a_story_lists_each_entry_once_with_its_receipt
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:98babd66582cb6022a56871fa0e9db7e91cd86319a281a28e939203dc2e768cf
    case: TC-history-a-story-lists-each-entry-once
    case_digest: f6338bdefd34eed566edafeef8dc084ac5daa95a4c180547cba0577ee56ee4d7
    observed: 2026-11-12T02:17:16Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 81d2bf8c3bc2a1705e67113cb95c14ce0a3b4c59
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:49:25Z
updated: 2026-11-12T02:18:07Z
---
# An archived story's What was verified lists every entry twice, once with its receipt and once as a bare ID

**What happened:** After session close or done, the story holds each entry's receipt line and then the session's own link line turned into a bare entry ID
Actual: The session's [claim](evidence/<id>.md) lines survive as bare IDs after the receipt lines

**Steps:** Close or finish a session with evidence entries

**Expected:** One What was verified line per entry, with its receipt (REQ-history-006, REQ-records-004)

**Why it matters:** Every story doubles its verified list

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:49:25Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-history-a-story-lists-each-entry-once failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-history-a-story-lists-each-entry-once-5de8b596051a6dd542630865
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-history-a-story-lists-each-entry-once passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-history-a-story-lists-each-entry-once-2071c95dbccd036ea394e9ed
