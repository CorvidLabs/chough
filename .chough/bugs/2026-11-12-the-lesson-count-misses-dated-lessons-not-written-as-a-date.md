---
kind: bug
id: 2026-11-12-the-lesson-count-misses-dated-lessons-not-written-as-a-date
aliases:
  - BUG-the-lesson-count-misses-dated-lessons-not-written-as-a-date-followed-by-bbd611173226150c9f13f54c
title: The lesson count misses dated lessons not written as a date followed by a comma
status: verified
classification: product-defect
scope: in-scope
impact: A resuming agent is told a module has no lessons
criteria: []
requirements: [REQ-cli-043]
cases: []
evidence:
  - OBS-tc-cli-knowledge-lesson-count-dated-e00c49000809f742fb04e98e
  - OBS-tc-cli-knowledge-lesson-count-dated-6e1911d886b1eb8d8d976a6d
receipts:
  - id: 2026-11-12-tc-cli-knowledge-lesson-count-dated
    aliases: OBS-tc-cli-knowledge-lesson-count-dated-e00c49000809f742fb04e98e
    claim: "Ran cargo test --locked --test vault -- --exact the_lesson_count_reads_every_dated_entry for TC-cli-knowledge-lesson-count-dated: failed (exit 101)."
    result: failed
    command: cargo test --locked --test vault -- --exact the_lesson_count_reads_every_dated_entry
    revision: faa164dc71e3c8a0ae749d435c356741994b6854
    environment: local-macos-arm64
    content: 45219ed1e1c5bd4001129a7d7e0643964dcd02390d076f2524c6058386015177
    fingerprint: sha256:fba499e4d52614961a9d6b077d84c623fa0c52c6e628d7b05d6ca0a1a3cd4220
    case: TC-cli-knowledge-lesson-count-dated
    case_digest: b22c652d32461fa354e0154d28086e28b09a16ba5546f71fe862b5cd53867ad4
    observed: 2026-11-12T02:02:24Z
    duration_seconds: 0
  - id: 2026-11-12-tc-cli-knowledge-lesson-count-dated-2662a3
    aliases: OBS-tc-cli-knowledge-lesson-count-dated-6e1911d886b1eb8d8d976a6d
    claim: "Ran cargo test --locked --test vault -- --exact the_lesson_count_reads_every_dated_entry for TC-cli-knowledge-lesson-count-dated: passed (exit 0)."
    result: passed
    command: cargo test --locked --test vault -- --exact the_lesson_count_reads_every_dated_entry
    revision: 0ee26ac0d72f466129525f27afaa5526cd55322e
    environment: local-macos-arm64
    content: cc056522bd209ed839bdb5d0a0b03f05210757d46be1edf71d7246a66228c3a2
    fingerprint: sha256:c0aa3993fe909591b5b66afc6bf7fbdeb870fc333954a3bba19081c3652651d0
    case: TC-cli-knowledge-lesson-count-dated
    case_digest: b22c652d32461fa354e0154d28086e28b09a16ba5546f71fe862b5cd53867ad4
    observed: 2026-11-12T02:17:40Z
    duration_seconds: 1
environment: local-macos-arm64, tester clone at 367dc86b, debug build, SpecSync 6.2.0
revision: d854ce1ac06a195e0c81d2aaf22d61bd64e2c556
found_by: t37f, test stage, area knowledge
related: []
created: 2026-11-12T01:38:48Z
updated: 2026-11-12T02:18:08Z
---
# The lesson count misses dated lessons not written as a date followed by a comma

**What happened:** A module context with '- 2026-11-01 quote fields.' lessons is indexed with 0 lessons
Actual: Only entries whose eleventh byte is a comma are counted

**Steps:** Lessons written with a date then a space, colon or as a heading

**Expected:** The count of recorded lessons (REQ-cli-043); the function's own comment says any entry that starts with a date

**Why it matters:** A resuming agent is told a module has no lessons

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-12T01:38:48Z · Reported (t37f, test stage, area knowledge)
- 2026-11-12T02:02:52Z · status: open -> investigating. Red at faa164dc (TC-cli-knowledge-lesson-count-dated, on t37x's branch before its fix): failed for this bug's reason; fix in 06122a3a.; evidence: OBS-tc-cli-knowledge-lesson-count-dated-e00c49000809f742fb04e98e
- 2026-11-12T02:18:08Z · status: investigating -> verified. Green at 0ee26ac0: TC-cli-knowledge-lesson-count-dated passed after the fix; full suite 626 passed, clippy and fmt clean at 7b645d60.; evidence: OBS-tc-cli-knowledge-lesson-count-dated-6e1911d886b1eb8d8d976a6d
