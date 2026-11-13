---
id: 2026-11-13-0-6-1-a-short-host-name-in-an-ordinary-word-is-not-a-leak
title: "0.6.1: a short host name in an ordinary word is not a leak"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T02:27:14Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-host-name-leak.md
  - .specsync/archive/changes/2026-11-13-0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak/
record: .chough/archive/sessions/2026-11-13-host-name-leak.md
actor:
  - agent: claude-code
    terminal: t382
---
# 0.6.1: a short host name in an ordinary word is not a leak

The leak check no longer reads an ordinary word equal to a short all-letter host name as a host-name leak; such a name is a leak only inside a path or a URL or as <user>@<host>, and a longer or hyphenated host name stays a leak anywhere (REQ-records-006).

Verified: the case red then green, the bug verified, and the full suite 730 of 730. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-host-name-leak.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak/)
