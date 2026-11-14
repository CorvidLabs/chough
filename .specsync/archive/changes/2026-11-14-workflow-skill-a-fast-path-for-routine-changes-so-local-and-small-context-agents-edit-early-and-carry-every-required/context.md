---
change: workflow-skill-a-fast-path-for-routine-changes-so-local-and-small-context-agents-edit-early-and-carry-every-required
artifact: context
---

# Context

On 2026-11-13/14 Leif ran a bake-off on #276 with three local models (gpt-oss-120b, Flint 40B, Ripple 65B) through Pi in Herdr. Every model spent 30 to 300 tool calls reading AGENTS.md, docs and specs before its first edit; gpt-oss never edited. The two that finished took 153 and 509 minutes, neither opened a Chough session, and Ripple's native change carried no delta, so `specsync change approve` refused with "semantic delta modules must exactly match affected specs". Leif asked to fix this "for all local agents" in chat on 2026-11-14 and, shown this fast path as a chough PR, answered "ok yes please do all of this".

LIFE-22 already holds the intent: any agent that can read Markdown and run commands, local agents included, should work.

Constraints: SKILL.md must stay under 6144 bytes (package test), so the fast path lives in the stage contract every agent already reads, with one pointer in SKILL.md. Changed bundle bytes need their previous digests in `KNOWN_PREVIOUS` and the release corpus manifest's current digests and manifest digest updated.
