# Roadmap

Durable intent is in [HI](../hi/). This roadmap records delivered work and proposed
sequencing; it is not human acceptance or proof of delivery.

| Stage | Outcome | State |
|---|---|---|
| First foundation | Skill, focused context, native status, bound observations | Delivered at d38aa19 (bootstrap, no native lifecycle) |
| Resumable stage workflow | Stages and gates, checkpoint schema 2, focused cases, local bugs, optional validation, release routing and librarian, per-host skills | Implemented in `evolve-chough-into-a-resumable-stage-workflow`; awaiting human implementation review |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Next; the shipping and checkout samples are rehearsals |
| More hosts | Exercise Cursor, Gemini, Grok and Kimi discovery and cold starts | Proposed; only Claude Code and Codex were exercised |
| Observation reuse | Recognize unchanged inputs at a descendant commit | Unapproved proposal ([NEXT_CHANGE.md](NEXT_CHANGE.md)); needs its own human decision |
| Native gate adapter | Reuse Trust where its policy fits | Proposed; needs degraded, failed, unavailable and baseline cases |
| Checkpoint adapter | APS, if it gains Windows builds, file input, compare-and-set and closed schemas | Deferred (D-010) |
| Test runner adapter | Magpie for real-command targets with explicit assertions | Deferred until installed and qualified per mode |
| Rune proof of concept | Optional cross-agent session bridge | Deferred; current-agent-first remains the model |
| Monitoring sources | Project-native health and log adapters as cases | Proposed per project; no general health service assumed |

More commands should follow friction observed in real use.
