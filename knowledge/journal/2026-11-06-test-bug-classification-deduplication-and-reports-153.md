---
id: 2026-11-06-test-bug-classification-deduplication-and-reports-153
title: Test bug classification, deduplication and reports (#153)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:16:54Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-bug-classification-deduplication-and-report-mappings-and-their.md
  - .specsync/archive/changes/2026-11-06-bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153/
record: .chough/archive/sessions/2026-11-06-bug-classification-deduplication-and-report-mappings-and-their.md
actor:
  - agent: claude-code
    terminal: t382
---
# Test bug classification, deduplication and reports (#153)

Issue #153 covered the audit's Bugs row: bug classification, deduplication and report mappings, and their recovery branches. The tests exposed two defects: a mistaken duplicate could not be reopened, and a bug could link to itself or to an unknown bug. The maintainer chose to fix both defects and the missing recovery hints in this change, with a small fix to the bugs code, rather than test the mappings alone and file the defects apart.

Six cases were defined. The two defect cases failed first; after the fix all six passed on macOS, the bug was verified as reproduced and then passing, and a mutation check caught 5 of 5 mapping mutations.

The maintainer approved the implementation review, the change was archived, and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-bug-classification-deduplication-and-report-mappings-and-their.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153/)
