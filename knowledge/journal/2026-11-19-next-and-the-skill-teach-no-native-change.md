---
id: 2026-11-19-next-and-the-skill-teach-no-native-change
title: next and the skill teach --no-native-change
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T01:30:37Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-351.md
  - .specsync/archive/changes/2026-11-19-fix-351/
record: .chough/archive/sessions/2026-11-18-fix-351.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# next and the skill teach --no-native-change

Nothing told an agent that chough checkpoint --no-native-change exists (#351). A move refused for lacking a native change, and chough next, now name both ways on; once the flag is set next says the session needs no native change, with the reason; the literal <reason> placeholder is refused. The workflow skill explains the flag, and a project installed by 0.6.1 gets the new text on chough skill update. Tests failed first; the verify lane passed on macOS after main was merged in, and the tester passed the recheck. Linux was not run locally.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-351.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-351/)
