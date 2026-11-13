---
id: 2026-11-07-evidence-prune-fails-closed-and-restores-partial-removals-65
title: Evidence prune fails closed and restores partial removals (#65)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T15:51:32Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-issue-65-evidence-prune-race.md
  - .specsync/archive/changes/2026-11-07-make-evidence-prune-apply-recheck-protections-and-restore-partial-removals/
record: .chough/archive/sessions/2026-11-07-issue-65-evidence-prune-race.md
actor:
  - agent: claude-code
    terminal: t382
---
# Evidence prune fails closed and restores partial removals (#65)

Evidence prune --apply deleted files directly with no rollback, so a protection change mid-run or a partial failure could leave evidence deleted. The fix rechecks protections, removes bytes only after checking them, and restores partial removals and earlier sets without overwriting a recreated file, reusing the library archive's verified removal under REQ-history-004. Retention rules, protections and output schemas were left as they were.

All four regressions failed first and then passed. The final frozen runs passed all 10 verify steps: 593 product tests on macOS and 597 on Linux, with strict spec checks at 100% coverage. Earlier lane failures from stale native verification were kept on record.

The maintainer approved the scope, a reapproval adding one test file, and the implementation review. The story closed as done on 7 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-issue-65-evidence-prune-race.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-make-evidence-prune-apply-recheck-protections-and-restore-partial-removals/)
