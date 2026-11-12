---
kind: bug
id: 2026-11-12-chough-correct-leaves-a-blank-line-inside-the-timeline-list
aliases: [BUG-chough-correct-leaves-a-blank-line-inside-the-timeline-list-fee2f2bc5eaf173fbae24d7b]
title: chough correct leaves a blank line inside the Timeline list
status: verified
classification: product-defect
scope: in-scope
impact: The Timeline renders as a loose list; every byte is kept
criteria: []
requirements: [REQ-history-010]
cases: []
evidence:
  - OBS-tc-history-a-correction-keeps-the-timeline-one-list-93956c5a288289b622070479
  - OBS-tc-history-a-correction-keeps-the-timeline-one-list-41ba160dcb5dd77c5fb2e1ab
receipts:
  - id: 2026-11-12-tc-history-a-correction-keeps-the-timeline-one-list
    aliases: OBS-tc-history-a-correction-keeps-the-timeline-one-list-93956c5a288289b622070479
    claim: "Ran cargo test --locked --test correct -- --exact a_correction_joins_the_timeline_list_with_no_blank_line for TC-history-a-correction-keeps-the-timeline-one-list: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test correct -- --exact a_correction_joins_the_timeline_list_with_no_blank_line
    revision: 5f167167fd07873c887741586ccf3f6e54f5c98a
    environment: local-macos-arm64
    content: 13091614c5d3c3e95b98e6e3e65d735513f1eab60e96102761dd290cf20b94fe
    fingerprint: sha256:8f524d90dc903c5c814955b747041c9f122ef4e3c6d075c0c9cea0a205ed650c
    case: TC-history-a-correction-keeps-the-timeline-one-list
    case_digest: e8b779d862ab9baaeed4143ef5be8af4ee809b0453876b18cbad36cea1994f83
    observed: 2026-11-12T01:59:32Z
    duration_seconds: 0
  - id: 2026-11-12-tc-history-a-correction-keeps-the-timeline-one-list-f6aa2f
    aliases: OBS-tc-history-a-correction-keeps-the-timeline-one-list-41ba160dcb5dd77c5fb2e1ab
    claim: "Ran cargo test --locked --test correct -- --exact a_correction_joins_the_timeline_list_with_no_blank_line for TC-history-a-correction-keeps-the-timeline-one-list: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test correct -- --exact a_correction_joins_the_timeline_list_with_no_blank_line
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:ddf5903c1f1e6b5f610ff3677e4327c526be5456d1521b77cb06a7d2c74753b7
    case: TC-history-a-correction-keeps-the-timeline-one-list
    case_digest: e8b779d862ab9baaeed4143ef5be8af4ee809b0453876b18cbad36cea1994f83
    observed: 2026-11-12T02:17:26Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 18025ad47c730af7589317f76608634b2487896f
found_by: t37j, test stage, areas sessions and bugs
related: []
created: 2026-11-12T01:41:12Z
updated: 2026-11-12T02:18:07Z
---
# chough correct leaves a blank line inside the Timeline list

**What happened:** A correction is appended after the file's trailing blank line, so the Timeline becomes a loose Markdown list for good
Actual: A blank line separates the correction from the entries before it

**Steps:** chough correct on a session whose Timeline ends the file

**Expected:** The correction joins the Timeline list as ordinary saves do (REQ-history-010)

**Why it matters:** The Timeline renders as a loose list; every byte is kept

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:41:12Z · Reported (t37j, test stage, areas sessions and bugs)
- 2026-11-12T02:01:23Z · status: open -> investigating. Red at 5f167167 (leif/0.6.0-test1-red: d854ce1a plus the regression tests 558386f3): TC-history-a-correction-keeps-the-timeline-one-list failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-history-a-correction-keeps-the-timeline-one-list-93956c5a288289b622070479
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-history-a-correction-keeps-the-timeline-one-list passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-history-a-correction-keeps-the-timeline-one-list-41ba160dcb5dd77c5fb2e1ab
