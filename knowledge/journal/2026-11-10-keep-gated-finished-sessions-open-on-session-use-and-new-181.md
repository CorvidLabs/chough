---
id: 2026-11-10-keep-gated-finished-sessions-open-on-session-use-and-new-181
title: Keep gated finished sessions open on session use and new (#181)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-10
time: 2026-11-10T23:06:59Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-10-keep-finished-sessions-with-open-human-gates-open-on-session.md
  - .specsync/archive/changes/2026-11-10-keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181/
record: .chough/archive/sessions/2026-11-10-keep-finished-sessions-with-open-human-gates-open-on-session.md
actor:
  - agent: claude-code
    terminal: t382
---
# Keep gated finished sessions open on session use and new (#181)

Issue #181: session use and session new archived finished sessions that still held an open human gate. Such sessions now stay open.

The case for it, that a finished session with an open gate stays open on a switch, failed first and then passed.

It ran under the maintainer's merge rule for bug fixes that bring behaviour in line with existing specs: the full lifecycle, with approval and review recorded under that rule, and a squash-merge once CI is green and every review thread is resolved. The change was archived and the story closed as done on 2026-11-10.

Read more: [the story](../../.chough/archive/sessions/2026-11-10-keep-finished-sessions-with-open-human-gates-open-on-session.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181/)
