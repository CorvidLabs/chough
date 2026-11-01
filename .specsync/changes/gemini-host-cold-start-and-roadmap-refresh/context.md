---
change: gemini-host-cold-start-and-roadmap-refresh
artifact: context
---

# Context

## Origin

The Chough roadmap (docs/ROADMAP.md) includes the milestone "More hosts: Exercise Cursor, Gemini, Grok and Kimi discovery and cold starts. Proposed; only Claude Code and Codex were exercised." Additionally, docs/exercises/cold-start.md notes that only a Codex session had been recorded and that Gemini had not been exercised.

This change records the genuine cold-start exercise for the Gemini host running in Google Antigravity on macOS arm64, documents the observed discovery path and friction, refreshes the roadmap to reflect merged PRs #8, #9, #15, #16 and #17 along with this Gemini exercise, and updates skill guidance so that Gemini and Antigravity agents can discover the skill and resume cold when stage is done.

## Constraints

- Current-agent-first v1: keep the small Rust helper small without adding a model client, provider runtime, or external state.
- No em or en dashes in any added text, in the skill bundle or documentation.
- Never record or simulate human decisions. Human scope approval and implementation review remain strictly with 0xLeif.
- Keep the canonical skill bundle in skills/chough-workflow and its copies in .agents/skills/chough-workflow and .claude/skills/chough-workflow byte-identical.
- Maintain tool versions matching CI: hi 0.8.0, fledge 1.7.2, and specsync 6.1.1.
