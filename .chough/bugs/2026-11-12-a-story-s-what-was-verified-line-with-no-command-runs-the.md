---
kind: bug
id: 2026-11-12-a-story-s-what-was-verified-line-with-no-command-runs-the
aliases:
  - BUG-a-story-s-what-was-verified-line-with-no-command-runs-the-fingerprint-d-9f57960e6480ce3b46f00d51
title: A story's What was verified line with no command runs the fingerprint digits on from the claim
status: verified
classification: product-defect
scope: in-scope
impact: Hard to read
criteria: []
requirements: [REQ-records-004]
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
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs
related: []
created: 2026-11-12T01:41:12Z
updated: 2026-11-12T02:18:07Z
---
# A story's What was verified line with no command runs the fingerprint digits on from the claim

**What happened:** The line reads '<claim> ef9e4cbbc26a [screenshot](...)' with no separator
Actual: The ' — ' separator is dropped with the command

**Steps:** An evidence entry with no command, such as a human observation with a screenshot, when the story is written

**Expected:** The claim and the receipt are separated as when a command is shown (REQ-records-004)

**Why it matters:** Hard to read

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:12Z · Reported (t37j, test stage, areas sessions and bugs)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-history-a-story-lists-each-entry-once failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-history-a-story-lists-each-entry-once-5de8b596051a6dd542630865
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-history-a-story-lists-each-entry-once passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-history-a-story-lists-each-entry-once-2071c95dbccd036ea394e9ed
