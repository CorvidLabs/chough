---
id: 2026-11-07-prototype-template-keeps-hidden-state-panels-hidden-58
title: Prototype template keeps hidden state panels hidden (#58)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-07
time: 2026-11-07T04:50:06Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template.md
  - .specsync/archive/changes/2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58/
record: .chough/archive/sessions/2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template.md
actor:
  - agent: claude-code
    terminal: t382
---
# Prototype template keeps hidden state panels hidden (#58)

When a project added its own display rules, the prototype template showed every state panel at once (#58). The fix adds a hidden-panel guard to both copies of the template stylesheet and records the stylesheet shipped in 0.1.0 to 0.4.0 as known-outdated.

Both new tests failed first and then passed, and the full cargo test run passed 575 tests.

The maintainer approved the scope and passed the implementation review of PR #193, and the story closed as done on 7 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template.md) · [the archived change](../../.specsync/archive/changes/2026-11-07-keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58/)
