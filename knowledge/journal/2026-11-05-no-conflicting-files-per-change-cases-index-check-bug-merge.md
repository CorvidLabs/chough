---
id: 2026-11-05-no-conflicting-files-per-change-cases-index-check-bug-merge
title: "No conflicting files: per-change cases, index check, bug merge driver"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-05
time: 2026-11-05T15:46:13Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-05-no-conflicting-files-per-change-case-files-a-knowledge-rebuild.md
  - .specsync/archive/changes/2026-11-05-no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12/
record: .chough/archive/sessions/2026-11-05-no-conflicting-files-per-change-case-files-a-knowledge-rebuild.md
actor:
  - agent: claude-code
    terminal: t382
---
# No conflicting files: per-change cases, index check, bug merge driver

To keep parallel work from producing conflicting files (DRIVE-12), test cases now live in per-change case files, a check names knowledge index files that are stale until a rebuild, and a merge driver joins two edits of one bug.

Three cases passed on Linux, one test run each: per-change case files load together with unique ids, the index check names stale files and passes after a rebuild, and two edits of one bug merge while a real disagreement stays a conflict.

The maintainer accepted it on PR #112 together with a lessons change on 2026-11-05, answering Yes, record and merge. The native review was recorded in the maintainer's name and the story closed done that day.

Read more: [the story](../../.chough/archive/sessions/2026-11-05-no-conflicting-files-per-change-case-files-a-knowledge-rebuild.md) · [the archived change](../../.specsync/archive/changes/2026-11-05-no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12/)
