---
id: 2026-11-19-g1-gates-and-session-pins-fail-closed
title: "G1: gates and session pins fail closed"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T19:44:14Z
kind: shipped
links:
  - 413
  - .chough/archive/sessions/2026-11-19-g1-gates-and-pins-fail-closed.md
  - .specsync/archive/changes/2026-11-19-fix-g1-gates-and-pins-fail-closed/
record: .chough/archive/sessions/2026-11-19-g1-gates-and-pins-fail-closed.md
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
# G1: gates and session pins fail closed

A hand-edited no_native_change can no longer allow the ungated move; chough change ship checks its whole history from the base, refusing with a working way out when Git cannot read it and going on where a squash merge removed the base; its gate close and move into done act on the pinned session; inbox list honors the pin; session use and new say when a pin still names another session and shelve resolves aliases; chough change refuses a leaking argument before SpecSync runs (#401, #400, #369, #393, #372, part of #367).

Verified at 41d486eb: all eleven verify steps and fifteen cases on macOS; the tester's run on 22 real repositories and its regression set (191, 4 fixed, none newly failing); corvid-agent's review and recheck.

Read more: the pull request 413 · [the story](../../.chough/archive/sessions/2026-11-19-g1-gates-and-pins-fail-closed.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-g1-gates-and-pins-fail-closed/)
