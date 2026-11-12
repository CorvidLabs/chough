---
id: 2026-11-06-workflow-skill-names-stage-advance-to-in-its-transition-table
title: Workflow skill names stage advance --to in its transition table
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:10:32Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-name-stage-advance-to-in-the-workflow-skill-s-transition-table.md
  - .specsync/archive/changes/2026-11-06-name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up/
record: .chough/archive/sessions/2026-11-06-name-stage-advance-to-in-the-workflow-skill-s-transition-table.md
actor:
  - agent: claude-code
    terminal: t382
---
# Workflow skill names stage advance --to in its transition table

After #147 merged, the chough-workflow skill's transition table was updated to name stage advance --to, as its own small follow-up change under DRIVE-9 and REQ-cli-052.

The new case first failed because the previous copy of the stages reference was still treated as current rather than known to be outdated; after the change it passed.

Scope was approved under the maintainer's overnight delegation, and the maintainer passed the implementation review of PR #151, with the merge planned right after #149. The story closed as done on 6 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-name-stage-advance-to-in-the-workflow-skill-s-transition-table.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up/)
