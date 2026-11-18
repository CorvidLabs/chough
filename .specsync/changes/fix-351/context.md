---
change: fix-351
artifact: context
---

# Context

GitHub #351, a follow-up to #348 (#281). #348 added `chough checkpoint
--no-native-change "<reason>"` for work that needs no SpecSync change. Nothing
told an agent the flag exists: the workflow skill did not mention it, and when a
move was refused as `refused:no-native-change`, the refusal and `chough next`
named only `chough checkpoint --native-change <id>` and `specsync change new`.

Leif's decision, in #351 on 2026-11-18: "yes, both": the skill explains the flag,
and when a gated move is refused for lacking a native change, `chough next`
offers both paths.

The host templates under `integrations/` do not cover stages or the native
change, so they are not changed.
