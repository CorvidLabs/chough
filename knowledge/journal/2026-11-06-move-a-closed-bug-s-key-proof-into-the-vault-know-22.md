---
id: 2026-11-06-move-a-closed-bug-s-key-proof-into-the-vault-know-22
title: Move a closed bug's key proof into the vault (KNOW-22)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T00:07:40Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under.md
  - .specsync/archive/changes/2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22/
record: .chough/archive/sessions/2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under.md
actor:
  - agent: claude-code
    terminal: t382
---
# Move a closed bug's key proof into the vault (KNOW-22)

When a bug closes, its key proof items now move into the vault under that bug, and the rest is trimmed once it goes cold (KNOW-22). An open bug keeps every linked observation.

The three KNOW-22 cases passed on Linux, one test each: a closed bug moves its key items into the vault, a closed bug releases its loose fixture, and an open bug keeps every linked observation. The two closed-bug tests failed first on main, and the native change passed specsync change check.

The maintainer approved the scope, accepted PR #121 with a passing review and chose to record and merge. The change was archived and the story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-closed-bugs-release-proof-key-items-move-into-the-vault-under-the-bug-and-the-rest-is-trimmed-when-cold-know-22/)
