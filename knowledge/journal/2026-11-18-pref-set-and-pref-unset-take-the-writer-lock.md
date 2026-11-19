---
id: 2026-11-18-pref-set-and-pref-unset-take-the-writer-lock
title: pref set and pref unset take the writer lock
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T19:44:06Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-266-prefs.md
  - .specsync/archive/changes/2026-11-18-fix-266-prefs/
record: .chough/archive/sessions/2026-11-18-fix-266-prefs.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# pref set and pref unset take the writer lock

pref set and pref unset take the writer lock, so they refuse while a .chough path is unmerged, as REQ-cli-069 promises (#330 review P2)

Why: 0xLeif decided "Scope approval of fix-266-prefs": "Fix first, you resolve" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18, his answer to his own P2 review thread on #330 ('Cover preference writers before relying on this lock as the merge guard … regress pref set/unset'): a small follow-up native change on #330's branch in which pref set and pref unset take the writer lock, with a regression test red first); recorded by chough-fixer-t4g; a relayed claim, not authenticated; ran specsync change approve fix-266-prefs as 0xLeif

Verified: cargo test --locked --test git_merge -- preference_writes_refuse_while_chough_is_unmerged: 1 passed.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-266-prefs.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-266-prefs/)
