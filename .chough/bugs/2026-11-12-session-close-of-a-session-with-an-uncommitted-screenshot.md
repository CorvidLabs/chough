---
kind: bug
id: 2026-11-12-session-close-of-a-session-with-an-uncommitted-screenshot
aliases:
  - BUG-session-close-of-a-session-with-an-uncommitted-screenshot-lists-it-twic-1e977743a8201d2052b32201
title: session close of a session with an uncommitted screenshot lists it twice in kept and copies its bytes into the archive
status: verified
classification: product-defect
scope: in-scope
impact: Confusing report; uncommitted bytes reach the archive folder
criteria: []
requirements: [REQ-records-012, REQ-records-013]
cases: []
evidence:
  - OBS-tc-records-close-lists-each-kept-file-once-e7187a0624d4f8d776813d9e
  - OBS-tc-records-close-lists-each-kept-file-once-3e6ed3a0ac082ef1d70c6499
receipts:
  - id: 2026-11-12-tc-records-close-lists-each-kept-file-once
    aliases: OBS-tc-records-close-lists-each-kept-file-once-e7187a0624d4f8d776813d9e
    claim: "Ran cargo test --locked --test sessions -- --exact closing_a_session_lists_each_kept_file_once for TC-records-close-lists-each-kept-file-once: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test sessions -- --exact closing_a_session_lists_each_kept_file_once
    revision: 0ee296aaab0e7d9cc584cc5be2d446701940fcf3
    environment: local-macos-arm64
    content: 2cafae9b427ce78cfa15d45b1fa80e7f728962a43bca0270971aa2af6e280d2b
    fingerprint: sha256:7a55273cb30cc9241893f4f7e032bd206e670b6a5b1b81a32692407701a06a84
    case: TC-records-close-lists-each-kept-file-once
    case_digest: 65b95089a69b4ea718e82f4f360458c667ef868a63263b747786e5ee5064a8d0
    observed: 2026-11-12T02:04:03Z
    duration_seconds: 1
  - id: 2026-11-12-tc-records-close-lists-each-kept-file-once-617816
    aliases: OBS-tc-records-close-lists-each-kept-file-once-3e6ed3a0ac082ef1d70c6499
    claim: "Ran cargo test --locked --test sessions -- --exact closing_a_session_lists_each_kept_file_once for TC-records-close-lists-each-kept-file-once: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test sessions -- --exact closing_a_session_lists_each_kept_file_once
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:15b49ec554e1f8f652197b0d3d0bcf14a83c9579d9682331575d2bc38e6a1e1f
    case: TC-records-close-lists-each-kept-file-once
    case_digest: 65b95089a69b4ea718e82f4f360458c667ef868a63263b747786e5ee5064a8d0
    observed: 2026-11-12T02:17:20Z
    duration_seconds: 2
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: 81d2bf8c3bc2a1705e67113cb95c14ce0a3b4c59
found_by: t38b, test stage, area records
related: []
created: 2026-11-12T01:49:25Z
updated: 2026-11-12T02:18:07Z
---
# session close of a session with an uncommitted screenshot lists it twice in kept and copies its bytes into the archive

**What happened:** kept names the .png twice, and a copy of the uncommitted screenshot is written beside the story, which links it
Actual: Duplicate kept entry; the archive holds bytes HEAD never had until the person commits

**Steps:** session close while an evidence screenshot is not yet committed

**Expected:** kept lists each kept file once, and the archive copies only committed bytes or says it copied uncommitted ones (REQ-records-012, REQ-records-013)

**Why it matters:** Confusing report; uncommitted bytes reach the archive folder

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:49:25Z · Reported (t38b, test stage, area records)
- 2026-11-12T02:04:19Z · status: open -> investigating. Red at 0ee296aa (leif/0.6.0-test1-red: d854ce1a plus the regression tests): TC-records-close-lists-each-kept-file-once failed for this bug's reason; fix in bc8ac154.; evidence: OBS-tc-records-close-lists-each-kept-file-once-e7187a0624d4f8d776813d9e
- 2026-11-12T02:18:07Z · status: investigating -> verified. Green at 0ee26ac0: TC-records-close-lists-each-kept-file-once passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-records-close-lists-each-kept-file-once-3e6ed3a0ac082ef1d70c6499
