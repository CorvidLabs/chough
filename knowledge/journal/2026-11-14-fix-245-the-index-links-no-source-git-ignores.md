---
id: 2026-11-14-fix-245-the-index-links-no-source-git-ignores
title: "Fix #245: the index links no source Git ignores"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-14
time: 2026-11-14T17:33:02Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-fix-245-index-ignored.md
  - .specsync/archive/changes/2026-11-14-fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes/
record: .chough/archive/sessions/2026-11-13-fix-245-index-ignored.md
actor:
  - agent: claude-code
---
# Fix #245: the index links no source Git ignores

chough library index links only the rule, skill, decision and policy files that exist and Git does not ignore, so an index committed in a working tree with a gitignored skill copy (.grok/skills) passes library index --check in a fresh clone and in CI (REQ-cli-043).

Verified: the case red then green, the bug verified, and the full suite 744 of 744. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-fix-245-index-ignored.md) · [the archived change](../../.specsync/archive/changes/2026-11-14-fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes/)
