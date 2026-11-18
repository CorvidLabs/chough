---
id: 2026-11-17-skill-release-pin-matches-the-edited-fast-path
title: Skill release pin matches the edited fast path
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-17
time: 2026-11-17T14:31:55Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-17-fix-skill-pin.md
  - .specsync/archive/changes/2026-11-17-pin-the-edited-workflow-fast-path-in-the-skill-release-corpus/
record: .chough/archive/sessions/2026-11-17-fix-skill-pin.md
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
# Skill release pin matches the edited fast path

The current bundle pin for references/stages.md now matches the edited workflow fast path. cargo test --locked --test skill_releases passed, 7 tests. This unblocks CI on the docs pull request.

Read more: [the story](../../.chough/archive/sessions/2026-11-17-fix-skill-pin.md) · [the archived change](../../.specsync/archive/changes/2026-11-17-pin-the-edited-workflow-fast-path-in-the-skill-release-corpus/)
