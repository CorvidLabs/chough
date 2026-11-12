---
id: 2026-11-08-document-what-chough-automates-and-what-the-human-decides
title: Document what Chough automates and what the human decides
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-08
time: 2026-11-08T02:05:46Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-explain-how-chough-guides-the-specsync-lifecycle-and-stages-in.md
  - .specsync/archive/changes/2026-11-08-explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18/
record: .chough/archive/sessions/2026-11-07-explain-how-chough-guides-the-specsync-lifecycle-and-stages-in.md
actor:
  - agent: claude-code
    terminal: t382
---
# Document what Chough automates and what the human decides

The README, walkthrough and lifecycle docs now explain how Chough works: what the agent runs automatically, what each helper command does, and what the human decides. It reused criteria FLOW-18, FLOW-13 and LIFE-3, since this clarifies existing behavior rather than adding any.

The scope was docs only, with no changes to helpers, runtime, stages, retention or SpecSync. Describing the archive-stage proposal as current behavior was rejected; it stays a separate, unapproved proposal.

The maintainer approved the scope and accepted the implementation in PR #198, both relayed. The native change was archived and the story closed done on 2026-11-08.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-explain-how-chough-guides-the-specsync-lifecycle-and-stages-in.md) · [the archived change](../../.specsync/archive/changes/2026-11-08-explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18/)
