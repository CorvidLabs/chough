---
id: 2026-11-18-a-blocking-bug-stops-only-its-session
title: A blocking bug stops only its session
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T00:36:40Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-blocking-bug-session.md
  - .specsync/archive/changes/2026-11-18-a-blocking-bug-stops-only-a-session-it-is-related-to/
record: .chough/archive/sessions/2026-11-18-blocking-bug-session.md
actor:
  - agent: grok-t1w
    model: grok
    terminal: t1w
---
# A blocking bug stops only its session

A blocking bug stops a session only when that session lists it, its evidence names the session, or they share a criterion, requirement, or case. Other sessions are warned by name. A bug that names nothing stops no session.

cargo test --locked for bugs, advance_to, and skill_releases passed. TC-cli-bug-only-open-blocking-blocks passed on local-macos-arm64. The full verify lane was not run.

Implementation review is the remaining human gate. Shipping archives the change so CI can pass the archive gate.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-blocking-bug-session.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-a-blocking-bug-stops-only-a-session-it-is-related-to/)
