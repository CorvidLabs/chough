---
id: 2026-11-06-an-empty-change-directory-is-not-an-open-change-148
title: An empty change directory is not an open change (#148)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:28:31Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-an-empty-change-directory-left-by-a-branch-switch-is-not-an.md
  - .specsync/archive/changes/2026-11-06-an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148/
record: .chough/archive/sessions/2026-11-06-an-empty-change-directory-left-by-a-branch-switch-is-not-an.md
actor:
  - agent: claude-code
    terminal: t382
---
# An empty change directory is not an open change (#148)

Issue #148: a change directory left empty by a branch switch was treated as an open native change. The fix, as the issue described, skips change directories that hold no file.

After the fix was committed the native check passed, and the case that an empty change directory is not open passed on Linux.

After the maintainer's review the change was archived, and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-an-empty-change-directory-left-by-a-branch-switch-is-not-an.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148/)
