---
id: 2026-11-18-a-session-that-needs-no-native-change-can-reach-done
title: A session that needs no native change can reach done
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T21:07:57Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-281.md
  - .specsync/archive/changes/2026-11-18-fix-281/
record: .chough/archive/sessions/2026-11-18-fix-281.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# A session that needs no native change can reach done

A finished session with no native change could only be archived as abandoned (#281). chough checkpoint --no-native-change <reason> now records that the work needs none, and why; its gated moves are then allowed as ungated:no-native-change-required, so it reaches done and is archived with outcome done. An open human gate still refuses, and the reason passes the leak guard. A new test failed on main first; the verify lane passed on macOS after main was merged in, and tester2 passed the recheck. Linux was not run locally. #351 covers the skill and chough next.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-281.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-281/)
