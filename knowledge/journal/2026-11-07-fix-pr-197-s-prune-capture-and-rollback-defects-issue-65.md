---
id: 2026-11-07-fix-pr-197-s-prune-capture-and-rollback-defects-issue-65
title: Fix PR 197's prune capture and rollback defects (issue 65)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T17:31:47Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-issue-65-pr197-p1-corrections.md
  - .specsync/archive/changes/2026-11-07-correct-prune-capture-authority-and-unsafe-rollback-for-pr-197/
record: .chough/archive/sessions/2026-11-07-issue-65-pr197-p1-corrections.md
actor:
  - agent: claude-code
    terminal: t382
---
# Fix PR 197's prune capture and rollback defects (issue 65)

Two P1 defects from issue 65 and PR 197 were corrected without changing retention policy or the old acceptance history: prune capture accepted replacement bytes as deletion authority, and prune rollback aborted on recreated symlink paths. The existing criteria and REQ-history-004 already covered the intent. Reopening the old acceptance was rejected in favour of a new scope and a fresh review.

Both defects were reproduced and their tests failed first; then both cases passed with the four other prune cases, and independent mutants failed as expected. Full qualification passed all 10 steps, with 600 tests on Linux and 596 on macOS. An earlier macOS run hit a checkpoint lock timeout; its cause is unverified. 4412 historical files matched the base.

The maintainer approved the scope and the corrected implementation. The change was archived and the story closed as done on 2026-11-07.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-issue-65-pr197-p1-corrections.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-correct-prune-capture-authority-and-unsafe-rollback-for-pr-197/)
