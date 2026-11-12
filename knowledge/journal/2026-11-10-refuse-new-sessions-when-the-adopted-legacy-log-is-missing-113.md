---
id: 2026-11-10-refuse-new-sessions-when-the-adopted-legacy-log-is-missing-113
title: Refuse new sessions when the adopted legacy log is missing (#113)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-10
time: 2026-11-10T16:06:34Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-09-chough-113-missing-log.md
  - .specsync/archive/changes/2026-11-10-refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113/
record: .chough/archive/sessions/2026-11-09-chough-113-missing-log.md
actor:
  - agent: claude-code
    terminal: t382
---
# Refuse new sessions when the adopted legacy log is missing (#113)

Before this fix, a new session still went ahead when a frozen legacy log the history had adopted was missing. Chough now refuses new-session writes in that case, while a history that never had a legacy log stays writable (#113).

The failing tests came first: the missing-log case failed and the never-had-a-log case passed. With the fix, all 39 sessions tests and all 25 history tests passed, with fmt and lint clean; the three cases were recorded from a review of those retained run logs, not a new run.

On 2026-11-10 the maintainer's relayed rule closed the acceptance gate: bug fixes that make behaviour match existing specs may run the full SpecSync lifecycle and squash-merge once CI is green and every review thread is resolved. The change was archived and the story closed done that day.

Read more: [the story](../../.chough/archive/sessions/2026-11-09-chough-113-missing-log.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113/)
