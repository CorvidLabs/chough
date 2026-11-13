---
id: 2026-11-13-0-6-1-next-with-no-open-session-exits-0-and-timing-says
title: "0.6.1: next with no open session exits 0, and timing says elapsed once"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-13
time: 2026-11-13T03:42:02Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-13-cli-output-fixes.md
  - .specsync/archive/changes/2026-11-13-0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once/
record: .chough/archive/sessions/2026-11-13-cli-output-fixes.md
actor:
  - agent: claude-code
    terminal: t382
---
# 0.6.1: next with no open session exits 0, and timing says elapsed once

With no open session, the normal state after done, chough next says so, names chough session new <key> and exits 0, while status still refuses (REQ-cli-021, REQ-history-006); chough timing says elapsed wall-clock once, in its header (REQ-reporting-001).

Verified: the cases red then green, both bugs verified, and the full suite 734 of 734. Next: Leif's review and merge.

Read more: [the story](../../.chough/archive/sessions/2026-11-13-cli-output-fixes.md) · [the archived change](../../.specsync/archive/changes/2026-11-13-0-6-1-chough-next-with-no-open-session-says-so-and-exits-0-and-timing-says-elapsed-wall-clock-once/)
